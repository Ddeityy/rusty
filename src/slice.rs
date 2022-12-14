fn main() {
    let s: &str = "hello world";
    let word = first_word(&s);
    let sword = second_word(&s);
    println!("{word}, {sword}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); //convert string (slice) to bytes

    //convert bytes into an enum array
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s
}
