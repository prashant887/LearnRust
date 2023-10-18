use std::collections::HashMap;

fn main(){
    let mut missions=HashMap::new();

    missions.insert("Sun",1);
    missions.insert("Moon",2);
    missions.insert("Mars",3);

    missions.entry("Moon").or_insert(4);

    println!(" missions {:?}",missions);

    missions.entry("venus").or_insert(5);

    println!(" missions {:?}",missions);


    let moon_mission=missions.get("Moon");

    println!(" moon_mission {:?}",moon_mission);


}