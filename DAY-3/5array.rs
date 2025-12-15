fn main(){
    let arr  = [10,20,30];
    println!("{}",arr[0]);

    println!("\n for loop for array \n");
    // for loop for array
    for element in arr{
        println!("{}",element);
    }

    println!("\n for loop for range \n");

    // for loop for range
    for num in 1..5{
        println!("{}",num);     //print numbers from 1 to 4
    }

    println!("\n");

    for num in 1..=5{
        println!("{}",num);     //print numbers from 1 to 5
    }

    println!("\n");

    for num in (1..=5).rev(){   //.rev reverse the range
        println!("{}",num);     //print numbers from 1 to 5 in reverse order
    }


    //if, else if , else
    println!("\n if, else if , else \n");  
    let age = 10;
    if age >= 18{
        println!("Elligible for voting");
    }else if age == 15{
        println!("You are hero");
    }else{
        println!("Not eligible for voting");
    }

    println!("\n LOOP \n");

    //loop 
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{}",counter);

        if counter == 20{
            break counter * 2;      //returns the value to result -> result = counter * 2
        }
    };

    println!("The result of the loop is {}",result);

    println!("\n TUPLE \n");

    let tuple1 = (10,20,"Hello");
    println!("{:?}",tuple1);
    println!("{}",tuple1.0)
}