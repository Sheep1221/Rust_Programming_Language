fn main(){
    func1(123, "Hello".to_string());
    func2(456, "World");
    let x = func3();
    println!("\nThe return value of func3 is {x}");
}

fn func1(x: i32, input: String){
    println!("The value of x is {x}");
    println!("The input sring is: {input}");
}

fn func2(x: i32, input: &str){
    println!("The value of x is {x}");
    println!("The input sring is: {input}");
}

fn func3() -> i32{
    return 789;
}
