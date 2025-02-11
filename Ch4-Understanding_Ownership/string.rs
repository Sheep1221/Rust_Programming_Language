fn main(){
    //let mut s = "Hello"; 
    // --> if we use string literal(&str), we cannot extend the string
    // --> we can only extend string with following combination
    // 1. string + string
    // 2. string + &str

    // relationship between string and &str (string literal)
    let string_obj = s.to_string(); // Convert &str to String
    let str_slice = &string_obj; // Convert String back to &str
    
    let mut s = String::from("Hello");
    //let mut s = "Hello".to_string();
    //s.push_str(", World !!!");
    s = s +", World";
    println!("s is : {s}");
}
