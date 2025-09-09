/*
  It seems MutexGuard which is the Ok() side of the Result type of lock() method return type
  on Mutex is a smart pointer and can be dereferenced using * operator

  - multiple ownership methods with Rc<T> and Arc<T>
  - Rc<T> -> reference counting
  - Arc<T> -> atomically reference-counted
  - Atomics are an additional kind of concurrency primitive
  - atomics work like primitive types but are safe to share across threads

  - counter is immutable, but we could get a mutable reference to value inside it; this means
  Mutex provides interior mutability as the Cell family does
  - Mutex<T> comes with the risk of creating deadlocks

  TODO: try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for
  mutexes in any language and have a go at implementing them in Rust. The standard library API
  documentation for Mutex<T> and MutexGuard offers useful information
*/

use std::{
    // rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn share_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut counter_value = c.lock().unwrap();
            *counter_value += 1;
        });

        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", *counter.lock().unwrap());
}

/*
  STEP 4
*/
fn _share_state_step_4() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", *counter.lock().unwrap())
}

/*
  STEP 3
*/
// fn _share_state_step_3() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..5 {
//         let counter = Rc::clone(&counter);
//         let h = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });

//         handles.push(h);
//     }

//     handles.into_iter().for_each(|h| h.join().unwrap());

//     println!("{}", *counter.lock().unwrap())
// }

/*
  STEP 2
*/
// fn _share_state_step_2() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..5 {
//         let h = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });

//         handles.push(h);
//     }

//     for h in handles {
//         h.join().unwrap()
//     }

//     println!("{}", *counter.lock().unwrap())
// }

/*
  STEP 1
*/
fn _share_state_step_1() {
    let n = Mutex::new(0);

    {
        let mut num = n.lock().unwrap();
        *num = 2;
    }

    println!("{n:?}");
}

fn _playground() {
    let counter = Mutex::new(0_i32);

    let s = counter.lock().unwrap();
    let _t: i32 = (*s).abs();
}
