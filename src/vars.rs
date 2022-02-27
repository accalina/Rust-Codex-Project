
pub fn run() {
    let age = 22;   // Default is "i32"
    let rand_number: i64 = 123;   // Add type to variable
    let code_hour = 4.5;    // Default is "f64"
    let is_active = true;   // Boolean
    let is_greater = 10 > 5;
    let single_char = 'a';  // use single quotes for chars
    let face = "\u{1f600}"; // use "\u" for unicode 

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("{:?}", (age, code_hour, is_active, rand_number, single_char));
    println!("is 10 greater than 5? {} {}", is_greater, face);
}