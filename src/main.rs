use data_structures::linked_list::List;

fn main() {
    linked_list();
}

fn linked_list() {
    let mut l = List::new();
    l.insert(0, "a".to_string());
    l.insert(1, "b".to_string());
    l.insert(2, "c".to_string());
    l.insert(3, "d".to_string());
    l.insert(4, "e".to_string());
    match l.at(0) {
        None => println!("Nothing at index 0"),
        Some(value) => println!("Found '{}' at index 0", value)
    }
    l.update(1, "aa".to_string());
    l.delete(0);
    match l.find(&"aa".to_string()) {
        None => println!("'aa' not found"),
        Some(index) => println!("Found 'aa' at index {}", index)
    }
}