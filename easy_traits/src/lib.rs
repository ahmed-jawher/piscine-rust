pub trait AppendStr {
    fn append_str(&mut self, str_to_append: &str) -> Self;
    fn append_number(&mut self, nb_to_append: f64) -> Self;
    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for String {
    fn append_str(&mut self, str_to_append: &str) -> Self {
        self.push_str(str_to_append);
        self.clone()
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.push_str(&nb_to_append.to_string());
        self.clone()
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        self.retain(|c| !matches!(c, '.' | ',' | '?' | '!'));
        self.clone()
    }
}
