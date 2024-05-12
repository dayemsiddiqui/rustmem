use crate::response_builder::{
    boolean_response, bulk_string_response, error_response, ok_response,
};
/**
* This module is responsible for parsing the RESP protocol.
* The RESP protocol is a simple text-based protocol that is used by Redis.
* For the purposes of this project, we will only implement a subset of the RESP protocol.
* We will support the following commands:
* - SET key value
* - GET key
* - DEL key
* - EXISTS key
* - INCR key
* - DECR key
* - QUIT

======================++++++++++++++++++++++++++++++======================

The client will send a command to the server in the following format:
- SET key value
- Command: *3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
- Response: +OK\r\n

- GET key
- Command: *2\r\n$3\r\nGET\r\n$3\r\nkey\r\n
- Response: $5\r\nvalue\r\n

- DEL key
- Command: *2\r\n$3\r\nDEL\r\n$3\r\nkey\r\n
- Response: +OK\r\n

- EXISTS key
- Command: *2\r\n$6\r\nEXISTS\r\n$3\r\nkey\r\n
- Response: :1\r\n

- QUIT
- Command: *1\r\n$4\r\nQUIT\r\n
- Response: +OK\r\n
*/
// Define  a global key-value store
//
// The key-value store will be a simple HashMap that stores the key-value pairs.
//
// The key-value store will be a static variable, which means that it will be shared across all threads.
use std::collections::HashMap;

static mut KV_STORE: Option<HashMap<String, String>> = None;

fn get_kv_store() -> &'static mut HashMap<String, String> {
    unsafe { KV_STORE.get_or_insert_with(|| HashMap::new()) }
}

pub fn process_request(request: &str) -> &'static str {
    let mut parts = request.split_whitespace();
    println!("Processing request...");
    // Third element is the command
    let command = parts.nth(2).expect(error_response());
    match command {
        "SET" => set_command(parts),
        "GET" => get_command(parts),
        "DEL" => del_command(parts),
        "EXISTS" => exists_command(parts),
        "QUIT" => quit_command(),
        _ => error_response(),
    }
}

fn set_command(mut parts: std::str::SplitWhitespace) -> &'static str {
    println!("Executing SET command...");
    let key = parts.nth(1).expect(error_response());
    let value = parts.nth(1).expect(error_response());
    get_kv_store().insert(key.to_string(), value.to_string());
    ok_response()
}

fn get_command(mut parts: std::str::SplitWhitespace) -> &'static str {
    println!("Executing GET command...");
    let key = parts.nth(1).expect(error_response());
    let kv_store = get_kv_store();
    if let Some(value) = kv_store.get(key) {
        return bulk_string_response(value);
    }
    println!("GET key: {}", key);
    error_response()
}

fn del_command(mut parts: std::str::SplitWhitespace) -> &'static str {
    println!("Executing DEL command...");
    let key = parts.nth(1).expect(error_response());
    let kv_store = get_kv_store();
    if kv_store.remove(key).is_some() {
        return ok_response();
    }
    error_response()
}

fn exists_command(mut parts: std::str::SplitWhitespace) -> &'static str {
    print!("Executing EXISTS command...");
    let key = parts.nth(1).expect(error_response());
    let kv_store = get_kv_store();
    if kv_store.contains_key(key) {
        return boolean_response(true);
    }
    boolean_response(false)
}

fn quit_command() -> &'static str {
    println!("QUIT");
    ok_response()
    // Close the connection
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_command() {
        let request = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let response = process_request(request);
        assert_eq!(response, "+OK\r\n");
    }

    #[test]
    fn test_get_command() {
        // When
        let set_request = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        process_request(set_request);

        let request = "*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n";
        let response = process_request(request);
        assert_eq!(response, "$5\r\nvalue\r\n");
    }

    #[test]
    fn test_del_command() {
        // When
        let set_request = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        process_request(set_request);

        let request = "*2\r\n$3\r\nDEL\r\n$3\r\nkey\r\n";
        let response = process_request(request);
        assert_eq!(response, "+OK\r\n");
    }

    #[test]
    fn test_exists_command() {
        // When
        // Set the key
        let set_request = "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let response = process_request(set_request);

        // Wait for the response
        assert_eq!(response, "+OK\r\n");

        // Then
        let request = "*2\r\n$6\r\nEXISTS\r\n$3\r\nkey\r\n";
        let response = process_request(request);
        assert_eq!(response, ":1\r\n");
    }

    #[test]
    fn test_quit_command() {
        let request = "*1\r\n$4\r\nQUIT\r\n";
        let response = process_request(request);
        assert_eq!(response, "+OK\r\n");
    }
}
