use std::sync::mpsc;
use std::thread;
fn main() {
    // Do not communicate by sharing memory; instead, share memory by communicating.
    // The ownership system enforces memory safety, but it does not enforce thread safety.
    // To ensure thread safety, Rust does not allow data to be shared between threads by default

    //Channels are used to send messages between threads. 
    //The channel has two halves: the transmitter and the receiver. The transmitter is used to send messages, and the receiver is used to receive messages. 
    //The transmitter can be cloned to allow multiple producers to send messages to the same channel.

    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();                      
    });

    let reciever = rx.recv().unwrap();
    println!("Got: {}", reciever);


    let (txx,rxx) = mpsc::channel();
    for i in 0..10{
        let producer = txx.clone();
        thread::spawn(move || {
            let mut sum:u64 = 0;
            for j in i*10000000..(i+1)*10000000{
                sum+=j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(txx); // close the channel so that the receiver will know when to stop waiting for messages

    let mut total:u64 = 0;
    for val in rxx{
        println!("recieving values from thread {}", val);
        total+=val;
    }  
    println!("Total: {}", total);
}