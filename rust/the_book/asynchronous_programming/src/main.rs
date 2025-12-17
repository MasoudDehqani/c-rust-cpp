#![allow(unused_imports)]

use std::{
    pin::{Pin, pin},
    process::Output,
    thread,
};

use asynchronous_programming::{get_page_title, race};
use futures::{
    future::{Either, join, join_all, join3},
    join,
};
use tokio::{
    runtime::Runtime,
    sync::mpsc::unbounded_channel,
    task::{spawn, yield_now},
    time::{Duration, Instant, sleep},
};

#[tokio::main]
async fn main() {
    let slow = async {
        sleep(Duration::from_secs(5)).await;
        "slow finished"
    };

    match timeout(slow, Duration::from_secs(2)).await {
        Ok(res) => println!("done with {res}"),
        Err(s) => println!("failed to be done in {} secs", s.as_secs_f32()),
    }
}

async fn timeout<F: Future>(fut: F, max_time: Duration) -> Result<F::Output, Duration> {
    let res = race(fut, sleep(max_time)).await;

    match res {
        Either::Left(a) => Ok(a),
        Either::Right(_) => Err(max_time),
    }
}

/*
  This is a form of cooperative multitasking.
  In some Rust-based embedded operation systems, this is the only
  kind of multitasking!
*/
// #[tokio::main]
// async fn main() {
//     let one_nanosecond = Duration::from_nanos(1);
//     let start = Instant::now();
//     async {
//         for _ in 1..1000 {
//             sleep(one_nanosecond).await;
//         }
//     }
//     .await;
//     let time = Instant::now() - start;
//     println!("sleep version finished in {}", time.as_secs_f32());

//     let start = Instant::now();
//     async {
//         for _ in 1..1000 {
//             yield_now().await;
//         }
//     }
//     .await;
//     let time = Instant::now() - start;
//     println!("yield version finished in {}", time.as_secs_f32());
// }

// #[tokio::main]
// async fn main() {
//     let a = async {
//         println!("a started");
//         slow("a", 30);
//         yield_now().await;
//         slow("a", 10);
//         yield_now().await;
//         slow("a", 20);
//         yield_now().await;
//         println!("a finished");
//     };

//     let b = async {
//         println!("b started");
//         slow("b", 75);
//         yield_now().await;
//         slow("b", 10);
//         yield_now().await;
//         slow("b", 15);
//         yield_now().await;
//         slow("b", 350);
//         yield_now().await;
//         println!("b finished");
//     };

//     race(a, b).await;
// }

// fn slow(name: &str, ms: u64) {
//     thread::sleep(Duration::from_millis(ms));
//     println!("{name} ran for {ms}ms");
// }

// #[tokio::main]
// async fn main() {
//     let a = async {
//         println!("a started");
//         slow("a", 30);
//         sleep(Duration::from_millis(1)).await;
//         slow("a", 10);
//         sleep(Duration::from_millis(1)).await;
//         slow("a", 20);
//         sleep(Duration::from_millis(1)).await;
//         println!("a finished");
//     };

//     let b = async {
//         println!("b started");
//         slow("b", 75);
//         sleep(Duration::from_millis(1)).await;
//         slow("b", 10);
//         sleep(Duration::from_millis(1)).await;
//         slow("b", 15);
//         sleep(Duration::from_millis(1)).await;
//         slow("b", 350);
//         sleep(Duration::from_millis(1)).await;
//         println!("b finished");
//     };

//     race(a, b).await;
// }

// #[tokio::main]
// async fn main() {
//     let slow = async {
//         println!("slow started");
//         sleep(Duration::from_millis(100)).await;
//         println!("slow finished");
//     };

//     let fast = async {
//         println!("fast started");
//         sleep(Duration::from_millis(50)).await;
//         println!("fast finished");
//     };

//     race(slow, fast).await;
// }

// fn main() {
//     let runtime = Runtime::new().unwrap();

//     runtime.block_on(async {
//         let (tx, mut rx) = unbounded_channel::<String>();
//         let tx2 = tx.clone();

//         let fut1 = async move {
//             let messages = vec!["hi", "from", "first", "future"];
//             for msg in messages {
//                 tx.send(format!("message: {msg}")).unwrap();
//                 sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async move {
//             let messages = vec!["hello", "from2", "second", "future2"];
//             for msg in messages {
//                 tx2.send(format!("{msg}")).unwrap();
//                 sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut3 = async {
//             while let Some(msg) = rx.recv().await {
//                 println!("{}", msg);
//             }
//         };

//         // join3(fut1, fut2, fut3).await;
//         // join!(fut1, fut2, fut3);

//         // let fut1 = Box::pin(fut1);
//         // let fut2 = Box::pin(fut2);
//         // let fut3 = Box::pin(fut3);
//         // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![fut1, fut2, fut3];

//         let fut1 = pin!(fut1);
//         let fut2 = pin!(fut2);
//         let fut3 = pin!(fut3);
//         let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![fut1, fut2, fut3];

//         join_all(futures).await;
//     })
// }

// fn main() {
//     let runtime = Runtime::new().unwrap();

//     runtime.block_on(async {
//         let fut1 = async {
//             for i in 0..10 {
//                 println!("{i} from task 1");
//                 sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async {
//             for j in 0..5 {
//                 println!("{j} from task 2");
//                 sleep(Duration::from_millis(500)).await;
//             }
//         };

//         join(fut1, fut2).await;

//         // Here again the code will be executed line by line
//         // fut1.await;
//         // fut2.await;
//     });
// }

// fn main() {
//     let args = std::env::args().collect::<Vec<String>>();
//     let runtime = Runtime::new().unwrap();

//     runtime.block_on(async {
//         let url1 = &args[1];
//         let url2 = &args[2];

//         let fut_1 = get_page_title(url1);
//         let fut_2 = get_page_title(url2);

//         let (url, maybe_title) = match race(fut_1, fut_2).await {
//             Either::Left(l) => l,
//             Either::Right(r) => r,
//         };

//         println!("url {} won!", url);
//         match maybe_title {
//             Some(title) => println!("title is {}", title),
//             None => println!("No title!"),
//         }
//     })
// }

// fn main() {
//     let args = std::env::args().collect::<Vec<String>>();
//     let runtime = Runtime::new().unwrap();
//     runtime.block_on(async {
//         let url = &args[1];
//         match get_page_title(url).await {
//             Some(title) => println!("Title is: {}", title),
//             None => println!("No Title"),
//         }
//     });
// }
