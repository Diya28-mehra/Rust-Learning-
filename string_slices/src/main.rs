fn main() {
    //String let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    //This is useful when you want to work with a part of a string without having to create a new string or copy the data.

    let mut s = String::from("Hi, My name is Diya Mehra!");
    s.push_str(" I am learning Rust."); 
    println!("{}", s);
    s.replace_range(0..2, "Hello"); 
    println!("{}", s);


    let mut name = String::from("Diya Mehra");
    //let ans = first_word(name);
    //println!("First word: {}", ans);
    println!("First word using better method: {}",first_word_better(&name));
}

fn first_word_better(str:&String)->&str{
    let mut space_index = 0;
    for (i,c) in str.char_indices(){
        if c==' '{
            space_index = i;
            break;
        }
    }
    return &str[0..space_index];    
}

fn first_word(str:String)->String{
    let mut ans = String::from("");
    for c in str.chars(){
        if c==' '{
            break;
        }
        ans.push_str(&c.to_string());
    }
    return ans;
}
