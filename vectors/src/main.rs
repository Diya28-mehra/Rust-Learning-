fn main() {
    let mut vac = Vec::new();
    vac.push(1);
    vac.push(2);
    //vac.push(String::from("Hello")); // rust is not a dynamically typed language, so this will cause a compile-time error
    vac.push(3);
    println!("vac: {:?}", vac);
    let vac2 = even_filter(&vac);
    println!("vac2: {:?}", vac2);
    println!("vac: {:?}", vac);


    let alphabets= vec!['a', 'b', 'c', 'd', 'e'];
    println!("alphabets: {:?}", alphabets);
}


fn even_filter(v: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in v{
        if i%2==0{
            new_vec.push(*i);
        }
    }
    return new_vec;
}