use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!(
        "The time; {:#?} : {:#?}",
        epoch,
        epoch + Duration::from_secs(1),
    );

    let _future = epoch + Duration::from_secs(60);

    lil_thread_test();
}

fn lil_thread_test() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::Builder::new()
                .name(format!("thready_weddy-{}", i))
                .spawn(|| {
                    loop_and_print();
                })
                .unwrap()
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join())
        .for_each(drop);
}

fn loop_and_print() {
    for _ in 1..10 {
        println!("{:?} sleeping for 1 second", thread::current().name());
        thread::sleep(Duration::from_millis(500))
    }
}
