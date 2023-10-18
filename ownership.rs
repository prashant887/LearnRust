fn main() {
    let planet="Earth"; //Global Scope
    if true {
        println!(" Plannet {}",planet);
    }
    println!(" Plannet {}",planet);

    //Shadowing 
    let shadow="mars";
    {
        println!(" Shadowing {}",shadow);
        let shadow=4;
        println!(" Shadowing {}",shadow);
    
    }
    println!(" Shadowing {}",shadow);

    //Given Data value is owned by one and only owner at given time . when owning variable goes out of scope value is dropped

    let outer_planet:String;

    {
        let inner_plannet=String::from("Mercury");
        println!(" inner_plannet {}",inner_plannet);
        //Ownership of Mercury is moved from inner_plannet to outer_planet
        //outer_planet=inner_plannet; //Shallow copy

        
                outer_planet=inner_plannet.clone(); //Deep copy
                println!(" inner_plannet {}",inner_plannet); //Since Ownership is moved inner_plannet is no more valoid

         


    }
    println!(" outer_planet {}",outer_planet);

    /*
    Rust Behaves diffent with Integers
    Ineger values is always on stack and not on heap 
    String data is on heap with pointer in stack , Since String value space is unknown
     */

     let outer_planet_int:i32;

     {
        let mut inner_planet_int=1;
        println!(" outer_planet_int {}",inner_planet_int);
        outer_planet_int=inner_planet_int;
        inner_planet_int+=1;

        println!(" outer_planet_int {}",inner_planet_int);


    }
    println!(" outer_planet_int {}",outer_planet_int);


    let orig_int:i32=1;
    modify_int(orig_int);
    println!(" Oring Value {}",orig_int);

    let orig_str:String=String::from("OrigStr");

    //When function is called ownership is transferred to function and original memory location is cleared for other use

    modify_str(orig_str.clone());


    println!("Orig String {}",orig_str);

    println!( " Rtn Sttring {} ",modify_str_rtn(orig_str));



}

fn modify_int(mut int:i32){
    int+=1;
    println!("In Modify Int : {}",int);
}

fn modify_str(str:String){
    println!("Modified String {}",str);
}

fn modify_str_rtn(str:String) ->String{
    return str;
}


/*
Move error

 let inner_plannet=String::from("Mercury");
   |             ------------- move occurs because `inner_plannet` has type `String`, which does not implement the `Copy` trait
...
26 |         outer_planet=inner_plannet;
   |                      ------------- value moved here
27 |         println!(" inner_plannet {}",inner_plannet);
   |                                      ^^^^^^^^^^^^^ value borrowed here after move
*/