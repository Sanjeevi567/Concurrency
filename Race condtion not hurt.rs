use std::sync::{Arc, Mutex};
use std::thread::spawn;
fn main() {
    let m = Arc::new(Mutex::new(10));
    let clone_1 = m.clone();
    spawn(move || {
        *clone_1.lock().unwrap() += 4;
    })
    .join()
    .unwrap();
    let clone_2 = m.clone();
    spawn(move || {
        *clone_2.lock().unwrap() += 8;
    })
    .join()
    .unwrap();
    println!("{:?}", *m.lock().unwrap());
    assert_eq!(*m.lock().unwrap(), 22);
}
