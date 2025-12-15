fn main(){
    let mut str = String :: from("Darshan");

    // let mb_str1 = &mut str;
    // let mb_str2 = &mut str;

    // mb_str1.push_str(" Ayare");     //Error will occur because their can be multiple R&W(muttable) owner


    let mb_str1 = &mut str;
    let ib_str2 = &str;

    mb_str1.push_str(" Ayare");     //Error will occur because their cant be R owners if their is any R&W owner


    //Errors / Warnings will occur according to the Ownership rules of the RUST
}