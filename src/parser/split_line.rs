pub fn split_line(line: &str) -> (Option<&str>, Option<&str>) {
    // split the line in a left and right hand sides
    if line.is_empty() {
        return (None, None);
    }
    match line.split_once('\t') {
        None => (Some(line), None),
        Some((left, right)) => {
            if right.is_empty() {
                return (Some(left), None);
            }
            (Some(left), Some(right))
        }
    }
}

#[cfg(test)]
mod split_line_tests {
    use super::*;
    #[test]
    fn test_line_is_empty() {
        let test_str = "";
        assert_eq!((None, None), split_line(&test_str));
    }
    #[test]
    fn test_line_has_no_tab() {
        let test_str = "notab";
        assert_eq!((Some("notab"), None), split_line(&test_str));
    }
    #[test]
    fn test_line_has_no_tab_field_has_space() {
        let test_str = "no tab";
        assert_eq!((Some("no tab"), None), split_line(&test_str));
    }
    #[test]
    fn test_line_has_one_tab() {
        let test_str = "one\ttab";
        assert_eq!((Some("one"), Some("tab")), split_line(&test_str));
    }
    #[test]
    fn test_line_has_one_tab_and_nothing_after() {
        let test_str = "one\t";
        assert_eq!((Some("one"), None), split_line(&test_str));
    }
    #[test]
    fn test_line_has_two_tabs() {
        let test_str = "one\ttab\tanother";
        assert_eq!((Some("one"), Some("tab\tanother")), split_line(&test_str));
    }
}
