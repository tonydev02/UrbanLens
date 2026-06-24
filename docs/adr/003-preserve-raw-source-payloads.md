# ADR 003 — Preserve Raw Source Payloads

## Status

Accepted — 2026-06-24

## Context

Public datasets contain blanks, categorical values, formatting quirks, revisions, and fields whose meaning changes by asset type. Normalization logic will evolve. Discarding original rows would make corrections, audits, and reproducible metric explanations impossible.

## Decision

Preserve every received source record and its exact dataset-artifact lineage before or alongside normalization. Record the dataset checksum/query, import run, source row ordinal/feature position, raw field representation, payload hash, validation result, and normalization-logic version.

Normalized observations reference their originating raw record. Rejected records remain available with structured issues. Raw payloads are not returned by default product queries or written to production logs.

## Alternatives Considered

- Store normalized data only: rejected because transformations cannot be audited or replayed.
- Store only the original downloaded file: rejected because record-level provenance and rejection analysis would be difficult.
- Log raw payloads: rejected because logs are not durable governed storage and may expose unnecessary details.
- Mutate raw records when schemas change: rejected because it destroys historical evidence.

## Consequences

- Storage use is higher and retention/backups must include raw artifacts/records.
- Reprocessing and normalization-version migrations become possible.
- Provenance queries can explain a displayed value.
- Access to raw content must be intentional and logged when administration is added.
- Artifact and record hashing must preserve legitimate duplicate rows rather than collapse them.
