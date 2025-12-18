enum Message{
    Quit,
    Text(String),       //Tuple
    Move{x:i32 , y:i32} //Structure
}

fn matching(mssg : &Message){
    match mssg{
        Message::Quit => println!("QUIT"),
        Message::Text(content) => println!("Message received - {}",content),
        Message::Move{x,y} => println!("Move ({},{})",x,y)
    }
}

enum State{
    Connected ,
    Disconnected ,
    Connecting
}

fn main(){
    let mssg1 = Message::Text(String::from("No Message"));
    let mssg2 = Message::Move{x:10,y:20};

    matching(&mssg1);

    let state = State::Connected;
    match state{
        State::Connected => println!("Connected"),
        _ => {}     //this will handle the remainig state
    }
}