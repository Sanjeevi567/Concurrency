use std::sync::{Arc,Mutex};
fn main(){
    let m=Arc::new(Mutex::new(vec![1,2,3,4,5]));
    let clone =m.clone();
    let m1 = std::thread::spawn(move||{
     
     
         *clone.lock().unwrap().push(45);
         
         //The solution
         //let mut local = clone.lock().unwrap();
         //local.push(45); //changes reflected in mutex inner type
        
       
        
}).join().unwrap();
println!("{:?}",m);
}
