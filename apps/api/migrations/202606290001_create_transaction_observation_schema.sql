ALTER TABLE raw_records
    ADD CONSTRAINT raw_records_id_import_run_dataset_unique UNIQUE (
        id,
        import_run_id,
        dataset_id
    );

CREATE TABLE transaction_observations (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    raw_record_id uuid NOT NULL,
    import_run_id uuid NOT NULL,
    dataset_id uuid NOT NULL REFERENCES datasets (id),
    source_record_hash text NOT NULL,
    normalization_version text NOT NULL,
    validation_status text NOT NULL,
    asset_type text NOT NULL,
    raw_asset_type text NOT NULL,
    price_category text NOT NULL,
    transaction_year smallint NOT NULL,
    transaction_quarter smallint NOT NULL,
    trade_price_jpy bigint,
    source_unit_price_jpy_per_m2 bigint,
    area_m2 numeric(14, 2),
    total_floor_area_m2 numeric(14, 2),
    total_floor_area_is_lower_bound boolean NOT NULL DEFAULT false,
    municipality_code text NOT NULL,
    prefecture_name text NOT NULL,
    municipality_name text NOT NULL,
    district_name text,
    nearest_station_name text,
    station_walk_minutes integer,
    floor_plan text,
    structure text,
    source_use text,
    intended_future_use text,
    city_planning text,
    renovation text,
    transaction_circumstances text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT transaction_observations_raw_record_import_run_fk FOREIGN KEY (
        raw_record_id,
        import_run_id,
        dataset_id
    ) REFERENCES raw_records (id, import_run_id, dataset_id),
    CONSTRAINT transaction_observations_id_import_run_unique UNIQUE (id, import_run_id),
    CONSTRAINT transaction_observations_raw_record_unique UNIQUE (raw_record_id),
    CONSTRAINT transaction_observations_source_record_hash_hex CHECK (
        source_record_hash ~ '^[0-9a-f]{64}$'
    ),
    CONSTRAINT transaction_observations_normalization_version_not_blank CHECK (
        btrim(normalization_version) <> ''
    ),
    CONSTRAINT transaction_observations_validation_status_known CHECK (
        validation_status IN ('valid', 'valid_with_warnings')
    ),
    CONSTRAINT transaction_observations_asset_type_known CHECK (
        asset_type IN (
            'land',
            'land_and_building',
            'used_condominium',
            'unknown'
        )
    ),
    CONSTRAINT transaction_observations_raw_asset_type_not_blank CHECK (
        btrim(raw_asset_type) <> ''
    ),
    CONSTRAINT transaction_observations_price_category_known CHECK (
        price_category IN ('transaction_price_information')
    ),
    CONSTRAINT transaction_observations_quarter_valid CHECK (
        transaction_year >= 2005
        AND transaction_quarter BETWEEN 1 AND 4
    ),
    CONSTRAINT transaction_observations_trade_price_positive CHECK (
        trade_price_jpy IS NULL
        OR trade_price_jpy > 0
    ),
    CONSTRAINT transaction_observations_source_unit_price_positive CHECK (
        source_unit_price_jpy_per_m2 IS NULL
        OR source_unit_price_jpy_per_m2 > 0
    ),
    CONSTRAINT transaction_observations_area_non_negative CHECK (
        area_m2 IS NULL
        OR area_m2 >= 0
    ),
    CONSTRAINT transaction_observations_total_floor_area_non_negative CHECK (
        total_floor_area_m2 IS NULL
        OR total_floor_area_m2 >= 0
    ),
    CONSTRAINT transaction_observations_total_floor_area_bound_has_value CHECK (
        total_floor_area_m2 IS NOT NULL
        OR total_floor_area_is_lower_bound = false
    ),
    CONSTRAINT transaction_observations_municipality_code_tokyo CHECK (
        municipality_code ~ '^13[0-9]{3}$'
    ),
    CONSTRAINT transaction_observations_prefecture_name_not_blank CHECK (
        btrim(prefecture_name) <> ''
    ),
    CONSTRAINT transaction_observations_municipality_name_not_blank CHECK (
        btrim(municipality_name) <> ''
    ),
    CONSTRAINT transaction_observations_station_walk_minutes_non_negative CHECK (
        station_walk_minutes IS NULL
        OR station_walk_minutes >= 0
    ),
    CONSTRAINT transaction_observations_updated_not_before_created CHECK (
        updated_at >= created_at
    )
);

CREATE INDEX transaction_observations_import_run_id_idx
    ON transaction_observations (import_run_id);
CREATE INDEX transaction_observations_dataset_id_idx
    ON transaction_observations (dataset_id);
CREATE INDEX transaction_observations_raw_record_id_idx
    ON transaction_observations (raw_record_id);
CREATE INDEX transaction_observations_source_record_hash_idx
    ON transaction_observations (source_record_hash);
CREATE INDEX transaction_observations_ward_period_idx
    ON transaction_observations (
        municipality_code,
        transaction_year,
        transaction_quarter
    );
CREATE INDEX transaction_observations_asset_period_idx
    ON transaction_observations (
        asset_type,
        transaction_year,
        transaction_quarter
    );

CREATE TABLE transaction_location_contexts (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_observation_id uuid NOT NULL REFERENCES transaction_observations (id)
        ON DELETE CASCADE,
    location_precision text NOT NULL,
    location geometry(Geometry, 4326),
    source_location_label text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT transaction_location_contexts_observation_unique UNIQUE (
        transaction_observation_id
    ),
    CONSTRAINT transaction_location_contexts_precision_known CHECK (
        location_precision IN (
            'exact_point',
            'nearest_station_point',
            'district_centroid',
            'ward_polygon',
            'unknown'
        )
    ),
    CONSTRAINT transaction_location_contexts_precision_geometry_consistent CHECK (
        (
            location_precision = 'unknown'
            AND location IS NULL
        )
        OR (
            location_precision IN (
                'exact_point',
                'nearest_station_point',
                'district_centroid'
            )
            AND location IS NOT NULL
            AND GeometryType(location) = 'POINT'
        )
        OR (
            location_precision = 'ward_polygon'
            AND location IS NOT NULL
            AND GeometryType(location) IN ('POLYGON', 'MULTIPOLYGON')
        )
    ),
    CONSTRAINT transaction_location_contexts_updated_not_before_created CHECK (
        updated_at >= created_at
    )
);

CREATE INDEX transaction_location_contexts_precision_idx
    ON transaction_location_contexts (location_precision);
CREATE INDEX transaction_location_contexts_location_gix
    ON transaction_location_contexts USING gist (location)
    WHERE location IS NOT NULL;

ALTER TABLE validation_issues
    ADD COLUMN transaction_observation_id uuid,
    ADD CONSTRAINT validation_issues_transaction_observation_import_run_fk FOREIGN KEY (
        transaction_observation_id,
        import_run_id
    ) REFERENCES transaction_observations (id, import_run_id);

CREATE INDEX validation_issues_transaction_observation_id_idx
    ON validation_issues (transaction_observation_id)
    WHERE transaction_observation_id IS NOT NULL;
