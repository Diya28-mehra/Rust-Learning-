fn main() {
    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Current length: {} capacity: {} pointer: {:?}", s.len(), s.capacity(), s.as_ptr());

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {} capacity: {} pointer: {:?}", s.len(), s.capacity(), s.as_ptr());


    // for i in 1..100{
    //     println!("Current length: {} capacity: {} pointer: {:?}", s.len(), s.capacity(), s.as_ptr());
    //     // Append some text to the string
    //     s.push_str(" and some additional text");
    //     println!("After update: {} capacity: {} pointer: {:?}", s.len(), s.capacity(), s.as_ptr());

    // }
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}", s1); // This line would cause a compile error because ownership has been moved.
    println!("{}", s2);


    let my_string = String::from("hello");
    takes_ownership(my_string);
   // println!("{}", my_string); // This line would cause a compile error because ownership has been moved.

    let str = String::from("hii");
    takes_ownership(str.clone());
    println!("{}", str); // This works because we cloned the string(we passed a copy not the same refercne).


    let  num:i64 = 5;
    findsquare(num);
    println!("{}", num); // This works because integers implement the Copy trait.


    let mut icecream = String::from("vanilla");
    icecream = give_ownership_back(icecream);
    println!("{}",icecream);



    //References and Borrowing
    let num = 10;
    borrow_variable(&num); // We are passing reference of num, not ownership.
    println!("{}", num); // This works because we only borrowed num, not took ownership.

    let myrust = String::from("Hello, Rust!");
    let r1 = &myrust; // First reference
    let r2 = &myrust; // Second reference
    println!("{} and {}", r1, r2); // Both references can be used here


    //mutable references at max 1 
    let mut s = String::from("It gives nested references");
    let ss = &mut s; // mutable reference
    //  let ss2 = &mut s; // This line would cause a compile error
    // let ss2 = &s; // This line would also cause a compile error
    //we cant have mutable and immutable references at the same time if we have a mutable reference already 
    //println!("{}",s); A mutable borrow blocks all other access to the value,even read-only access.
    println!("{}",ss); 

    // here we are passing mutable reference
    takes_reference(&mut s);
    println!("{}",s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

fn findsquare(mut some_integer: i64) {
    some_integer = some_integer * some_integer;
    println!("Square is {}", some_integer);
}
fn give_ownership_back(s: String) -> String {
   return s // Return the string, transferring ownership back to the caller
}   
 
fn borrow_variable(s: &i32) {
    println!("Borrowed variable: {}", s);
}

fn takes_reference(s: &mut String) {
    s.push_str(" trying to change");
}

