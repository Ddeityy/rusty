fn main() {
    // return lifetime is valid if either x or y is valid
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T, // any type
    ) -> &'a str
    where
        T: Display, // type that implements Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let x = "a";
    let y = "aa";
    let a = "bug";

    let r = longest_with_an_announcement(x, y, a);
    println!("{r}")
}
