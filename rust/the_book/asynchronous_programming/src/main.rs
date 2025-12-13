use asynchronous_programming::get_page_title;
#[allow(unused_imports)]
use tokio::runtime::Runtime;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        let url = &args[1];
        match get_page_title(url).await {
            Some(title) => println!("Title is: {}", title),
            None => println!("No Title"),
        }
    });
}
