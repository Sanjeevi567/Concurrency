use std::thread::{scope,spawn};
fn main(){
//without initializing we can't use it anywhere
    let mut x;
    scope(|s|{
       s.spawn(||{
           x=10;
          
       }); 
    });
    spawn(move||{
        x=10;
        println!("{x}");
    });
}
