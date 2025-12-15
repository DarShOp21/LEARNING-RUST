fn main(){
    let string = String::from("Hello");    
    println!("{}",string);     //Hello

    printString(string);

    println!("{}",string);     //Error will occur , explain in the image "2functionNstring.png"
}

fn printString(str : String){
    println!("{}",str);

}