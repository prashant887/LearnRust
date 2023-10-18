fn main(){
    let message=String::from("Message From Earth");
    let last_word=&message[13..13+5];
    println!("Last Word is {}",last_word);

    println!("Last Infinte WOrds {}",&message[10..]);

    let planets=[1,2,3,4,5,6,8,9];

    let inner_palnnet: &[i32]=&planets[..4];

    println!("Inner Planaets {:?}",inner_palnnet);

    println!("Trimmed spaces -  {}",trim_spaces(&message[1..]));

    /*
        Slice cant be used as string reference , string refence has pointer length and capacity
        slice will have refence and lenght , slice will not have all charatristics of string so cant be used as string

     */
    let first_word=get_first_word(&message[8..]);
    println!("First Word is {}",first_word);

}
fn get_first_word(s:&str) -> &str {
    let bytes=s.as_bytes();
    for (idx,&item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[..idx];
        }
    }
    &s
}

fn trim_spaces(s:&str)->&str{
    //locate firs not empty space
    let mut start=0;
    for (index,chars) in s.chars().enumerate(){
        if chars != ' '{
            start=index;
            break;
        }
    }
    //search in rev order 
    let mut end=0;
    for (index,chars) in s.chars().rev().enumerate(){
        if chars != ' ' {
            end=s.len()-index;
            break;
        }
    }
&s[start..end]
}