//Given a list of integers, use a vector to
//return the median - value in the middle position
//return mode - the value that occurs most often
use std::collections::HashMap;
fn main() {
    let mut odd: Vec<i32> = vec![3, 5, 6, 7, 3, 9, 4, 10, 23, 58, 32, 21, 4, 3, 5, 6, 6];
    let mut even: Vec<i32> = vec![3, 5, 6, 7, 3, 9, 4, 10, 23, 58, 32, 21, 4, 3, 5, 6];

    fn median(mut vec: &mut Vec<i32>) {
        vec.sort();
        match vec.len() % 2 {
            0 => {
                let index: i32 = vec.len() as i32 / 2;
                let median: i32 = vec[index as usize] + vec[index as usize - 1];
                let result: f32 = median as f32 / 2.0;
                println!("The median is {:?}", vec.get(result as usize).unwrap())
            },
            1 => {
                let result = vec.len() as i32 / 2;
                println!("The median is {:?}", vec.get(result as usize).unwrap())
            },
            _ => println!("?")
        }
    }

    
    
    
    fn mode(vec: &Vec<i32>) {
        let mut hash = HashMap::new();
        for item in vec{ 
            let count = hash.entry(item).or_insert(0);
            *count += 1;
        }   
        let result = hash.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let (k, v) = result;
        println!("Mode {k} occured {v} times");
    }

    median(&mut odd);
    median(&mut even);
    mode(&odd);

}