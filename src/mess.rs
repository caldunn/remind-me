use rmlib::time_heap::{Request, TimingHeap};

fn main() {
    println!("Hello World!");
    let mut notification_store = TimingHeap::new();
    notification_store.add(Request::new(13))
}
