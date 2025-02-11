fn main(){
    let s1 = String::from("hello");
    
    //let s2 = s1; 
    // -> in rust, it will "move" the ptr from s1 to s2
    //    in order to reduce the cost on space 
    
    let s2 = s1.clone();
    // we need to use "clone" for string

    println!("s1={}", s1);
    println!("s2={}", s2);


    {
        let s3 = s1;
        println!("s3={}", s3);
    }
    //println!("s1={}", s1);
    // even out of the spec of s3, s1 still can't be used
    // (arleady deleted)

    let x = 5;
    let y = x;
    // for int, it will do "clone" automatically

    println!("x={}", x);
    println!("y={}", y);
}
