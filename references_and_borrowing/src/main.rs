fn greet(name: &String) {
    println!("Hello, {}.", name);
}

fn main() {
    let first_name: String = String::from("Fernando");
    let last_name: String = String::from("Pereira");

    greet(&first_name);

    let full_name = format!("{} {}", first_name, last_name);

    greet(&full_name);
}
