fn main() {
    println!("{}", reverse_string("Hello guys"));
}
fn reverse_string(text: &str) -> String {
    let rev: String = text.chars().rev().collect();
    rev
}
