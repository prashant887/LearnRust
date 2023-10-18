fn main(){
    let rocket_fuel=String::from("RP-1");
let (rocket_fuel,length)=process_fule(rocket_fuel);
    println!("\n Rocket Fule {} {}",rocket_fuel,length);

    let refrence=borrowing(&rocket_fuel);
    println!("\n Refrence Fule {} {}",rocket_fuel,refrence);


    //Single Mutable Reference 
    let mut rocket_fuel_mut=String::from("RP-1");
    let mut_len=mutable_ref(&mut rocket_fuel_mut);
    println!("\n Refrence Mutable  Fule {} {}",rocket_fuel_mut,mut_len);




}

fn process_fule(str:String)->(String,usize) {
    println!("\n Process Fule {}",str);
    let length=str.len();

    return (str,length);

}

fn borrowing(str:&String) -> usize {
    println!("\n Borrowing Fule {}",str);
    return str.len();

}

fn mutable_ref(str:&mut String) -> usize {
    println!("\n Borrowing Fule {}",str);
    str.push_str(" Flamable ");
    return str.len();

}