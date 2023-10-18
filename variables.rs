fn main(){
    let  x=10;
    let mut y=21;
    println!("{} {}",x,y);
    y=25;
    println!("{} {}",x,y);

    let is:i8=-30; //Range  -(2n - 1) to 2n - 1 - 1

    let iu:u8=30; //Range 0 to 2n - 1

    println!("Integers {} {}",is,iu);

    //Float 32 and 64

    let fd = 2.20; // f64

    let fs: f32 = 3.10; // f32

    println!("Floats {} {}",fd,fs);

    let ni:i8=10;

    let di:i8=3;

    println!("Quotent Int= {}",ni/di);

    let nf:f32=10.0;


    let df:f32=3.0;

    println!("Quotent Float= {}",nf/df);


    println!("Quotent Cast= {}",ni as f32/df);

    println!("Quotent Precision= {:.3}",ni as f32/df);
    println!("Quotent Space= {:8.3}",ni as f32/df);
    println!("Quotent Zero Space= {:08.3}",ni as f32/df);


    println!("Quotent of {nu} and {dn} is {q}",dn=df,nu=nf,q=ni as f32/df);

    //Bitwise

    let binval=0b1111_0101u8;

    println!("Value = {0}  Bin={0:08b} Inv = {}",binval);
    println!("Inv = {0:08b}",!binval);



    //Bools
    let tf:bool=true;
    println!("True = {} False = {}",tf,!tf);

    //Char
    let finger='\u{2764}';
    println!(" {} ",finger);

    




}