enum Traffic{
    Red,
    Yellow,
    Green
}

impl Traffic{
    fn new() -> Self{
        Traffic::Red
    }

    fn change(&mut self){
        *self = match self{
            Traffic::Red => Traffic::Green,
            Traffic::Green => Traffic::Yellow,
            Traffic::Yellow => Traffic::Red
        };
    }

    fn status(&self) {
        match self{
            Traffic::Red => println!("Red"),
            Traffic::Green => println!("Green"),
            Traffic::Yellow => println!("Yellow")
        }
    }

    fn can_cross_by_walking(&self) -> bool{
        match self{
            Traffic::Red => true,
            _ => false
        }
    }
}

fn main(){
    let mut traffic1 = Traffic::new();

    traffic1.status();
    traffic1.change();
    traffic1.status();  
    println!("{}",traffic1.can_cross_by_walking());
    traffic1.change();
    traffic1.status();  
    println!("{}",traffic1.can_cross_by_walking());
    traffic1.change();
    traffic1.status();  
    println!("{}",traffic1.can_cross_by_walking());
}