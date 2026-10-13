# Architecture

```text
Internal API inventory / CapabilityStatement review
  → patient-access-map.json (your artifact)
       → validate_cms_map()
            · patient_access_resources[]
            · bulk_export_supported flag
  → Vec<ValidationIssue>
```

| Audience | Uses this map for |
|----------|-------------------|
| Product / compliance | CMS-0057-F gap checklist before cert |
| Engineering | Align documented surface with deployed FHIR server |
| Patient apps | Scope and resource expectations vs. bulk path |

This is **not** a legal interpretation of CMS rules or a substitute for attestation. It sanity-checks a **JSON map** your team maintains.

| Module | Role |
|--------|------|
| `validate.rs` | map shape rules |
| `outcome.rs` | issue model |
| `bin/validate-cms-map.rs` | CLI |

Out of scope: Inferno execution, live endpoint probes, attorney review.
