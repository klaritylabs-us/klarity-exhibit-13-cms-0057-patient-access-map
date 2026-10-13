use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_map_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    let resources = v.get("patient_access_resources").and_then(|x| x.as_array());
    if resources.is_none() || resources.unwrap().is_empty() {
        r.push(ValidationIssue::error("patient_access_resources", "CMS-0057 map should list Patient, Coverage, etc.", "cms-resources"));
    }
    if v.get("bulk_export_supported").and_then(|x| x.as_bool()) != Some(true) {
        r.push(ValidationIssue::warning("bulk_export_supported", "patient access programs expect bulk export path", "bulk-flag"));
    }
    Ok(r)
}
