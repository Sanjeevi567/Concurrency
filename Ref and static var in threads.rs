use std::collections::HashSet;
use std::thread::spawn;
fn main() {
    let stack_local: &str = "Live in main thread";
    let heap_local: HashSet<usize> = HashSet::new();
    spawn(|| {
        println!("{} {:?}", stack_local, heap_local);
    })
    .join()
    .unwrap();

    static mut MONTHS: u8 = 12;
    spawn(|| {
        println!("{}", MONTHS);
    })
    .join()
    .unwrap();
}
