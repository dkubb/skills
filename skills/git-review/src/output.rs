//! Write machine-readable command results.

use std::io::Write;

use serde_json::Value;
use skill_core::SkillError;

/// Build a JSON object from statically named fields.
pub(crate) fn json_object<const FIELD_COUNT: usize>(fields: [(&str, Value); FIELD_COUNT]) -> Value {
    Value::Object(
        fields
            .into_iter()
            .map(|(name, value)| (name.to_owned(), value))
            .collect(),
    )
}

/// Write one JSON value followed by the JSON Lines record separator.
pub(crate) fn write_json_line(out: &mut dyn Write, value: &Value) -> Result<(), SkillError> {
    writeln!(out, "{value}")?;
    Ok(())
}
