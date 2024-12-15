use std::sync::atomic::{AtomicBool, Ordering};

static EXIT: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut handle = interrupter::set_handler(|| {
        println!("interrupt signal intercepted!");
        EXIT.store(true, Ordering::Release);
    })
    .unwrap();

    // This task only does work
    let other_task = std::thread::spawn(|| {
        let mut counter = 0_u64;
        while !EXIT.load(Ordering::Acquire) {
            for _ in 0..1_000_000 {
                counter += 1;
            }
        }
        counter
    });

    // This loop works AND polls
    let mut main_counter = 0_u64;
    loop {
        // work
        for _ in 0..1_000_000 {
            main_counter += 1;
        }

        // poll
        if handle.poll() {
            break;
        }
    }

    // Retrieve results and print
    let counter = other_task.join().unwrap();
    println!("main_counter = {main_counter}; counter {counter}");
}
