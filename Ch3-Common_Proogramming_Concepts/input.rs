use std::io;

fn main() {
    let mut input = String::new();

    println!("Please Input a number Here:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // We need to trnasfer it in our side if we want the input as an integer
    // trim() -> remove white space
    // parse() -> it can convert a string to integer
    let number:i32 = input.trim().parse().expect("Input not an integer");
    println!("The input value is: {number}");


    input = "".to_string();
    println!("Please Input a string Here:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("The input value is: {input}");

}
