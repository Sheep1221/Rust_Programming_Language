fn main(){
    let x = 123;
    let s = "Hello".to_string();
    copy_int(x);
    println!("x={}", x);
    
    copy_string(s.clone());
    println!("s={}", s);

    let s2 = transfer_string(s);
    println!("s2={}", s2);
    //println!("s={}", s);
    // Since the ownership of "s" is taken by transfer_string
    // So we can't use it any more
}

fn copy_string(str1: String){
    println!("In copy_string; str1={}", str1);
}

fn copy_int(val: i32){
    println!("In copy_int; val={}",val);
}

fn transfer_string(str1: String) -> String{
    let str2 = str1.clone() + " World";
    // if we still need to use str1, we also need to clone() it
    println!("[transfer_string] str1 = {}", str1);
    return str2
}
