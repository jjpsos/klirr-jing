use crate::prelude::*;


/// Converts any `Serialize`able Rust struct into Typst syntax.
pub fn to_typst_let<T: Serialize>(name: &str, data: &T) -> String {
    let value = serde_json::to_value(data).expect("Serialization failed");
    format!("#let {} = {}", name, to_typst_value(&value, 0))
}

/// Recursively converts a serde_json::Value into pretty-printed Typst syntax.
fn to_typst_value(value: &Value, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let next_indent = indent + 1;
    let next_indent_str = "  ".repeat(next_indent);

    match value {
        Value::Object(map) => {
            // Flatten single-entry enum-like objects (e.g. { "Net": 30 }) to (net: 30)
            if map.len() == 1 {
                if let Some((variant, inner)) = map.iter().next() {
                    if inner.is_number() || inner.is_string() || inner.is_object() {
                        return format!(
                            "(\n{}{}: {},\n{})",
                            next_indent_str,
                            variant.to_lowercase(),
                            to_typst_value(inner, next_indent),
                            indent_str
                        );
                    }
                }
            }

            let fields = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}{}: {}",
                        next_indent_str,
                        k,
                        to_typst_value(v, next_indent)
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n");

            format!("(\n{},\n{})", fields, indent_str)
        }

        Value::Array(arr) => {
            let items = arr
                .iter()
                .map(|v| format!("{}{}", next_indent_str, to_typst_value(v, next_indent)))
                .collect::<Vec<_>>()
                .join(",\n");

            format!("[\n{},\n{}]", items, indent_str)
        }

        Value::String(s) => format!("\"{}\"", s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Null => "none".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_typst_let() {
        let input = InvoiceInputData::sample();
        let typst = to_typst_let("invoice", &input);
        let expected = include_str!("../../render/src/input.typ");
        pretty_assertions::assert_eq!(typst, expected);
    }
}
