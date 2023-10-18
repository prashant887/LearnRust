fn main(){
    let mut astronauts:Vec<String> = Vec::new();
    astronauts.push(String::from("Ankit"));
    astronauts.push(String::from("Bobby"));
    astronauts.push(String::from("Juan"));

    println!(" astronauts {:?}",astronauts);

    let last=astronauts.pop();

    println!(" last {:?}",last);

    let third=astronauts.get(2);

    println!(" last {:?}",third);

    let countdown=vec![5,4,3,2,1];

    println!(" astronauts {:?}",countdown);


}