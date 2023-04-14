use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::spawn,
};
fn main() {
    let data = [('a', 1), ('b', 2), ('c', 3), ('d', 4), ('e', 5)];
    //This shared data live in main thread
    let shared_data = Arc::new(Mutex::new(HashMap::from(data)));
    //initial value of hashmap
    println!("{:?}", shared_data);

    //Increase the ref count  to share with this spawn.
    let clone = shared_data.clone();
    spawn(move || {
        let mut hashmap = clone.lock().unwrap();
        hashmap.insert('f', 6);
    })
    .join()
    .unwrap();

    //Increase the ref count again to share with this spawn.
    let clone = shared_data.clone();
    spawn(move || {
        let mut hashmap = clone.lock().unwrap();
        hashmap.insert('g', 7);
    })
    .join()
    .unwrap();

    println!("{:?}", shared_data.lock().unwrap());
}
