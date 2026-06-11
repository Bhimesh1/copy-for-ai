pub fn estimate_tokens(content: &str) -> usize {
    let character_count = content.chars().count();

    if character_count == 0 {
        return 0;
    }

    character_count.div_ceil(4)
}

#[cfg(test)]
mod tests {
    use super::estimate_tokens;

    #[test]
    fn estimates_empty_text_as_zero_tokens() {
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn estimates_tokens_using_four_characters_per_token() {
        assert_eq!(estimate_tokens("abcd"), 1);
        assert_eq!(estimate_tokens("abcde"), 2);
    }
}
