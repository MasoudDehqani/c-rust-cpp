pub fn search_query_in_content<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];

    content
        .lines()
        .filter(|&l| l.contains(query))
        .for_each(|s| res.push(s));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        let content = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

        assert_eq!(
            vec!["Are you nobody, too?"],
            search_query_in_content(content, "too")
        );
    }
}
