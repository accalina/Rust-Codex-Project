
pub fn run(){

    // (&str) This is primitive string, immutable fixed length string somewhere in memory
    let hello = "Hello ";
    println!("{}", hello);

    // (str)
    let mut name = String::from("Acc");  // create string from text
    name.push('a');  // add char into 'name' string
    name.push_str("lina");  // add string into 'name' string
    println!("{}", name);

    let mut name2 = "Se".to_string();
    name2.push_str("th");
    println!("{}", name2);    

    let mut greeting = String::from(hello);
    greeting.push_str(&name);
    println!("{}", greeting);

    let (system_name, is_alive) = ("x230", true);
    println!("System {} is {}", system_name, is_alive);

}