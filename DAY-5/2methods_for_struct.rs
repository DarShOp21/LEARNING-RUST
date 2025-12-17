struct Rectangle {
    length : i32,
    width : i32
}

impl Rectangle {
    fn new(length:i32 , width : i32) -> Self {
        Self{length,width}
    }

    fn area(&self)->i32{
        self.length*self.width
    }

    fn compare(&self , other : &Rectangle)->bool{
        self.area() > other.area()
    }
}

fn main(){
    let r1 = Rectangle::new(10,20);
    let r2 = Rectangle::new(5,15);

    println!("{}",r1.compare(&r2));
}