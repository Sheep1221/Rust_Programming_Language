fn main(){
    let mut s1 = String::from("Hello");
    
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    let len = change(& mut s1);
    println!("The length of {} is {}", s1, len);

    let r1 = &s1;
    let r2 = &s1;
    //let r3 = & mut s1;  //--> Cannot borrow mutable and immutable at the same time
    println!("r1={}, r2={}", r1, r2);
    
    let r3 = & mut s1; 
    //Since r1 and r2 will not used after this line, so we can borrow it as mutable
    println!("r3={}", r3);
}

// we can also use "&str" here
fn calculate_length(s: &String) -> usize{
    return s.len();
}

// we can't use "& mut str" here
fn change(s: & mut String) -> usize{
    s.push_str(" World");
    return s.len();
}

// &str -> string literal or string slice
// string literal -> fixed
// string slice -> (eg. let slice: &str = &s[0..5];)

// &String -> reference to String
// dynamic, can be changed, stored in heap
