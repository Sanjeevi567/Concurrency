use std::sync::mpsc::channel;
use std::thread::spawn;
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];
    let (sender, rec) = channel();
    //Here the data is first moved to spawn thread closure.
    //It's enforced by the compiler.
    spawn(move || {
        //here the data again moved to the channel thread which only available at the receiver.
        sender.send(data).unwrap();
        //we can't even use data in same threas onces above code is used.
        println!("{:?}",data);
    })
    .join()
    .unwrap();
    //Because of the move semantics the data moved to above thread
    //So we are not allowed to use here.
    println!("{:?}",data);
}
