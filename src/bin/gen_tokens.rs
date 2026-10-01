use vox_shared::schema_tokens::{COLOR_TOKENS, ICON_TOKENS};

fn exact(value: f32) -> f64 {
    value.to_string().parse().unwrap_or_default()
}

fn main() {
    let colors: Vec<_> = COLOR_TOKENS
        .iter()
        .map(|c| serde_json::json!({ "l": exact(c.l), "c": exact(c.c), "h": exact(c.h) }))
        .collect();
    let out = serde_json::json!({ "color_tokens": colors, "icon_tokens": ICON_TOKENS });
    println!("{}", serde_json::to_string_pretty(&out).unwrap_or_default());
}
