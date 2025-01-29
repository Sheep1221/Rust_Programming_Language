fn main(){
    let tup:(i32, f64, char) = (123, 123.0, 'z');
    let (x, y, z) = tup;

    println!("x={x}");
    println!("y={y}");
    println!("z={z}");


    
    let mut arr1 = [0; 5];
    arr1[2] = 123;
    println!("arr1:");
    for i in arr1{
        println!("{i}");
    }
    

    //let mut arr2:[i32;5];
    //let mut arr2:[i32;5]; -> Can't be used without intialized
    let mut arr2:[i32;5] = [1,2,3,4,5];
    println!("arr2:");
    for i in arr2{
        println!("{i}");
    }

}
