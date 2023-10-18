use std::io;

fn main(){
let mut buffer=String::new();
println!(" Enter Message");
let _ = io::stdin().read_line(&mut buffer);
println!("  Message {} ",buffer);

let number:i32=buffer.trim().parse().unwrap() ;
println!("  Number {} ",number+1);

}