use std::sync::{Arc,Mutex};
fn main(){
    let m=Arc::new(Mutex::new(10));
    let clone =m.clone();
    std::thread::spawn(move||{
        let mut lock_1 = clone.lock().unwrap();
        *lock_1+=1;
        //drop(lock_1);
        let mut lock_2 = clone.lock().unwrap();
        *lock_2+=1;
    }).join().unwrap();
    println!("{:?}",m.lock().unwrap());
}
