fn main() {
    //Arrays have fixed lenght and same datatype , cant change length dynamically 
    let letters=['a','b','c']; //non mutable

    let mut letters_mutable=['a','b','c']; // mutable
    letters_mutable[1]='x';

    let numbers:[i32;5];
    numbers=[0;5];

    let index:usize=numbers.len();

    println!(" Index {}",index);

    //Tuple can have mixed datatype

    let mut stuff:(u8,f32,char)=(10,32.10,'x');
    println!(" {} ",stuff.2);
    stuff.0+=3;

    let(o,tw,th)=stuff;

    println!(" {} ",o);


}