use std::sync::{Arc,RwLock};
use std::thread::spawn;
fn main(){ 
  let vec = Arc::new(RwLock::new(Vec::new()));
  let vec1=vec.clone();
  spawn(move||{
    let  mut write = vec1.write().unwrap();
    write.push(12);
    println!("{}",write.len());
  }).join().unwrap();
  let vec1=vec.clone();
  spawn(move||{
    let read = vec1.read().unwrap();
    //mutator method not allowed
    read.push(12);
//But multiple read methods allowed
    println!("{} {} ",read.capacity(),read.len());
  }).join().unwrap();
  println!("{:?}",vec);
}
