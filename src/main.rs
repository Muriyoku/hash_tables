use std::{collections::HashMap};

fn main() {
    let mut fruits: HashMap<String, f64> = HashMap::new();

    insert_or_incress_fruit_value("banana".to_string(), &mut fruits);
    insert_or_incress_fruit_value("apple".to_string(), &mut fruits);
    insert_or_incress_fruit_value("barry".to_string(), &mut fruits);
    insert_or_incress_fruit_value("apple".to_string(), &mut fruits);

    word_counter();

    let mut remove_fruits_hash: HashMap<String, f64> = HashMap::new();

    remove_fruits_hash.insert("orange".to_string(), 1.0);
    remove_fruits_hash.insert("banana".to_string(), 4.0);
    remove_fruits_hash.insert("berry".to_string(), 2.0);

    remove_fruits("orange".to_string(), &mut remove_fruits_hash);
    remove_fruits("apple".to_string(), &mut remove_fruits_hash);
}

fn insert_or_incress_fruit_value(fruit: String, list: &mut HashMap<String, f64>) {
    if list.contains_key(&fruit) {
        list.entry(fruit.clone()).and_modify(|p| *p += 1.0);

        println!("Price of {} was incresed in: {}", fruit, 1.0);
    } else {
        let price: f64 = 3.0;

        list.insert(fruit.clone(), price);

        println!("Fruit: {} added with price: {}", fruit, price);
    };
}
fn remove_fruits(fruit: String, list: &mut HashMap<String, f64>) {
    match list.remove(&fruit) {
        Some(_) => println!("Removed {} successfully", fruit), 
        None => println!("Item was not found"),
    }
}
fn word_counter() {
    let words: Vec<String> = vec![
        "rust".to_string(),
        "hashmap".to_string(),
        "rust".to_string(),
        "exercise".to_string(),
    ];

    let mut word_couter_hashmap: HashMap<String, i32> = HashMap::new();

    for w in words { 
        word_couter_hashmap.entry(w)
        .and_modify(|c| *c += 1)
        .or_insert(1);
    }

    println!("{:?}", word_couter_hashmap);
}