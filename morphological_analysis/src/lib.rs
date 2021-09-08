pub fn extract(text: &str) -> &str {
    for i in text.chars() {
        print!("{},", i);
    }
    return text;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
