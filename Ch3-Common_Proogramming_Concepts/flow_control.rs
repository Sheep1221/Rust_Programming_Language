fn main(){
    let mut num = 3;
    // In rust, we must using {} for if else statement
    if num < 5 {
        println!("The numuber < 5");
    }
    else {
        println!("The number > 5");
    }

    num = if num<5 {100} else {0};
    println!("num after changed = {num}\n");

    // Must in different datatype
    //let num2 = if num<5 {100} else {"zero"};
    //println!("num2 = {num2}");

    // rust has no C style for loop
    // This is same as for(i=0; i<10; ++i)
    for i in 0..10 {
        println!("In for loop: {i}");
    }
    
    println!("");

    let mut cnt = 0;
    
    // We can use label to leave multiple layers of loop when break
    'jumpout: loop{
        println!("Jump out inner loop");
        loop{
            cnt+=1;
            println!("count = {cnt}");
            if cnt>=10 {
                break 'jumpout;
            }
            else if cnt%3 == 0 {
                break;
            }
        }
    }
}
