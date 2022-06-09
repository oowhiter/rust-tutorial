#![allow(unused)]

mod thread {
    use std::thread;
    use std::time::Duration;

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn spawn() {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });
            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }

        #[test]
        fn join_last() {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });
            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
            handle.join().unwrap();
        }

        #[test]
        fn join_mid() {
            let handle = thread::spawn(|| {
                for i in 1..10 {
                    println!("hi number {} from the spawned thread!", i);
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handle.join().unwrap();
            for i in 1..5 {
                println!("hi number {} from the main thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        }

        #[test]
        fn with_move() {
            let v = vec![1, 2, 3];
            let handle = thread::spawn(move || {
                println!("Here's a vector: {:?}", v);
            });
            // drop(v);
            handle.join().unwrap();
        }
    }
}

mod channel {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn use_channel() {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();
                // val is already moved.
            });

            // blocking
            let received = rx.recv().unwrap();
            // // non-blocking
            // let received = rx.try_recv().unwrap();

            println!("Got: {}", received);
        }

        #[test]
        fn multi_val() {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }

        #[test]
        fn multi_sender() {
            let (tx, rx) = mpsc::channel();

            let tx1 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("thread"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            thread::spawn(move || {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1));
                }
            });

            for received in rx {
                println!("Got: {}", received);
            }
        }
    }
}

mod mutex {
    use std::sync::{Mutex, Arc};
    use std::thread;

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn use_mutex() {
            let m = Mutex::new(5);
            {
                // blocking
                let mut num = m.lock().unwrap();
                // // non-blocking
                // m.try_lock()

                *num = 6;
            }
            println!("m = {:?}", m);
        }

        #[test]
        // Mutex cannot be shared between threads.
        // `Rc` is not thread-safe.
        // `Arc`, Atomically Reference Counted, is thread-safe!
        fn share_mutex() {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.lock().unwrap());
        }
    }
}
