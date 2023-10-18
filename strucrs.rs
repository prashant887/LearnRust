struct Shuttle {
    name:String,
    crew:u8,
    fuel:f64
}

//methods in struct

impl Shuttle {
    fn get_name(&self) -> &str{
        &self.name
    }

    fn add_fuel(&mut self,gallon:f64) {
        self.fuel+=gallon

    }
    //Assoicated Function 
    fn new(name:&str)->Shuttle{
        Shuttle {
            name:String::from(name),
            crew:7,
            fuel:30.32
        }
    }

}


struct Color(u8,u8,u8);
struct Point(u8,u8,u8);

fn get_y(p:Point)->u8 {
    return p.1;
}


fn main(){
    let vechile=Shuttle {
        name:String::from("Endavor"),
        crew:7,
        fuel:45.34
    };

    println!("Name {}  {} {}",vechile.name,vechile.crew,vechile.fuel);

    let mut another_vechile=Shuttle {
        name:String::from("Vaouger"),
        ..vechile

    };

    println!("Name {}  {} {}",another_vechile.name,another_vechile.crew,another_vechile.fuel);


    let vechacle_name=vechile.get_name();

    println!(" Name {} ",vechacle_name);

    another_vechile.add_fuel(102.34);

    println!(" New Fuel {} ",another_vechile.fuel);

    //assocated function
    let assoc_vec= Shuttle::new("Lica");

    println!("Name {}  {} {}",assoc_vec.name,assoc_vec.crew,assoc_vec.fuel);

    let red=Color(0,10,22);

    println!("Color {}",red.0);

    let chord:Point=Point(2,3,4);
    let y:u8=get_y(chord);

    println!("Chord Y {}",y);



}