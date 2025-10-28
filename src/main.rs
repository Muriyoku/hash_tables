use std::collections::HashMap;

fn main() {
    let mut vote: HashMap<String, bool> = HashMap::new();

    voted(&mut vote, String::from("Murilo"));
    voted(&mut vote, String::from("Emilly"));
    voted(&mut vote, String::from("Murilo"));

    let mut fruits: HashMap<String, f64> = HashMap::new();

    insert_or_incress_fruit_value("banana".to_string(), &mut fruits);
    insert_or_incress_fruit_value("apple".to_string(), &mut fruits);
    insert_or_incress_fruit_value("barry".to_string(), &mut fruits);
    insert_or_incress_fruit_value("apple".to_string(), &mut fruits);
}

fn voted(list: &mut HashMap<String, bool>, name: String) -> bool {
    
    if list.contains_key(&name) {
        println!("You already voted: {}", name);
        return true; 
    } else {
        println!("Ok, it's finished, {}!", name);
        list.insert(name, true);
    }

    false 
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