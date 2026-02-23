use std::collections::HashMap;
fn main() {
    //HashMap stores a key value pair in rust.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);  

    let color = scores.get("red");
    println!("{:?}", color); //always returns an option type because the key may not exist in the hash map.

    match color {
        Some(score) => println!("The score for Blue is: {}", score),
        None => println!("Blue team not found."),
    }
    
    let input_vc = vec![(String::from("diya"),10),(String::from("sneha"),20),(String::from("sneha"),23)];
    let hm = group_values_by_key(input_vc);
    println!("{:?}", hm);
}


fn group_values_by_key(vec : Vec<(String,i32)>)->HashMap<String,i32>{       
    let mut hm = HashMap::new();
    for (key,value) in vec{
        hm.insert(key,value);
    }
    return hm;
}