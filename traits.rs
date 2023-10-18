use std::any;
use std::fmt;

#[derive(PartialEq,PartialOrd)]

struct Satallite{
    name: String,
    velocity: f64
}

struct SpaceStatation{
    name:String,
    crew:u8,
    altitude:u32
}

trait Description{
   // fn description(&self) -> String;

   fn description(&self) -> String {
    //Default Desc
    String::from("Default This is Object Flying Throug Space")
   }
}

impl Description for Satallite{
    /* 
    Uses Default desc
    fn description(&self) -> String {
        format!("the {} Flying at {} miles per second",self.name,self.velocity)
    }
    */
    

}

impl Description for SpaceStatation{
    fn description(&self) -> String {
        format!("the {} Flying at {} miles per second with {} people",self.name,self.altitude,self.crew)
    }

}

fn print_type<T:fmt::Debug>(item:T){
    println!("{:?} is {}",item,any::type_name::<T>());
}

//fn comapre_and_print<T:fmt::Display+PartialEq+From<U>,U:fmt::Display+PartialEq+Copy>(a:T,b:U)
fn comapre_and_print<T,U>(a:T,b:U)
where T:fmt::Display+PartialEq+From<U>,
U:fmt::Display+PartialEq+Copy
{
    if a==T::from(b){
        println!(" {} is equal to {}",a,b)
    }
    else {
        println!(" {} is not equal to {}",a,b)

    }
}

//Display Trait
fn get_displable()->impl fmt::Display{
    "thrteen"
}

impl fmt::Display for Satallite{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{} Flying at {} miles per Hour",self.name,self.velocity)
    }
}

fn main(){
    let hubble=Satallite{
        name:String::from("Hubble Station"),
        velocity:4.72
    };

    let iss=SpaceStatation{
        name:String::from("Internatonal Space Station"),
        crew: 6,
        altitude:30

    };

    let gps=Satallite{
        name:String::from("GPS Pos"),
        velocity:2.32
    };

    println!(" Hubble {}",hubble.description());
    println!(" ISS {}",iss.description());

    println!(" Hubble == GPS is {}",hubble==gps);
    println!(" Hubble > GPS is {}",hubble>gps);


    //Trait Bonds here extends functions to support ops
    print_type(13);

    print_type(13.0);

    print_type("threteen");

    print_type([13]);

    //Mutliple Trait Bonds

comapre_and_print(1.0,1);
comapre_and_print(1.1,1);


println!("Output {}",get_displable());

println!(" Hubble {} ",hubble);

}