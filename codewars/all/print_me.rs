fn print_me(msg: &str) {
    println!("msg = {}", msg);
}
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn greet(&self) {
        println!("Привет, меня зовут {}", self.name);
    }
}

fn main() {
    let string = "привет, мир";
    print_me(string);

    let owned_string = "привет, мир".to_string(); // или String::from_str("привет, мир")
    print_me(&owned_string);

    let counted_string = std::rc::Rc::new("привет, мир".to_string());
    print_me(&counted_string);

    let atomically_counted_string = std::sync::Arc::new("привет, мир".to_string());
    print_me(&atomically_counted_string);

    let person = Person { name: "Valerian" };
    person.greet();
}