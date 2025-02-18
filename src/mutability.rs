fn main() {
    let mut name = String::new();
    println!("Enter your name: ");
    std::io::stdin().read_line(&mut name).unwrap();
    let mut name = name.trim().to_string();
    add_to_string(&mut name);

    println!("Hello, {}", name);
    println!("The length of the name is {}", get_len(&name));
}

fn add_to_string(s: &mut String) {
    s.insert_str(0, "Buenos días, ");
    s.push_str("!");
}
fn get_len(s: &String) -> usize {
    s.len()
}
