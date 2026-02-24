use std::thread;
use std::time::Duration;

fn main() {
    //multithreading in rust
    //we can create multiple threads to run concurrently and perform tasks in parallel

    // let handle = thread::spawn(||{
    //     for i in 1..10{
    //         println!("Hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(500));
    //     }
    // });

    // handle.join().unwrap(); //wait for the spawned thread to finish before starting the main thread

    // for i in 1..10{
    //     println!("Hello number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(500));
    // }


    let v = vec![1,2,3,4,5];
    // thread::spawn(||{
    //     print!("Here's a vector: {:?}", v);
    // });
    //it will generate error because By default, closures borrow variables.
    //So It is borrowing v, not owning it.
    //if main function finishes before spawned thread,
    //the spawned thread will try to access v which has been dropped and it will cause a panic


    //to complie this code we need to add the join mmethod
    //otherwise the main thread will finish before the spawned thread 
    //so we move the ownership of v to the spawned thread using move keyword
    thread::spawn(move ||{
        print!("Here's a vector: {:?}", v);
    }).join().unwrap(); //wait for the spawned thread to finish before starting the main thread
    //print!("Value of v: {:?}", v);
    //we cant do this as v is moved to the spawned thread and we cant access it in the main thread anymore



    let x =1;
    {
        let vec = vec![1,2,3];
        thread::spawn(move ||{
            println!("Here's a vector: {:?}", vec);
        });     
        //print!("Value of vec: {:?}", vec);
        //we cant do this as vec is moved to the spawned thread and we cant access it in the main thread anymore
    } 
    println!("Value of x: {}", x);

}
