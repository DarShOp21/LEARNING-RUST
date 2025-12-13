// Online Rust compiler to run Rust program online
// Print "Try programiz.pro" message

fn main() {

    //SIGN AND UNSIGN integers
    let num1:u8 = 10;       //unsign i.e positive number of 8 bytes 
    //Range of unsign int is -- (0 -> 255)
    let num2:i8 = -10;      //sign i.e negative and positive number of 8 bytes
    //Ranfe of sign int is -- (-128 -> 127)
    println!("{}",num1);
    println!("{}",num2);

    //LENGTH AND CAPACITY OF STRING
    let mut str = String::from("DARSHAN");
    println!("{}",str);
    println!("Length - {}",str.len());
    println!("Capacity - {}",str.capacity());
    str.push_str(" A");
    str.push_str(" NAHHS");
    println!("Length - {}",str.len());
    println!("Capacity - {}",str.capacity());
    
    
    //ACCESS TRANSFER
    let str1 = String::from("Darshan");     //Stores the data in heap memory
    let str2 = str1;        //the access for data stored at the address pointing the data of str1 is given to str2
    
    println!("{}",str2); 
    // println!("{}",str1);    //Error will occur because , in RUST a data can have only one owner , after transfering the access to str2 , the access is revoked from str1
    
    
    let num1 = 10;      //Stores the data in stack memory
    let num2 = num1;    //stack will store num2 = 10 
    println!("{} {}",num1 , num2);
    
    let char = "a";     //Stores the data in stack memory
    let char2 = char;   //stack will store num2 = 10 
    println!("{} {}",char , char2);
}
