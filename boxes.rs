use std::mem;
//use std::ops::Add;

struct Shuttle{
    name: String,
    crew: u8,
    fuel: f64
}

fn sum_boxes<T:std::ops::Add<Output=T>>(a:Box<T>,b:Box<T>)->Box<T>{
    Box::new(*a+*b)
}

fn main() {
    let vechile=Shuttle {
        name:String::from("Atlantis"),
        crew:7,
        fuel: 30.2
    };

    println!("Vechile Size on Stack : {} Bytes",mem::size_of_val(&vechile));

    let boxed_vechicle:Box<Shuttle> = Box::new(vechile);
    println!("Vechile Size on Stack : {} Bytes",mem::size_of_val(&boxed_vechicle));
    println!("Vechile Size on Heap : {} Bytes",mem::size_of_val(&*boxed_vechicle));

    let one = Box::new(1);
    let two=Box::new(2);

    println!("SUm {}",*sum_boxes(one,two));

    let fone = Box::new(2.32);
    let ftwo = Box::new(43.2);

    println!("SUm {}",*sum_boxes(fone,ftwo));




}