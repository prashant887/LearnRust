fn main() {
    let make_x_odd=false;

    let x=if make_x_odd {1} else {2};

    println!("Value Of X {}",x);

    let mut count=0;

    let lp=loop {
        if count==10 {
            break count*10;
        }
        count+=1;
    };

    println!("Count {}",lp);

    //hile loop cant return value

    //For loop executes iterator , it has next method
    let message=['h','e','l','l','o'];

    for w in message {
        println!(" {} ",w);
    }

    for (idx,&elem) in message.iter().enumerate() {
        println!(" {} {}",idx,elem);
        if elem=='l' {
            println!("Special");
        }
    }


    for number in 0..5 {
        println!(" {} ",number);
    }
}