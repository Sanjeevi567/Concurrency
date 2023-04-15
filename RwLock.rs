use std::sync::{Arc, Mutex, RwLock};
use std::thread::spawn;
fn main() {
    let vec = Arc::new(RwLock::new(Vec::new()));
    let vec1 = vec.clone();
    spawn(move || {
        let mut write = vec1.write().unwrap();
        //Both read and write access are permitted but can't move out ownership.
        write.push(12);
    })
    .join()
    .unwrap();
    let vec1 = vec.clone();
    spawn(move || {
        let read = vec1.read().unwrap();
        //Only allowed to read .Can't mutate and move out the ownership.
        //mutator method
        read.clear();
        println!("{} {} ", read.capacity(), read.len());
    })
    .join()
    .unwrap();
    //This will not panic since readlock don't allow the mutator.
    println!("{:?}", vec.read().unwrap().get(0));
}
