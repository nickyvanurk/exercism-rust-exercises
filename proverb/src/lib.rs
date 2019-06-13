pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() { return String::new(); }

    let mut proverb = String::new();
    let mut last_word = list[0];

    for word in list.iter().skip(1) {
        proverb += format!(
            "For want of a {} the {} was lost.\n",
            last_word, word
        ).as_str();

        last_word = word;
    }

    proverb + format!("And all for the want of a {}.", list[0]).as_str()
}
