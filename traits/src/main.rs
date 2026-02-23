pub trait Summary{
    // fn summarize(&self)-> String;
    fn summarize(&self)-> String{
        return String::from("Read more...");
    }
}

struct User{
    name: String,
    age: u32,
}

// impl Summary for User{
//     fn summarize(&self)->String{
//         return format!("User {} is {} years old", self.name,self.age);
//     }
// }
struct Fix;
impl Summary for Fix{}
impl Summary for User{}
impl Summary for String{}
fn main() {
    //Traits are a way to define shared behavior in Rust.
    //They allow you to specify a set of methods that a type must implement in order to be considered as implementing that trait.
    // This is similar to interfaces in other programming languages.r

    let user = User{
        name: String::from("Alice"),
        age: 30,
    };
    println!("{}", user.summarize());
    notify(user);
    notify(String::from("Hello, world!"));
}

fn notify(u:impl Summary){
    println!("Breaking news! {}", u.summarize());
}