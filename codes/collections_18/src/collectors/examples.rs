
use std::collections::HashMap;

fn median_at_vector(&mut Vec<i32> numbers){

    numbers.sort();


    let wanted: Option<&i32> = numbers.get(numbers.len() / 2);


    match wanted {
        Some(val) => println!("This is value: {}",val),
        None => println!("Value is none"),
    }

}
const CONSONANTS: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n',
    'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
];

fn is_consonant(c: char) -> bool {
    CONSOANTS.contains(&c.to_ascii_lowercase())
}

fn convert_strings_to_ping_latin(word: &str) -> str {
    let mut chars = word.chars();

    match chars.next() {
        Some(first) if is_consonant(first) => {
            format!("{}{}ay",chars.as_str(), first)
        }
        Some (_) => format!("{word}way"),
        None => String::new()
    }

    
}

let mut database: HashMap<String, Vec<String>> = HashMap::new();

fn add_to_db(name_of_employee: &str, name_of_department: &str){
database.entry(name_of_employee).or_insert(name_of_department);
}

fn take_list_of_people() -> Vec<String> {

}
