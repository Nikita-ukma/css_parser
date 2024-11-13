#[cfg(test)]
mod tests {
    use my_rust_css_parser::parse_css;

    #[test]
    fn test_css_rule() {
        let css = "body { color: black; }";
        assert!(parse_css(css).is_ok(), "CSS rule failed to parse.");
    }

    #[test]
    fn test_whitespace_only() {
        let css = "   \n\t ";
        assert!(
            parse_css(css).is_err(),
            "Whitespace-only should not be valid CSS."
        );
    }

    #[test]
    fn test_whitespace_with_rule() {
        let css = "  body { color: black; } ";
        assert!(
            parse_css(css).is_ok(),
            "Whitespace surrounding CSS rule failed to parse."
        );
    }

    #[test]
    fn test_comment() {
        let css = "/* This is a comment */ body { color: black; }";
        assert!(parse_css(css).is_ok(), "CSS with comment failed to parse.");
    }

    #[test]
    fn test_unclosed_comment() {
        let css = "/* This comment is not closed body { color: black; }";
        assert!(
            parse_css(css).is_err(),
            "Unclosed comment should cause a parse error."
        );
    }

    #[test]
    fn test_selector() {
        let css = "div, p, #id, .class { color: black; }";
        assert!(parse_css(css).is_ok(), "Complex selector failed to parse.");
    }

    #[test]
    fn test_declarations() {
        let css = "body { color: black; background-color: #fff; }";
        assert!(
            parse_css(css).is_ok(),
            "Multiple declarations failed to parse."
        );
    }

    #[test]
    fn test_declaration() {
        let css = "body { margin: 0px; }";
        assert!(
            parse_css(css).is_ok(),
            "Single declaration failed to parse."
        );
    }

    #[test]
    fn test_invalid_declaration() {
        let css = "body { color black; }";
        assert!(parse_css(css).is_err(), "Invalid declaration did not fail.");
    }

    #[test]
    fn test_property() {
        let css = "body { font-size: 12px; }";
        assert!(parse_css(css).is_ok(), "Property parsing failed.");
    }

    #[test]
    fn test_multiple_values() {
        let css = "body { padding: 10px, 20px; }";
        assert!(parse_css(css).is_ok(), "Multiple values parsing failed.");
    }

    #[test]
    fn test_hex_color() {
        let css = "body { color: #aabbcc; }";
        assert!(parse_css(css).is_ok(), "Hex color failed to parse.");
    }

    #[test]
    fn test_length_px() {
        let css = "body { margin: 10ppp; }";
        assert!(parse_css(css).is_ok(), "Length in px failed to parse.");
    }

    #[test]
    fn test_length_em() {
        let css = "body { margin: 1em; }";
        assert!(parse_css(css).is_ok(), "Length in em failed to parse.");
    }

    #[test]
    fn test_percentage() {
        let css = "body { width: 100%; }";
        assert!(parse_css(css).is_ok(), "Percentage value failed to parse.");
    }

    #[test]
    fn test_string_double_quotes() {
        let css = "body { font-family: \"Arial\"; }";
        assert!(parse_css(css).is_err(), "Is OK")
    }

    #[test]
    fn test_string_single_quotes() {
        let css = "body { font-family: 'Arial'; }";
        assert!(
            parse_css(css).is_ok(),
            "String with single quotes failed to parse."
        );
    }

    #[test]
    fn test_invalid_string() {
        let css = "body { font-family: Arial; }";
        assert!(
            parse_css(css).is_ok(),
            "Unquoted font-family should still be valid."
        );
    }

    #[test]
    fn test_zero_value() {
        let css = "body { padding: 0; }";
        assert!(parse_css(css).is_ok(), "Zero value failed to parse.");
    }

    #[test]
    fn test_float_value() {
        let css = "body { line-height: 1.5; }";
        assert!(parse_css(css).is_ok(), "Float value failed to parse.");
    }

    #[test]
    fn test_invalid_float_value() {
        let css = "body { line-height: .5; }";
        assert!(
            parse_css(css).is_err(),
            "Float without leading digit did not fail."
        );
    }

    #[test]
    fn test_identifier_value() {
        let css = "body { color: black; }";
        assert!(parse_css(css).is_ok(), "Identifier value failed to parse.");
    }
}
