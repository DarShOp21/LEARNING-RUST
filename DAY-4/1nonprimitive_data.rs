fn main(){
    println!("\n ARRAY");
    //Array
    let mut arr1 = [10,20,30];             //The array is stored in the stack memory
    arr1[1] = 25;
    println!("{:?}",arr1);

    //Defining an array - arr_name:[datatype ; size_of_array] 
    // let arr2:[i32 ; 6] = [1,2,3];       //This will give an error bcoz the size of the provided array is 3
    let arr2:[i32 ; 6] = [1,2,3,4,5,6];    
    println!("{:?}",arr2);

    //The solution for storing in Heap memory is VECTORS

    println!("\n VECTORS");

    //Defining a vector - vect_name : Vec<i32> = Vec :: new()
    //Defining a vector - vect_name = vec![vect_data]

    let mut arr = vec![10,20,30];
    println!("{:?}",arr);
    println!("Size of the vector is - {}",arr.len());
    println!("Capacity of the vector is - {}",arr.capacity());
    println!("Heap address of the vector is - {:p}",arr.as_ptr());
    println!("Stack address of the vector is - {:p}",&arr);
    arr.push(40);
    println!("{:?}",arr);
    println!("Size of the vector is - {}",arr.len());
    println!("Capacity of the vector is - {}",arr.capacity());
    println!("Heap address of the vector is - {:p}",arr.as_ptr());
    println!("Stack address of the vector is - {:p}",&arr);

    println!("\n FOR LOOP IN VECTORS");
    // for num in arr{
    //     println!("{}",num);
    // }

    // println!("{:?}",arr);     //This will give an error , bcoz of the prev for loop where we have passed the arr to the loop the ownership of the data is transfered 

    //To resolve this , we pass the address of the array i.e the for loop borrows the array and once the loop is executed the ownership is returned

    for num in &arr{
        println!("{}",num);
    }

    for num in &mut arr{
        *num += 2;      //here the num will have the address for the value , hence the star is used to point the value on that address
        println!("{}",num);
    }
}