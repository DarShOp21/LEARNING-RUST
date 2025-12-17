struct Book{
    name : String,
    author : String,
    price : i32,
    pages : i32
}

fn main(){
    let book1 = Book{
        name : String::from("How to make Epic Money"),
        author : String::from("Ankoor Warikoo"),
        price : 399,
        pages : 250
    };

    let book2 = Book{
        name : String::from("How to make friends and influence people"),
        author : String::from("Dale Carneggle"),
        price : 299,
        pages : 359
    };

    let books = vec![book1,book2];
    let mut bookCount = 1;
    for book in &books{
        println!("Book-{}",bookCount);
        println!("{}",book.name);
        println!("{}",book.author);
        println!("{}",book.price);
        println!("{}",book.pages);
        println!("\n");
        bookCount += 1;
    };
}