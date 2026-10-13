# CMS-0057 patient access API map — Rust

Small library and CLI that sanity-check a **patient access capability map** JSON before you publish API documentation or run gap analysis against **CMS-0057-F** expectations: which resources are exposed for patient access and whether **bulk export** is flagged.

Synthetic fixtures only. Not ONC certification, not a substitute for Inferno or attestation workflows, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 13** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [CMS Interoperability and Patient Access](https://www.cms.gov/regulations-and-guidance/guidance/interoperability/index) (context only—not legal advice)

## What it checks

- **`patient_access_resources`** is a non-empty array (e.g. Patient, Coverage, ExplanationOfBenefit)
- Warning if **`bulk_export_supported`** is not **`true`** (patient access programs typically expect a bulk path)

| Problem | What breaks |
|--------|-------------|
| **Empty resource list** | Patient app scopes don’t match what the server actually serves |
| **Bulk flag false/missing** | App store review or payer QA flags missing export pathway |
| **Map out of sync with CapabilityStatement** | Documentation passes, production endpoints don’t |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-cms-map -- examples/valid-cms-map.json
cargo run --bin validate-cms-map -- examples/invalid-cms-map.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-cms-map -- examples/valid-cms-map.json
```

**Pass** (`examples/valid-cms-map.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-cms-map.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "patient_access_resources",
    "detail": "CMS-0057 map should list Patient, Coverage, etc.",
    "rule_id": "cms-resources"
  }
]
```


## Layout

- [docs/architecture.md](./docs/architecture.md) — patient access map artifact
- `src/validate.rs` — rules
- `examples/` — valid and broken maps for tests and demos

## Inferno vs this repo

Inferno exercises **Patient Access** and **Bulk** scenarios on live FHIR APIs. This repo is **your internal API map artifact** — the checklist product and compliance teams align before certification runs.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
