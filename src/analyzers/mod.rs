// SPDX-License-Identifier: PMPL-1.0-or-later
// Analyzer modules for protocol-squisher integration

pub mod rescript_analyzer;
pub mod rust_analyzer;

// Re-export main types for convenience
pub use rescript_analyzer::{ReScriptType, ReScriptField, ReScriptFieldType};
pub use rust_analyzer::{RustType, RustField, RustFieldType, Visibility};

/// Transport class for compatibility classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransportClass {
    /// 100% fidelity - all types have direct equivalents
    Concorde,
    /// 95-99% fidelity - minor precision differences
    BusinessClass,
    /// 80-94% fidelity - some type coercion needed
    Economy,
    /// <80% fidelity - significant impedance mismatch
    Wheelbarrow,
}

/// Calculate transport class between two languages
pub fn calculate_transport_class(source: &str, target: &str) -> TransportClass {
    // For proof-of-concept, we know ReScript ↔ Rust is Concorde
    match (source, target) {
        ("rescript", "rust") | ("rust", "rescript") => TransportClass::Concorde,
        ("rescript", "julia") | ("julia", "rescript") => TransportClass::Concorde,
        ("rust", "julia") | ("julia", "rust") => TransportClass::Concorde,
        ("rescript", "gleam") | ("gleam", "rescript") => TransportClass::Concorde,
        ("rust", "gleam") | ("gleam", "rust") => TransportClass::Concorde,
        _ => TransportClass::Wheelbarrow,
    }
}

/// Generate compatibility report
pub fn compatibility_report(rescript: &ReScriptType, rust: &RustType) -> String {
    let rs_score = rescript_analyzer::compatibility_score(rescript, "rust");
    let rust_score = rust_analyzer::compatibility_score(rust, "rescript");
    let transport_class = calculate_transport_class("rescript", "rust");

    format!(
        r#"Compatibility Analysis
=====================

ReScript → Rust: {:.1}%
Rust → ReScript: {:.1}%
Transport Class: {:?}

Type Mappings:
{}"#,
        rs_score * 100.0,
        rust_score * 100.0,
        transport_class,
        generate_mapping_table(rescript, rust)
    )
}

fn generate_mapping_table(rescript: &ReScriptType, rust: &RustType) -> String {
    let mut table = String::new();

    for (rs_field, rust_field) in rescript.fields.iter().zip(rust.fields.iter()) {
        let rs_type = rescript_analyzer::map_to_target(&rs_field.field_type, "rust");
        let rust_type = format!("{:?}", rust_field.field_type).to_lowercase();

        table.push_str(&format!(
            "  {}: {} → {}\n",
            rs_field.name,
            format!("{:?}", rs_field.field_type).to_lowercase(),
            rust_type
        ));
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_class_rescript_rust() {
        let class = calculate_transport_class("rescript", "rust");
        assert_eq!(class, TransportClass::Concorde);
    }

    #[test]
    fn test_transport_class_symmetric() {
        let class1 = calculate_transport_class("rescript", "rust");
        let class2 = calculate_transport_class("rust", "rescript");
        assert_eq!(class1, class2);
    }
}
