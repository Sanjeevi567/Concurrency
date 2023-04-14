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
    scope(|s| {
        s.spawn(|| {
            println!("{:?}", m);
        });
    });
}
