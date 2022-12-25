//Given a list of integers, use a vector to
//return the median - value in the middle position
use std::collections::HashMap;
fn main() {
    let mut odd: Vec<i32> = vec![3, 5, 6, 7, 3, 9, 4, 10, 23, 58, 32, 21, 4, 3, 5, 6, 6];
    let mut even: Vec<i32> = vec![3, 5, 6, 7, 3, 9, 4, 10, 23, 58, 32, 21, 4, 3, 5, 6];
    
    median(&mut odd);
    median(&mut even);
    mode(&odd);


    let str: String = String::from("associated value");

    pig_latin(&str);


    let mut ledger: HashMap<&str, &str> = HashMap::new();

    ledger.insert("Amir", "Sales");
    ledger.insert("Ivan", "Sales");

    let sally_eng: String = String::from("Add Sally to Engineering");
    let emma_sales: String = String::from("Add Emma to Sales");

    employ(&sally_eng, &mut ledger);
    employ(&emma_sales, &mut ledger);
    list_department(String::from("Sales"), &ledger);


}

//return median - the value in the middle
fn median(vec: &mut Vec<i32>) {
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

//return mode - the value that occurs most often
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

//Using a hash map and vectors, create a text interface to allow a user to add employee names to
//a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
//Then let the user retrieve a list of all people in a department or
//all people in the company by department, sorted alphabetically.
fn employ<'a, 'b> (person: &'a String, hashmap: &'b mut HashMap<&'a str, &'a str>) -> &'b HashMap<&'a str, &'a str> {
    let mut word = person.split_whitespace();
    word.next();
    let name: &'a str = word.next().unwrap();
    word.next();
    let dep: &'a str = word.next().unwrap();
    hashmap.insert(&name, &dep);
    println!("{:?}", &hashmap);
    return hashmap
} // thanks icewind

fn list_department<'a, 'b> (dep: String, hashmap: &'b HashMap<&'a str, &'a str>) {
    let mut list: Vec<&&str> = Vec::new();
    for (k, v) in hashmap {
        if v.to_string() == dep {
            list.push(&k);
        } else { () }
    }
    println!("{:?}", list);
}

//Convert strings to pig latin.
//The first consonant of each word is moved to the end of the word and “ay” is added,
//so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
//Keep in mind the details about UTF-8 encoding!
fn pig_latin (str: &String) {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    for word in str.split_whitespace() {
        if vowels.contains(&word.chars().nth(0).unwrap()) {
            println!("{word}-hay");
        } else {
            let first = &&word.chars().nth(0).unwrap();
            let end = &word[1..];
            println!("{end}-{first}ay");
        }
    }
}