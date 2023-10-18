struct Rectangle<T,U>{
    width:T,
    height:U

}

struct Square<T>{
    side:T
}

impl<T,U> Rectangle<T,U>{
    fn get_width(&self) -> &T {
        &self.width
    }
}

//This takes only those datatypes that can be compared like number , float
fn get_biggest<T:PartialOrd>(a:T,b:T)->T{
    if a>b {
        return a;
    }
    else {
        return b;
    }
}

fn main(){
    let rect=Rectangle{
        width:1u8,
        height:3u16
    };

    println!(" Rect {} {}",rect.width,rect.height);

    let sq=Square{
        side:3.2f32
    };

    println!(" Square {:?}",sq.side);

    let rect=Rectangle{
        width:1.1f64,
        height:3.3f32
    };

    println!(" Rect {} {}",rect.width,rect.height);

    let widht=rect.get_width();

    println!("Widht {}",widht);

    println!("Biggest is  {}",get_biggest(32,56));

//Box Data Type 

}