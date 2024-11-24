use std::collections::HashMap;

pub fn hash_map() {
    let mut users = HashMap::new();
    users.insert(String::from("harsh"), 22);

    let first_user_age = users.get("fnkd");

    match first_user_age {
        Some(age) => println!("Age is {}", age),
        None => println!("User not found"),
    }
}
