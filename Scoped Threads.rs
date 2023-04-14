use std::thread::scope;
fn main() {
    let mut m = vec![1, 2, 3, 4, 5];
    scope(|s| {
        s.spawn(|| {
            m.push(56);
            let rm = m.pop();
            println!("The removed element is {rm:?}");
        });
    });
    //We can read the data across thread without Arc.
    scope(|s| {
        s.spawn(|| {
            println!("{:?}", m);
        });
        s.spawn(||{
            println!("{}",m[0]+m[1]);
        });
    });
}
