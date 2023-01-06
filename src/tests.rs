#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            // if x.is_err()
            Err(String::from("Bitch ass"))
        }
    }

    #[test]
    fn test_longest() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[should_panic]
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
