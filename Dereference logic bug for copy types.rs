use std::sync::{Arc,Mutex};
fn main(){
    let m=Arc::new(Mutex::new([1,2,3,4,5]));
    let clone =m.clone();
    let m1 = std::thread::spawn(move||{
     
     //Trying to move the data to the caller.
     //For move types this is an error.
     //But for copy types this is problem and then we have a logic bug
       *clone.lock().unwrap()
        
    //solution.
    //The paranthese are important ,otherwise confusing compile error shows up.
   //(*clone.lock().unwrap())[0]=12;
        
        
}).join().unwrap();
println!("{:?}",m.lock().unwrap());
//This is independent of the mutex inner type
println!("{:?}",m1);
}
