pub mod common {
    pub fn get_third() {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }
    pub fn hash() -> i32 {
        use std::collections::HashMap;

        let a = HashMap::from([(1, 2)]);
        let s = for (k, v) in &a {
            println!("{k}: {v}")
        };
        let mut scores: HashMap<String, i32> = HashMap::new();
        scores.insert(String::from("play"), 10);
        let x = String::from("play");
        let y = scores.get(&x).copied().unwrap_or(0);
        y
    }
}