use std::fs::File;
use std::io::*;
use std::thread::spawn;
fn main() {
    //The data in the main thread
    let data = File::create("Concurrency.txt").unwrap();

    let task_1 = spawn(move || {
        //This closure have code to manipulate data.
        //The data is moved here to the closure.
        let mut task = data;
        for _ in 0..100 {
            task.write(b"Spawning_Thread ").unwrap();
        }
        let mut file = File::open("Concurrency.txt").unwrap();
        let mut st = String::new();
        file.read_to_string(&mut st).unwrap();
        let vec = st
            .split_whitespace()
            .map(|st| st.to_string())
            .collect::<Vec<_>>();
        return vec;
    })
    .join()
    .unwrap();

    let task_2 = spawn(move || {
        //the task here is to sum the even integers between (0 to 10000)
        let task = (0..=10000)
            .into_iter()
            .filter(|&el: &usize| {
                let el = if el % 2 == 0 { true } else { false };

                el
            })
            .sum::<usize>();
        return task;
    })
    .join()
    .unwrap();

    //we don't use move here because the data and code is live in closure and returns
    //it to task_3 where we can use them inside main thread.
    let task_3 = spawn(|| {
        //the task here is to find the count the digit which are power of two
        //between 0 to 1000(inclusive).
        let task = (0..=1000)
            .into_iter()
            .filter(|&el: &usize| el.is_power_of_two())
            .count();
        return task;
    })
    .join()
    .unwrap();

    //The job of main thread is to wait for the child threads to printing the result from them

    //Only printing first five so that we clearly see them
    println!("Result from first thread  {:?}\n", &task_1[..5]);

    println!("Result from second thread  {}\n", task_2);

    println!("Result from third thread{}\n", task_3);
}
