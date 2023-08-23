use std::collections::HashMap;

fn main() {
    //Init
    let mut scores = HashMap::new();
    //Insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    //Query
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //Loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    //ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{field_name}, {field_value}");//Error,所以权已转移

    //cover
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    //entry
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    //update
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
