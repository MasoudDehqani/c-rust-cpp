use std::io::ErrorKind;

mod io_reader;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query is not provided"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("file path is not provided"),
        };

        Ok(Config { query, file_path })
    }
}

pub fn mini_grep(query: &str, file_path: &str) {
    let file_content = io_reader::utils::reader(file_path);

    match &file_content {
        Ok(s) => {
            io_reader::utils::handle_content_in_query(&s, &query);
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("Not Found"),
            _ => println!("NOTHING"),
        },
    };
}
