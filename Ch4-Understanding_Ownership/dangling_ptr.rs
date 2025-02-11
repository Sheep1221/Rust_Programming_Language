fn main(){
    let refer = dangling();
    println!("{}", refer);
}

fn dangling() -> String{
    let s = String::from("Hello");
    return s;
}


// Dangling reference
// --> When we return a reference which life time already ended
// It's suggested not to return reference

//fn dangling() -> &String{
//    let s = String::from("Hello");
//    return &s;
