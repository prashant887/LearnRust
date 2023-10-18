use std::env;

fn main(){

    for (index,value) in env::args().enumerate() {
        println!("Index {}  Value {}",index,value);
    }

}