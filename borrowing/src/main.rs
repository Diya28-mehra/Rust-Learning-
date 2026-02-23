fn main() {
    let mut s1 = String::from("Hey Baby");
    // s1 = do_something(s1); // we are moving the ownership of s1 to the function do_something and then returning it back to s1
    // println!("s1: {}", s1); // now we can use s1 because it has been returned back to us

    //Another way to not to pass ownership and retaining it is by using references
    do_something(&s1); // we are passing a reference of s1 to the function do_something, so we are not moving the ownership of s1
    println!("s1: {}", s1); // now we can use s1 because we are not moving the ownership of s1
    


    // What if I want to borrow a value and modify it? We can use mutable references for that
    let mut s2 = String::from("Hello ");
    do_something_mut(&mut s2); 
    println!("s2: {}", s2); // now we can use s2 because we are not moving the ownership of s2



    //Rules of References:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references, but not both.
    // 2. References must always be valid.

    let mut a = String::from("Pops Pops");
    let r1 = &a; // immutable reference
    let r2 = &a; // another immutable reference
    // let r3 = &mut a; // error: cannot borrow `a` as mutable because it is also borrowed as immutable
    println!("r1: {}, r2: {}", r1, r2); // we can use r1 and r2 because they are immutable references


    let mut b = String::from("Poti Poti");
    let r3 = &mut b; // mutable reference
    // let r4 = &b; // error: cannot borrow `b` as immutable because it is also borrowed as mutable
    println!("r3: {}", r3); // we can use r3 because it is a mutable reference

}

fn do_something(s: &String) {
    println!("s: {}", s);  // s is a reference to a String value
    // we are not moving ownership of s, so s remains valid after the function call
}


fn do_something_mut(s: &mut String) {
    s.push_str("Nannu"); // we are modifying the value of s, but we are not moving ownership of s, so s remains valid after the function call
}
