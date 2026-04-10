use std::io;
fn main() {
    println!("Give sentence you wish to count words");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    println!("{}", count_words(&input));
}
fn count_words(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}
