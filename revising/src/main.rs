fn main() {
    println!("{}",is_even(45));
    println!("{}",fib(5));
    let mystring = String::from("Hello World");
    println!("{}",getlength(&mystring));
}

fn is_even(num: i32)-> bool{
    if num%2==0 {
        return true;
    }
    return false;
}

fn fib(num : i32)->i32{
    let mut f = 0;
    let mut s = 1;
    if num==0{
        return f;
    }
    if num==1{
        return s;
    }
    for _ in 1..num-1{
        let temp = s;
        s = f+s;
        f = temp;
    }
    return s;
} 

fn getlength(s:&str)-> usize{
    return s.chars().count();
}