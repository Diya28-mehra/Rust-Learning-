fn main() {
    //Takes lot of time to understand the concept of lifetimes in Rust, but it is very important to understand it to write safe and efficient code. 
    //Lifetimes are a way to specify how long a reference should be valid for.
    // They help prevent dangling references and ensure that references do not outlive the data they point to.


    let longest_str;
    let str1 = String::from("Hello");
    let str2 = String::from("smaller");
    longest_str = longest(str1, str2);
    println!("The longest string is: {}", longest_str);

    //Lets change the syntax 
    let lstr;
    let s1 = String::from("papapapapa");
    {
        let s2 = String::from("smaller");
        lstr = longest(s1, s2);
    }
    println!("The longest string is: {}", lstr);

    //Nothing related to lifetimes, but now lets change the function signature a bit
    // let lls;
    // let ss1 = String::from("adbjahsbfhafhfihuehreg");
    // {
    //     let ss2 = String::from("a");
    //     lls = longest_string(ss1.as_str(), ss2.as_str());
    // }
    // println!("The longest string is: {}", lls);
    //The above code will not compile 
    //because the function longest_string returns a reference to a string 
    //that is owned by the function, and the reference is not valid outside the function.
    //Dangling pointer problem handled by lifetimes in Rust. 


    //Lifetimes in Rust are used to ensure that references are valid for as long as they are used.
    //In the function longest_string, we are returning a reference to a string that is owned by the function.
    //The lifetime of the returned reference must be tied to the lifetime of the input references.
    //This is done using lifetime parameters in function signatures.

    let longest_str_ref;
    let str3 = String::from("a");
    {
        let str4 = String::from("aaaaaaaaaaaa");
        longest_str_ref = longest_string_ref(str3.as_str(), str4.as_str());   
        // borrowed value foes not live logn enough  
    }
    println!("The longest string is: {}", longest_str_ref);
    //borrow later used here

}


fn longest_string_ref<'a>(s1: &'a str,s2: &'a str) -> &'a str {
    // It says the life time of the returned reference is the same as the lifetime of the input references.
    // Intersection of the lifetimes of the input references, so the returned reference will be valid as long as both input references are valid.
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// fn longest_string(a: &str, b: &str) -> &str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }


fn longest (a:String,b:String)->String{
    if a.len()>b.len(){
        a
    }else{
        b
    }
}