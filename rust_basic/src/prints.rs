
pub fn run(){
    // Basic Formating
    println!("Hello {}, Welcome to {}!", "Accalina", "Rustlang");

    // Positional Arguments
    println!("{0} likes to {1}, and {0} likes to {2}", "Seth", "Code", "Play");

    // Named Arguments
    println!("{person} is {me}'s friends", person="Seth", me="Accalina");

    // Placeholder traits
    println!("Binary: {:b}. Hex: {:x}. Octal: {:o}.", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10+10)
}