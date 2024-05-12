pub fn ok_response() -> &'static str {
    "+OK\r\n"
}

pub fn error_response() -> &'static str {
    "-ERROR\r\n"
}

pub fn bulk_string_response(value: &str) -> &'static str {
    let response = format!("${}\r\n{}\r\n", value.len(), value);
    // Return the response as a static string
    // This is necessary because the response needs to live for the entire duration of the program
    // The 'static lifetime is the longest possible lifetime, and it means that the string will live for the entire duration of the program.
    Box::leak(response.into_boxed_str())
}

pub fn boolean_response(value: bool) -> &'static str {
    if value {
        ":1\r\n"
    } else {
        ":0\r\n"
    }
}
