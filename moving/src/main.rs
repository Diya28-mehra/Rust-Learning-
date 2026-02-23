fn main() {
    //call the function 
    let a = String::from("hello");

    // only one owner at a time
    // when we assign a to b, we are moving the ownership of the value from a to b
    // a is no longer valid and we cannot use it anymore
    // let b = a;
    // println!("a: {}, b: {}", a, b); //we will get an value borrowed error because a has been moved to b
    // we cant use a anymore because it has been moved to b
    // reference of a is now pointing to null because it has been moved to b


    let b = a.clone(); // we can use clone to create a copy of the value and avoid the move
    println!("a: {}, b: {}", a, b); // now we can use both a and b because they are different values in memory



    let a = String::from("Hii Mutko");
    //do_something(a); // we are moving the ownership of a to the function do_something
    //println!("a: {}", a); // we will get an value borrowed error because a
    do_something(a.clone()); // we can use clone to create a copy of the value and avoid the move
    println!("a: {}",a); 



    //one more method we can return ownership from a function
    let mut a = String::from("Hello Poppy");
    a = do_something(a); // we are moving the ownership of a to the function do_something_and_return and then returning it back to a
    println!("a: {}", a); // now we can use a because it has been returned back to us
} 

fn do_something(s: String)->String {
    println!("s: {}", s);
    return s;
}

