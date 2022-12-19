fn main() {
    let user1 = User::base_user(1);
    match user1.active {
        true => println!("{:#?}", user1),
        false => println!(
            "{0}: {1} {2} is offline",
            user1.id, user1.name, user1.surname
        ),
    }
    println!("{}", user1.full_name());
}

#[derive(Debug)]
struct User<'a> {
    id: i32,
    name: &'a str,
    surname: &'a str,
    active: bool,
}

impl User<'_> {
    fn full_name(&self) -> String {
        self.name.to_owned() + " " + self.surname
    }

    fn base_user(id: i32) -> Self {
        // basically a constructor
        Self {
            id,
            name: "Base",
            surname: "User",
            active: true,
        }
    }
}
