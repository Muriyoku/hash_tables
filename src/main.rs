use std::collections::HashMap;

fn main() {
    let mut vote: HashMap<String, bool> = HashMap::new();

    voted(&mut vote, String::from("Murilo"));
    voted(&mut vote, String::from("Emilly"));
    voted(&mut vote, String::from("Murilo"));
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
