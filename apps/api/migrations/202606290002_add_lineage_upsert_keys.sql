ALTER TABLE data_sources
    ADD CONSTRAINT data_sources_name_unique UNIQUE (name);

ALTER TABLE datasets
    ADD CONSTRAINT datasets_artifact_identity_unique UNIQUE (
        source_id,
        source_dataset_name,
        retrieval_method,
        retrieval_query,
        artifact_sha256
    );
