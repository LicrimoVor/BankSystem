fn process_numbers_old(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in numbers {
        if n > 0 && n % 2 == 0 {
            result.push(n * 2);
        }
    }
    result
}

fn process_numbers_new(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&n| n > 0 && n % 2 == 0)
        .map(|&n| n * 2)
        .collect()
}

fn find_user(id: u32) -> Option<String> {
    if id > 0 {
        Some(format!("User{}", id))
    } else {
        None
    }
}

fn get_email(user: &str) -> Option<String> {
    Some(format!("{}@example.com", user))
}

fn validate_email(email: &str) -> Result<String, String> {
    if email.contains('@') {
        Ok(email.to_string())
    } else {
        Err("Invalid email".to_string())
    }
}

fn get_validated_email(user_id: u32) -> Result<String, String> {
    find_user(user_id)
        .and_then(|user| get_email(&user))
        .ok_or_else(|| "User not found".to_string())
        .and_then(|email| validate_email(&email))
}

fn main() {
    match get_validated_email(42) {
        Ok(email) => println!("Email: {}", email),
        Err(e) => println!("Error: {}", e),
    }
}
