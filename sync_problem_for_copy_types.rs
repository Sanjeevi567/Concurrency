#[allow(unused_variables,unused_assignments)]
use std::sync::{Mutex,Arc};
use std::thread::spawn;
fn main(){
let a= std::sync::Arc::new(Mutex::new([0,0,0,0]));
let udt_copy = UdtCopy::new();
let b = Arc::new(Mutex::new(udt_copy));
let a_clone = a.clone();
let b_clone = b.clone();
spawn(move||{
//these values are created inside the thread so changes not reflected.
    let mut y = *a_clone.lock().unwrap(); 
     y=[1,2,3,4];
     println!("a in thread \n {:?}\n",y);
     
     //for a copy type we get indepedent copy i.e not synced with the mutex when 
     //dereferenced data stored in the variable so making changes to them
     //only reflected in the local scope.
     let mut udt = *b_clone.lock().unwrap(); 
     udt.push(10,10.0,[1,2,3,4,5],"Not reflected in b");
     println!("b in thread \n {:?}\n",udt);
     
}).join().unwrap();
println!("a should be changed in main thread but not \n {:?}\n",a.lock().unwrap());
println!("b should be changed in main thread but not \n {:?}\n",b.lock().unwrap());
//This is possible because we can take out the values only if the type is copy.
//Move type can't allowed this.
//assert_eq!(*a.lock().unwrap(),[1, 2, 3, 4]);
//assert_eq!(*b.lock().unwrap(),UdtCopy { n: 10, n1: 10.0, n2: [1, 2, 3, 4, 5], n3: "Not reflected in b" });
let a_clone= a.clone();
let b_clone= b.clone();
spawn(move||{
    let mut y = a_clone.lock().unwrap();
     *y = [1,2,3,4];
     println!("a in other thread \n{:?}\n",y);
     
     //Here the data is synchronized with the outer Mutex,
     //Thus the results are reflected in all threads.
     let mut udt = b_clone.lock().unwrap(); 
     udt.push(10,10.0,[1,2,3,4,5],"reflected in b");
     println!("b in other thread \n{:?}\n",udt);
}).join().unwrap();
println!("a is changed here \n{:?}\n",a.lock().unwrap());
println!("b is changed here \n {:?}",b.lock().unwrap());
}
//All types are copy.
#[derive(Copy,Clone,Debug)]
struct UdtCopy{
    n:i32,
    n1:f64,
    n2:[i32;5],
    n3:&'static str
}
impl UdtCopy{
    fn new()->Self{
        Self{
            n:Default::default(),
            n1:Default::default(),
            n2:Default::default(),
            n3:Default::default(),
        }
    }
    fn push(&mut self,n:i32,n1:f64,n2:[i32;5],n3:&'static str){
        self.n =n;
        self.n1=n1;
        self.n2=n2;
        self.n3=n3;
    }
}
