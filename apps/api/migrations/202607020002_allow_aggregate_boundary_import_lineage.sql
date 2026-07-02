ALTER TABLE area_boundaries
    DROP CONSTRAINT area_boundaries_raw_lineage_all_or_none,
    ADD CONSTRAINT area_boundaries_raw_lineage_consistent CHECK (
        raw_record_id IS NULL
        OR import_run_id IS NOT NULL
    );
