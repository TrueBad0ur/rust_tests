fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

fn main() {
    let string1 = String::from("long string is long");

    println!("The longest string is {}", string1);
}

