use std::cell::RefCell;
fn main(){
    //inherent mutability
let mut x = 10; //Note the mut in front of x
let y = &mut x; //We can't mutate the y but we can mutate x through y.
*y+=1;
println!("{}",x);
//Interior mutability
//we can mutate the data inside the refcell without any mutable reference or mutable data
let m = RefCell::new(10); 
 *m.borrow_mut()+=1;
 println!("{:?}",m.into_inner());
}
