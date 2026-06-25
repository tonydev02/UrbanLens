CREATE TABLE data_sources (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    publisher text NOT NULL,
    source_url text NOT NULL,
    license_url text NOT NULL,
    metadata_verified_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT data_sources_name_not_blank CHECK (btrim(name) <> ''),
    CONSTRAINT data_sources_publisher_not_blank CHECK (btrim(publisher) <> ''),
    CONSTRAINT data_sources_source_url_not_blank CHECK (btrim(source_url) <> ''),
    CONSTRAINT data_sources_license_url_not_blank CHECK (btrim(license_url) <> ''),
    CONSTRAINT data_sources_updated_not_before_created CHECK (updated_at >= created_at)
);

CREATE TABLE datasets (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id uuid NOT NULL REFERENCES data_sources (id),
    source_dataset_name text NOT NULL,
    retrieval_method text NOT NULL,
    retrieval_query jsonb NOT NULL DEFAULT '{}'::jsonb,
    source_version text,
    retrieved_at timestamptz NOT NULL,
    artifact_sha256 text NOT NULL,
    format text NOT NULL,
    record_count bigint NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT datasets_source_dataset_name_not_blank CHECK (btrim(source_dataset_name) <> ''),
    CONSTRAINT datasets_retrieval_method_not_blank CHECK (btrim(retrieval_method) <> ''),
    CONSTRAINT datasets_artifact_sha256_hex CHECK (artifact_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT datasets_format_not_blank CHECK (btrim(format) <> ''),
    CONSTRAINT datasets_record_count_non_negative CHECK (record_count >= 0),
    CONSTRAINT datasets_updated_not_before_created CHECK (updated_at >= created_at)
);

CREATE INDEX datasets_source_id_idx ON datasets (source_id);

CREATE TABLE import_runs (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    dataset_id uuid NOT NULL REFERENCES datasets (id),
    started_at timestamptz NOT NULL DEFAULT now(),
    completed_at timestamptz,
    status text NOT NULL DEFAULT 'pending',
    normalization_version text NOT NULL,
    records_received bigint NOT NULL DEFAULT 0,
    records_imported bigint NOT NULL DEFAULT 0,
    records_updated bigint NOT NULL DEFAULT 0,
    duplicates_skipped bigint NOT NULL DEFAULT 0,
    records_rejected bigint NOT NULL DEFAULT 0,
    warning_records bigint NOT NULL DEFAULT 0,
    error_kind text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT import_runs_id_dataset_unique UNIQUE (id, dataset_id),
    CONSTRAINT import_runs_status_known CHECK (
        status IN (
            'pending',
            'running',
            'completed',
            'completed_with_warnings',
            'failed'
        )
    ),
    CONSTRAINT import_runs_terminal_status_completed_at CHECK (
        status NOT IN ('completed', 'completed_with_warnings', 'failed')
        OR completed_at IS NOT NULL
    ),
    CONSTRAINT import_runs_normalization_version_not_blank CHECK (
        btrim(normalization_version) <> ''
    ),
    CONSTRAINT import_runs_counts_non_negative CHECK (
        records_received >= 0
        AND records_imported >= 0
        AND records_updated >= 0
        AND duplicates_skipped >= 0
        AND records_rejected >= 0
        AND warning_records >= 0
    ),
    CONSTRAINT import_runs_updated_not_before_created CHECK (updated_at >= created_at)
);

CREATE INDEX import_runs_dataset_id_idx ON import_runs (dataset_id);
CREATE INDEX import_runs_status_idx ON import_runs (status);

CREATE TABLE raw_records (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    dataset_id uuid NOT NULL REFERENCES datasets (id),
    import_run_id uuid NOT NULL,
    source_position bigint NOT NULL,
    external_id text,
    payload_json jsonb NOT NULL,
    payload_sha256 text NOT NULL,
    validation_status text NOT NULL DEFAULT 'pending',
    validation_errors jsonb NOT NULL DEFAULT '[]'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT raw_records_import_run_dataset_fk FOREIGN KEY (import_run_id, dataset_id)
        REFERENCES import_runs (id, dataset_id),
    CONSTRAINT raw_records_id_import_run_unique UNIQUE (id, import_run_id),
    CONSTRAINT raw_records_dataset_position_unique UNIQUE (dataset_id, source_position),
    CONSTRAINT raw_records_source_position_positive CHECK (source_position > 0),
    CONSTRAINT raw_records_payload_sha256_hex CHECK (payload_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT raw_records_validation_status_known CHECK (
        validation_status IN ('pending', 'valid', 'valid_with_warnings', 'rejected')
    ),
    CONSTRAINT raw_records_validation_errors_array CHECK (
        jsonb_typeof(validation_errors) = 'array'
    )
);

CREATE INDEX raw_records_dataset_id_idx ON raw_records (dataset_id);
CREATE INDEX raw_records_import_run_id_idx ON raw_records (import_run_id);
CREATE INDEX raw_records_payload_sha256_idx ON raw_records (payload_sha256);

CREATE TABLE validation_issues (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    import_run_id uuid NOT NULL REFERENCES import_runs (id),
    raw_record_id uuid,
    issue_code text NOT NULL,
    severity text NOT NULL,
    field_name text,
    raw_value_summary text,
    message text NOT NULL,
    disposition text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT validation_issues_raw_record_import_run_fk FOREIGN KEY (
        raw_record_id,
        import_run_id
    ) REFERENCES raw_records (id, import_run_id),
    CONSTRAINT validation_issues_issue_code_not_blank CHECK (btrim(issue_code) <> ''),
    CONSTRAINT validation_issues_severity_known CHECK (severity IN ('warning', 'rejection')),
    CONSTRAINT validation_issues_message_not_blank CHECK (btrim(message) <> ''),
    CONSTRAINT validation_issues_disposition_not_blank CHECK (btrim(disposition) <> '')
);

CREATE INDEX validation_issues_import_run_id_idx ON validation_issues (import_run_id);
CREATE INDEX validation_issues_raw_record_id_idx
    ON validation_issues (raw_record_id)
    WHERE raw_record_id IS NOT NULL;
CREATE INDEX validation_issues_severity_idx ON validation_issues (severity);

CREATE TABLE areas (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    dataset_id uuid NOT NULL REFERENCES datasets (id),
    source_code text NOT NULL,
    name text NOT NULL,
    area_type text NOT NULL,
    geometry geometry(MultiPolygon, 4326),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT areas_dataset_type_code_unique UNIQUE (dataset_id, area_type, source_code),
    CONSTRAINT areas_source_code_not_blank CHECK (btrim(source_code) <> ''),
    CONSTRAINT areas_name_not_blank CHECK (btrim(name) <> ''),
    CONSTRAINT areas_area_type_not_blank CHECK (btrim(area_type) <> ''),
    CONSTRAINT areas_updated_not_before_created CHECK (updated_at >= created_at)
);

CREATE INDEX areas_dataset_id_idx ON areas (dataset_id);
CREATE INDEX areas_area_type_idx ON areas (area_type);
CREATE INDEX areas_geometry_gix ON areas USING gist (geometry) WHERE geometry IS NOT NULL;
