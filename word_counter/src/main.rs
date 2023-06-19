use std::io;

fn count(text: &str) {
    let no_of_lines = text.lines().count();
    let no_of_words = text.split_whitespace().count();
    let no_of_characters = text.len();
    println!("line: {:?}\nwords: {:?} \ncharacters: {:?}", no_of_lines, no_of_words, no_of_characters);
}

fn main() {
    let mut text = String::new();
    println!("write the text \n");
    io::stdin().read_line(&mut text).expect("failed to read the line");
    count(&text)
}
