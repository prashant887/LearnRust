fn main() {
    say_hello();
    print_number(10);
    println!("{} ",adding(3,5));
}


fn say_hello(){
    println!("Hey Hello");
}

fn print_number(num:i32){
    println!("Number is {}",num);
}

fn adding(x:i32,y:i32) -> i32 {
    //x+y without ; 
    return x+y ;//Not adding ; makes it expression and not execute return x+y ; or x+y
}