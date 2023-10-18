use std::fs;
use std::io;

fn read_and_combine(f1: &str,f2: &str) -> Result<String,io::Error> {
    let mut s1=fs::read_to_string(f1)?; //Sends error
    let s2=match fs::read_to_string(f2){
        Ok(s) =>s,
        Err(e)=>return Err(e)
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)

}

fn main(){
let result=fs::read_to_string("some_file");

let contents=match result {
    Ok(message) => message,
    //Err(error)=> String::from("No FIle Found")
    Err(error) => match error.kind() {
        io::ErrorKind::NotFound=>String::from("File Not Found"),
        io::ErrorKind::PermissionDenied=>String::from("Permission Denied"),
        _ => panic!("Another Error {:?}",error)

    }
};

println!("conents {:?}",contents);

let result=read_and_combine("one.txt","two.txt");

match result {
    Ok(message) => message,
    //Err(error)=> String::from("No FIle Found")
    Err(error) => match error.kind() {
        io::ErrorKind::NotFound=>String::from("File Not Found"),
        io::ErrorKind::PermissionDenied=>String::from("Permission Denied"),
        _ => panic!("Another Error {:?}",error)

    }
};

}