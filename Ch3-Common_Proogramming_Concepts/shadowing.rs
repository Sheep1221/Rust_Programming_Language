fn main(){
    let x = 5;
    println!("The value of x is {x}");
    // The first x is shadowing by the second x (if we use "let" again)
    let x = x+1;
    println!("The value of x is {x}");
    {
        // shadowing the x again in this scope
        let x = x*2;
        println!("The value of x is {x}");
    }
    // The x will turn back as it go out of the scope within bracket
    println!("The value of x is {x}");

    let str = "Hello";
    println!("str = {str}");
    // The datatype can also changed by using shadowing
    let str = str.len();
    println!("str = {str}");

    // if we already "mut" the variable, we are not able to change its type
    /*
    let mut str = "Hello";
    println!("str = {str}");
    str = str.len();
    println!("str = {str}");
    */
}
