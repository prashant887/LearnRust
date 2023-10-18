fn main(){
    let message=String::from("Earth");

    println!(" Default {}",message);

    let mut mutableMessage=String::from("Earth");

    println!(" Orig {}",mutableMessage);

    mutableMessage.push_str(" Is Large");


    println!(" Mutated {}",mutableMessage);



}