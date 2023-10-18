use std::fs;

fn main(){

    let containts=fs::read_to_string("planets.txt").unwrap();
    println!("File Containts {}",containts);

    for line in containts.lines() {
        println!(" Planets {}",line);
    }

    let containts=fs::read("planets.txt").unwrap();
    println!("File Containts Bytes {:?}",containts); //Debug Formatter


    let mut write_data=String::new();
    write_data.push_str("This is First Line\n");
    write_data.push_str("This is Second Line\n");

    let _ = fs::write("speech.txt",write_data);


}