fn main(){
    let mut str = String::from("Hello");

    let str1 = &str;            //Here the value of str is borrowed by str1
    println!("{}",str1);

    // let str2 = &mut str;
    // str2.push_str(" Hi");
    // println!("{}",str2);
    // println!("{}",str);

    //immutable borrowing using functions
    immutable_borrow(&str);

    // mutable borrowing using functions
    mutable_borrow(&mut str);

    println!("After mutable borrowing function , value of str = {}",str);
}

fn immutable_borrow(string : &String){
    println!("{}",string);
}

fn mutable_borrow(string : &mut String){
    string.push_str(" from Darshan");
    println!("{}",string);
}
