pub trait Summary {
    fn summary(&self) -> String;
    fn summary_with_default(&self) -> String {
        String::from("This is a default summary")
    }
    fn summary_from_summary(&self) -> String {
        format!("summary from summary: {}", self.summary())
    }
}
