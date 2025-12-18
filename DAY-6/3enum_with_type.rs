
enum Option<T>{
    Some(T),
    None
}

fn matching<T: std::fmt::Display>(option : &Option<T>){
    match option{
        Option::Some(value) => println!("{}",value),
        Option::None => println!("NONE"),
    }
}

fn main(){
    let opt1 = Option::Some(String::from("SOMETHING.."));
    // let opt2 = Option::None;    //Error will occur bcoz here we are not using the Some(T) value and hence we cant get the value of T
    let opt2 : Option<i32> = Option::None;

    matching(&opt1);
    matching(&opt2);
}