fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("the length of the string '{}' is {} ", &s1, len);

    let s2 = String::from("testing without reference");
    let (s2, len) = calculate_length_without_reference(s2);
    println!("the length of the string '{}' is {} ", s2, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn calculate_length_without_reference(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
