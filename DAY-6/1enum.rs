enum State{
    Connected ,
    Disconnected ,
    Connecting
}

enum TrafficLight{
    Red,
    Green,
    Yellow
}

fn main(){
    println!("Server status -- ");
    let mut connectionStatus = State::Connected;

    match connectionStatus {
        State::Connecting => println!("CONNECTING"),
        State::Connected => println!("CONNECTED"),
        State::Disconnected => println!("DISCONNECTED"),
    }
    

    println!("\nTraffic Signal");
    let TrafficLight = TrafficLight::Red;

    match TrafficLight {
        TrafficLight::Red => println!("STOP"),
        TrafficLight::Green => println!("GO"),
        TrafficLight::Yellow => println!("SLOW DOWN")
    }

    //return something for specifc value of enum
    let mssg = match TrafficLight {
        TrafficLight::Red => String::from("STOP"),
        TrafficLight::Green => String::from("GO"),
        TrafficLight::Yellow => String::from("SLOW DOWN")
    };

    println!("{}",mssg);
}