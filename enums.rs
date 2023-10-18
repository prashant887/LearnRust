enum Shape {
Circle(f64),
Rectangle(f64,f64),
Triangle(f64,f64,f64)
}



impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r)=>r*2.0*std::f64::consts::PI,
            Shape::Rectangle(l,w)=>(2.0*l)+(2.0*w),
            Shape::Triangle(a,b,c)=>a+b+c
            }
    }
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64,f64)
}

impl Location {
    fn display(&self){
        match *self {
Location::Unknown=>println!("Unknwon Location"),
Location::Anonymous=>println!("Anyomyous Location"),
Location::Known(lat,log)=>println!("Place with {} and {}",lat,log)
        }
    }
}
fn main(){
    let my_shape=Shape::Rectangle(20.3,56.2);
match my_shape {
    Shape::Circle(r)=>println!("Circle With Radius {}",r),
    Shape::Rectangle(l,w)=>println!("Rectangle With Length {} Width {}",l,w),
    Shape::Triangle(a,b,c)=>println!("Triangle With 3 sides  {} {} {}",a,b,c)
};

let number=4u8;

let result=match number{
    0 => "zero",
    1 => "one",
    2 => "two",
    _ => {
        println!("{} Dint Match any Number ",number);
        "something else"
    }
};

println!(" Number Result {}",result);


let perimeter=my_shape.get_perimeter();

println!(" Peremeter Result {}",perimeter);

let coutdown=[5,4,3,2,1];

let number=coutdown.get(4);

println!("Number is {:?}",number);

let number=coutdown.get(5);
let number=number.unwrap_or(&0)+1;
println!("Number is {:?}",number);

let number=coutdown.get(4);
let number=match number {
    Some(number) => number+1,
    None => 0
};
println!("Number is {:?}",number);


let my_loc=Location::Anonymous;

println!("Location is {:?}",my_loc.display());


}