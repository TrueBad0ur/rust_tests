fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result: &'a str = "really long string";
    result
}

fn main() {
    let string1 = String::from("long string is long");

    println!("The longest string is {}", string1);
}

