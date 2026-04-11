fn main() {
    println!("{}", check_palindrome("racecar"));
}
fn check_palindrome(text: &str) -> bool {
    text.chars().rev().collect::<String>() == text
}
