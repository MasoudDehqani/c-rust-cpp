#![allow(unused_imports)]

use std::{
    pin::{Pin, pin},
    process::Output,
};

use asynchronous_programming::{get_page_title, race};
use futures::{
    future::{Either, join, join_all, join3},
    join,
};
use tokio::{
    runtime::Runtime,
    sync::mpsc::unbounded_channel,
    task::spawn,
    time::{Duration, sleep},
};

fn main() {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let (tx, mut rx) = unbounded_channel::<String>();
        let tx2 = tx.clone();

        let fut1 = async move {
            let messages = vec!["hi", "from", "first", "future"];
            for msg in messages {
                tx.send(format!("message: {msg}")).unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async move {
            let messages = vec!["hello", "from2", "second", "future2"];
            for msg in messages {
                tx2.send(format!("{msg}")).unwrap();
                sleep(Duration::from_millis(500)).await;
            }
        };

        let fut3 = async {
            while let Some(msg) = rx.recv().await {
                println!("{}", msg);
            }
        };

        // join3(fut1, fut2, fut3).await;
        // join!(fut1, fut2, fut3);
        let fut1 = Box::pin(fut1);
        let fut2 = Box::pin(fut2);
        let fut3 = Box::pin(fut3);
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![fut1, fut2, fut3];

        join_all(futures).await;
    })
}

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
