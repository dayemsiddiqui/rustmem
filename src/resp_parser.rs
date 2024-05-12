use std::default;

pub fn process_request(request: &str) -> Option<&str> {
    println!("Received: {}", request);
    let command = request.trim();
    let first_character = command.chars().next()?;
    let response = match first_character {
        '+' => ok_response(),
        '-' => error_response(),
        '*' => parse_array(&command),
        _ => error_response(),
    };

    return Some(response);
}

fn ok_response() -> &'static str {
    "+OK\r\n"
}

fn error_response() -> &'static str {
    "-ERROR\r\n"
}

fn parse_array(command: &str) -> &'static str {
    let second_character = command
        .chars()
        .nth(1)
        .expect("Failed to get second character");
    let array_length = second_character
        .to_digit(10)
        .expect("Failed to parse array length");
    let command_terms = command.split_whitespace().collect::<Vec<&str>>();
    let command_length = command_terms.len();
    println!("Commands, length: {:?}", command_length);
    for term in command_terms {
        println!("Term: {}", term);
    }
    return ok_response();
}
