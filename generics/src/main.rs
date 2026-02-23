fn main() {
    let bigger = largest(1,2);
    let bigger_char = largest('a', 'b');
    println!("The bigger number is {}", bigger);
    println!("The bigger char is {}", bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a:T,b:T)->T{
    if a>b {
        a
    }
    else {
        b
    }
}


// fn largest_32(a:i32,b:i32)->i32{
//     if (a>b){
//         a
//     }
//     else {
//         b
//     }
// }

// fn largest_char(a:char,b:char)->char{
//     if (a>b){
//         a
//     }
//     else {
//         b
//     }
// }


