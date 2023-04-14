use std::thread::spawn;
use std::sync::mpsc::channel;
fn main(){
    let (sender,receiver) = channel();
    //we can have as many as clone we want but only one receiver i.e
    //receiver can't be cloned.
    let first_clone = sender.clone();
    let second_clone = sender.clone();
    spawn(move||{ 
     //the task here is to convert the vector of chars to vector of
     //strings with added " first spawn" to each char values.
      let task = vec!['a','b','c'].iter().
       map(|el|{
       let mut el = el.to_string();
       el.push_str(" First spawn");
       el})
      .collect::<Vec<String>>();
      
      sender.send(task).unwrap();
    });
   // println!("{:?}",sender);
     spawn(move||{
     //the task here is to convert the vector of i32 to vector of
     //strings with added " second spawn" to each i32 values.
      let task = vec![1,2,3,4,5].iter()
      .map(|el|el.to_string())
       .map(|mut r#str|{
       r#str.push_str(" Second spawn");
       r#str
       })
      .collect::<Vec<String>>();
      
      first_clone.send(task).unwrap();
    });
   // println!("{:?}",first_clone);
    spawn(move||{
    //the task here is to find all the digit which are power of two and
    //convert them to vector of strings
    let task = (0..=1000).into_iter()
      .filter(|&el:&usize|el.is_power_of_two())
      .map(|power|power.to_string())
      .collect::<Vec<_>>();
      
    second_clone.send(task).unwrap();
    });
    //println!("{:?}",second_clone);
    //The receiver returns iterator but the order of receiving 
    //them is different for each run.
    for i in receiver{
        println!("\n{:?}",i);
    }
    
}
