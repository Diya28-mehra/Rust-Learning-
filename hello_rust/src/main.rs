fn main() {
    let mut x:i32 = -11;
    for _i in 0..1000{
        x+=1;
    }
    let y:u32 = 10;
    let f:f64 = -3.14;   
    println!("Hello, world! x = {}, y = {}, f = {}", x, y, f);
    let male = true;
    let _female = false;
    if male{
        println!("You are a man");
    }
    let greeting = String::from("Hello worlds");
    let char = greeting.chars().nth(1);
    match char{
        Some(c)=> println!("The first character is {}",c),
        None=> println!("The string is empty"),
    }
    println!("{}",char.unwrap());
    for i in 1..10{
        print!("{} ",i);
    }

    let sentence = String::from("This is a sample sentence for testing.");
    let words = get_firstword(sentence);
    println!("\nFirst word: {}", words);
}

fn get_firstword(sentence: String) -> String {
    let mut ans = String::new();
    for char in sentence.chars(){
        println!("{}", char);
        ans.push_str(char.to_string().as_str());
        if char==' '{
             break;
        }
    }
    return ans;
}

