use std::fs::File;
use std::fs::io;
use std::io::{self, Read};
use std::error::Error;

// () or any Error type
fn main() -> Result<(), Box<dyn Error>> {
    //verbose with match
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    //less verbose with "?" -> Ok
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    //use "?" where a recoverable error might occur
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }


    // Error handling through type declaration
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
    
            Guess { value }
        }
    
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
