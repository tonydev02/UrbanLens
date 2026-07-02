ALTER TABLE areas
    ADD COLUMN administrative_code text,
    ADD COLUMN name_ja text,
    ADD COLUMN name_en text,
    ADD COLUMN source_id uuid REFERENCES data_sources (id),
    ADD COLUMN current_boundary_id uuid;

UPDATE areas
SET
    administrative_code = source_code,
    name_ja = name
WHERE administrative_code IS NULL
    OR name_ja IS NULL;

ALTER TABLE areas
    ALTER COLUMN administrative_code SET NOT NULL,
    ALTER COLUMN name_ja SET NOT NULL,
    ADD CONSTRAINT areas_administrative_code_not_blank CHECK (
        btrim(administrative_code) <> ''
    ),
    ADD CONSTRAINT areas_name_ja_not_blank CHECK (btrim(name_ja) <> ''),
    ADD CONSTRAINT areas_area_type_known CHECK (area_type IN ('ward')),
    ADD CONSTRAINT areas_source_code_administrative_code_consistent CHECK (
        source_code = administrative_code
    );

UPDATE areas
SET source_id = datasets.source_id
FROM datasets
WHERE areas.dataset_id = datasets.id
    AND areas.source_id IS NULL;

ALTER TABLE areas
    ALTER COLUMN source_id SET NOT NULL;

ALTER TABLE datasets
    ADD CONSTRAINT datasets_source_id_id_unique UNIQUE (source_id, id);

CREATE UNIQUE INDEX areas_type_administrative_code_unique
    ON areas (area_type, administrative_code);

CREATE INDEX areas_source_id_idx ON areas (source_id);
CREATE INDEX areas_administrative_code_idx ON areas (administrative_code);
CREATE INDEX areas_name_ja_idx ON areas (name_ja);

CREATE TABLE area_boundaries (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    area_id uuid NOT NULL REFERENCES areas (id) ON DELETE CASCADE,
    source_id uuid NOT NULL REFERENCES data_sources (id),
    dataset_id uuid NOT NULL REFERENCES datasets (id),
    import_run_id uuid,
    raw_record_id uuid,
    administrative_code text NOT NULL,
    name_ja text NOT NULL,
    name_en text,
    source_record_hash text NOT NULL,
    source_feature_id text,
    source_feature_position bigint,
    boundary_version text NOT NULL,
    location_precision text NOT NULL DEFAULT 'ward_polygon',
    geometry geometry(MultiPolygon, 4326) NOT NULL,
    is_current boolean NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT area_boundaries_source_dataset_fk FOREIGN KEY (
        source_id,
        dataset_id
    ) REFERENCES datasets (source_id, id),
    CONSTRAINT area_boundaries_raw_record_lineage_fk FOREIGN KEY (
        raw_record_id,
        import_run_id,
        dataset_id
    ) REFERENCES raw_records (id, import_run_id, dataset_id),
    CONSTRAINT area_boundaries_administrative_code_not_blank CHECK (
        btrim(administrative_code) <> ''
    ),
    CONSTRAINT area_boundaries_name_ja_not_blank CHECK (btrim(name_ja) <> ''),
    CONSTRAINT area_boundaries_source_record_hash_hex CHECK (
        source_record_hash ~ '^[0-9a-f]{64}$'
    ),
    CONSTRAINT area_boundaries_source_feature_position_positive CHECK (
        source_feature_position IS NULL
        OR source_feature_position > 0
    ),
    CONSTRAINT area_boundaries_boundary_version_not_blank CHECK (
        btrim(boundary_version) <> ''
    ),
    CONSTRAINT area_boundaries_location_precision_known CHECK (
        location_precision = 'ward_polygon'
    ),
    CONSTRAINT area_boundaries_geometry_valid CHECK (ST_IsValid(geometry)),
    CONSTRAINT area_boundaries_geometry_multipolygon CHECK (
        GeometryType(geometry) = 'MULTIPOLYGON'
    ),
    CONSTRAINT area_boundaries_updated_not_before_created CHECK (
        updated_at >= created_at
    ),
    CONSTRAINT area_boundaries_raw_lineage_all_or_none CHECK (
        (
            raw_record_id IS NULL
            AND import_run_id IS NULL
        )
        OR (
            raw_record_id IS NOT NULL
            AND import_run_id IS NOT NULL
        )
    )
);

CREATE UNIQUE INDEX area_boundaries_area_boundary_version_unique
    ON area_boundaries (area_id, boundary_version);
CREATE UNIQUE INDEX area_boundaries_dataset_administrative_hash_unique
    ON area_boundaries (dataset_id, administrative_code, source_record_hash);
CREATE UNIQUE INDEX area_boundaries_one_current_per_area_unique
    ON area_boundaries (area_id)
    WHERE is_current;

CREATE INDEX area_boundaries_area_id_idx ON area_boundaries (area_id);
CREATE INDEX area_boundaries_source_id_idx ON area_boundaries (source_id);
CREATE INDEX area_boundaries_dataset_id_idx ON area_boundaries (dataset_id);
CREATE INDEX area_boundaries_import_run_id_idx
    ON area_boundaries (import_run_id)
    WHERE import_run_id IS NOT NULL;
CREATE INDEX area_boundaries_raw_record_id_idx
    ON area_boundaries (raw_record_id)
    WHERE raw_record_id IS NOT NULL;
CREATE INDEX area_boundaries_administrative_code_idx
    ON area_boundaries (administrative_code);
CREATE INDEX area_boundaries_geometry_gix
    ON area_boundaries USING gist (geometry);

ALTER TABLE areas
    ADD CONSTRAINT areas_current_boundary_fk FOREIGN KEY (current_boundary_id)
        REFERENCES area_boundaries (id);

CREATE INDEX areas_current_boundary_id_idx
    ON areas (current_boundary_id)
    WHERE current_boundary_id IS NOT NULL;

CREATE INDEX transaction_observations_period_idx
    ON transaction_observations (transaction_year, transaction_quarter);
CREATE INDEX transaction_observations_trade_price_idx
    ON transaction_observations (trade_price_jpy)
    WHERE trade_price_jpy IS NOT NULL;
CREATE INDEX transaction_observations_area_m2_idx
    ON transaction_observations (area_m2)
    WHERE area_m2 IS NOT NULL;
CREATE INDEX transaction_observations_station_walk_minutes_idx
    ON transaction_observations (station_walk_minutes)
    WHERE station_walk_minutes IS NOT NULL;
CREATE INDEX transaction_observations_ward_asset_period_idx
    ON transaction_observations (
        municipality_code,
        asset_type,
        transaction_year,
        transaction_quarter
    );
