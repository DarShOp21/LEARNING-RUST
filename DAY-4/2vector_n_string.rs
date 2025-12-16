fn main(){
    let mut vector : Vec<String> = Vec::new();

    vector.push(String::from("Darshan"));
    vector.push(String::from("Naresh"));
    vector.push(String::from("Ayare"));

    println!("{:?}",vector);

    //Inserting data on a particular index
    println!("\n INSERTING DATA ON A PARTICULAR INDEX-");
    //vector_name.insert(index_no , data);
    vector.insert(1,String::from("OP"));
    println!("{:?}",vector);        // ["Darshan", "OP", "Naresh", "Ayare"] , the data will get right-shipt from the provided index

    //Getting data at a particular index
    println!("\n GETTING DATA AT A PARTICULAR INDEX-");
    //vector_name.get(index_no);
    // println!("{}",vector.get(1));   //Error will occur bcoz String cannot be formatted with default formatter i.e {}
    println!("{:?}",vector.get(2));     //Some("Naresh")
    println!("{:?}",vector.get(6));     //None , bcoz their is not data at index 6
}