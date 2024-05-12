pub fn ok_response() -> String {
    return "+OK\r\n".to_owned();
}

pub fn error_response(key: &str, value: &str) -> String {
    // Format the error response
    return format!("-{} {}\r\n", key.to_uppercase(), value);
}

pub fn bulk_string_response(value: &str) -> String {
    return format!("${}\r\n{}\r\n", value.len(), value);
}

pub fn boolean_response(value: bool) -> String {
    if value {
        return ":1\r\n".to_owned();
    } else {
        return ":0\r\n".to_owned();
    }
}
