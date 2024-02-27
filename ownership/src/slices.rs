fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is {}", word);

    let s2="this is a string slice directly ";
    let word2=first_word_for_slice(s2);
    println!("the first word is {}", word2);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_for_slice(s: &str)->&str{
    let bytes= s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
return &s[0..i]
        }
    }
    &s[..]

}
