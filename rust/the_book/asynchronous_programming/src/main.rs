use asynchronous_programming::{get_page_title, race};
use futures::future::Either;
#[allow(unused_imports)]
use tokio::runtime::Runtime;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let url1 = &args[1];
        let url2 = &args[2];

        let fut_1 = get_page_title(url1);
        let fut_2 = get_page_title(url2);

        let (url, maybe_title) = match race(fut_1, fut_2).await {
            Either::Left(l) => l,
            Either::Right(r) => r,
        };

        println!("url {} won!", url);
        match maybe_title {
            Some(title) => println!("title is {}", title),
            None => println!("No title!"),
        }
    })
}

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
