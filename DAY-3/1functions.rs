fn main(){
    let a = 10;
    let b = 20;
    add_num(a,b);

    let sub = sub_num(b,a);
    println!("Subtraction - {}",sub);

    let mult = mul_num(a,b);
    println!("{}",mult)
}

//Function without returning anything
fn add_num(a:i32 , b:i32){
    println!("Addition - {}",a+b);
}

//Function returning a value
fn sub_num(a : i32 , b : i32) -> i32 {
    return a-b;
}

fn mul_num(a:i32 , b:i32) -> String {
    let mult = a*b;
    let str = format!("Multiplicatio - {mult}");
    return str;
}