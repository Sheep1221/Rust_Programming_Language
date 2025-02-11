fn main(){
    let s1 = "Hello World";
    let s2 = &s1[0..5];
    let s3 = &s1[6..11];
    println!("s1={}",s1);
    println!("s2={}",s2);
    println!("s3={}",s3);
    
    let s4 = first_word1(&s1);
    println!("s4={}", s4);
    
    first_word2(&s1);

    first_word3(&s1);

    first_word4(&s1);
}


// first_word 1 & 2 use "as_bytes()" -> which can only deal with ASCII
// first_word 3 & 4 use "chars()" -> which can deal with ASCII and Unicode

fn first_word1(s: &str) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s;
}

fn first_word2(s: &str){
    let bytes = s.as_bytes();
    println!("In first_word_2");
    for i in 0..bytes.len(){
        if bytes[i] == b' '{
            break;
        }
        println!("{}", bytes[i] as char);
    }
    println!();
}

fn first_word3(s: &str){
    println!("In first_word_3");
    for i in 0..s.chars().count(){
        let c = s.chars().nth(i).unwrap_or(' ');
        if c == ' '{
            break;
        }
        println!("{}", c);
    }
    println!();
}


fn first_word4(s: &str){
    println!("In first_word_4");
    // for (i, c) in s.chars().enumerate() { 
    // --> we can use it to get the index i;
    for c in s.chars() {
        if c == ' '{
            break;
        }
        println!("{}", c);
    }
    println!();
}
