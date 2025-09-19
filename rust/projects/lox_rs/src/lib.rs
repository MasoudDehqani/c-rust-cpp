pub struct Scanner;

impl Scanner {
    pub fn scan(source: String) {
        let tokens = Self::tokens(source);
        for token in tokens {
            println!("{}", token)
        }
    }

    fn tokens(_source: String) -> Vec<String> {
        vec![]
    }
}
