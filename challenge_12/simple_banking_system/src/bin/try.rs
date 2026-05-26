fn main() {
let country = String::from("Austria"); // Now we have a String called co
let country_ref = &country; // country_ref is a reference to this data. 
let country = 8; // Now we have a variable called country that is an i8.
println!("{}, {}", country_ref, country); // country_ref still refers to
}