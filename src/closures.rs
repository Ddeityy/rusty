fn main() {
    use std::thread;
    // anonymous fn that returns itself
    let closure = |x| x;
    // closure is now infered as String
    let s = closure(String::from("sTrInG"));
    // calling closure(1) will not compile due to type inference
    println!("{:?}", s);

    let list = vec![1, 2, 3];
    let mut mut_list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let immut_borrow = || println!("From closure: {:?}", list);
    let mut mut_borrow = || mut_list.push(7);

    // create a new thread
    // if the main thread finishes before the new thread
    // &list will be invalid, so move owndership to closure
    thread::spawn(move || println!("From thread: {:?}", list))
        .join() // wait for all threads to finish and convert to <JoinHandle>
        .unwrap(); // unwraps <JoinHandle>
                   //immut_borrow();
                   //mut_borrow();

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    // gets called 6 times, sorts 1 by 1, FnMut
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?} \nsorted in {num_sort_operations} operations", list);
}
