use anyhow::Result;

pub fn print_matches(pattern: &str, content: &str, mut writer: impl std::io::Write) -> Result<()>  {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}",line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_matches_prints_matched_line() {
        let test_content = "first line\nsecond line\nthird line";
        
        let mut result = Vec::new();
        print_matches("second", test_content, &mut result).unwrap();

        assert_eq!(result, b"second line\n");

    }
}