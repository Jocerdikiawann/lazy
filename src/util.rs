struct RequestData {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

pub fn generate_curl(req: &RequestData) -> String {
    let mut curl = format!("curl -X {} '{}'", req.method, req.url);

    for (key, value) in &req.headers {
        let safe_value = value.replace("'", "'\\''");
        curl.push_str(&format!(" \\\n -H '{}: {}'", key, safe_value));
    }

    if let Some(body_text) = &req.body {
        let safe_body = body_text.replace("'", "'\\''");
        curl.push_str(&format!(" \\\n -d '{}'", safe_body));
    }

    curl
}

pub fn make_json_pretty(raw_json: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(raw_json) {
        Ok(parsed_json) => {
            serde_json::to_string_pretty(&parsed_json).unwrap_or_else(|_| raw_json.to_string())
        }
        Err(_) => raw_json.to_string(),
    }
}
