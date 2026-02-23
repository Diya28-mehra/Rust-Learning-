struct User <'a>{
    name: &'a str,
}

struct Friend<'a, 'b> {
    f_name: &'a str,
    l_name: &'b str,
}
fn main() {
    // let name = String::from("Alice");
    // let user = User { name: &name };
    // println!("User name: {}", user.name); 
    // the reference of user is valid as long as name is in scope


    let user;
    {
        let name = String::from("Alice");
        user = User { name: &name };
        println!("User name: {}", user.name); 
    }
    // println!("User name: {}", user.name);
    // the reference of user is invalid because name goes out of scope

    //why do we need structs with references to have a lifetime parameter?
    // because the struct contains a reference, we need to specify how long that reference is valid

    let friend: Friend;
    let f_name = String::from("Alice");
    {
        let l_name = String::from("Smith");
        friend = Friend { f_name: &f_name, l_name: &l_name };
        println!("Friend name: {} {}", friend.f_name, friend.l_name);
    }
    // println!("Friend name: {} {}", friend.f_name, friend.l_name);
    // the references in friend are invalid because f_name and l_name go out of scope

}