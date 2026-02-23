fn main() {
    //In Rust, iterators are lazy, which means they do not do anything
    // until you call a method that consumes the iterator, such as `collect`, `sum`, or `for_each`.
    // This allows you to chain multiple iterator methods together without creating intermediate collections.

    let mut numbers = vec![1, 2, 3, 4, 5];
    let iter = numbers.iter();
    //explicitly we are seeing for i in loop uses iterator under the hood
    for val in iter{
        print!("{} ", val);
    }
    print!("\n{:?}", numbers);
    //the iterators provide a method by borrowing the values.

    //using mutable iterators
    let mut_iter = numbers.iter_mut();
    for val in mut_iter{
        *val *= 2; // we can modify the values in place using iter_mut  
    }
    println!("\n{:?}", numbers);


    //using next iterator 
    let nums = vec![10, 20, 30];
    let mut itter = nums.iter();
    while let Some(val) = itter.next(){
        print!("{} ", val);
    }
    println!();

    //using IntoIter 
    // it converts to a iterator which takes the ownership of the values, so we can no longer use the original vector after this.
    let into_iter = numbers.into_iter();
    for val in into_iter{
        print!("{} ", val);     
    }
    //println!("{:?}", numbers); //we can't print this as the ownership is moved 



    //Some methos come with iterators - consuming adapter and iterator adaptors
    //Consuming adapter: methods that consume the iterator and produce a final value, such as `sum`, `product`, `count`, etc.

    let v = vec![1,2,3,4,5];
    let v1_iter = v.iter();
    let sum:i32= v1_iter.sum();

    // for i in v1_iter {
    //     println!("{}", i); // this will not print anything as the iterator is already consumed by sum() method
    // }
    //Now we can't consume it again as the ownership is moved to sum() method, so we can't use it again.
    println!("Sum of vector elements: {}", sum);


    //Iterator adaptors: methods that take an iterator and produce a new iterator, such as `map`, `filter`, `take`, etc.
    let v2 = vec![1,2,3,4,5];
    let v2_iter = v2.iter();
    let v2_iter2 = v2_iter.map(|x| x * x);       
    for x in v2_iter2 {
        print!("{} ", x); 
    }
    println!();
    let v2_iter3 = v2.iter().filter(|&&x| x % 2 == 0); // filter even numbers
    for x in v2_iter3 {
        print!("{} ", x);   
    }

    println!("{:?}",v2);
}
