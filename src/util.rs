pub fn make_json_pretty(raw_json: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(raw_json) {
        Ok(parsed_json) => {
            serde_json::to_string_pretty(&parsed_json).unwrap_or_else(|_| raw_json.to_string())
        }
        Err(_) => raw_json.to_string(),
    }
}
