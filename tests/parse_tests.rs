use css_parser::parse_css;

mod tests {
    use super::*;
    #[test]
    fn test_basic_css_parsing() {
        let css = "body { color: black; }";
        assert!(parse_css(css).is_ok());
    }

    #[test]
    fn test_invalid_css_parsing() {
        let invalid_css = "body { color black }"; // missing colon and semicolon
        assert!(parse_css(invalid_css).is_err());
    }

    #[test]
    fn test_whitespace_parsing() -> anyhow::Result<()> {
        let css = " ";
        let result = parse_css(css);
        assert!(
            result.is_err(),
            "Whitespace alone should not be a valid"
        );
        Ok(())
    }
    #[test]
    fn test_css_rule_parsing() -> anyhow::Result<()> {
        let css = "body { color: black; }";
        assert!(parse_css(css).is_ok(), "Valid CSS rule failed to parse");
        Ok(())
    }

    #[test]
    fn test_selector_parsing() -> anyhow::Result<()> {
        let css = "body";
        let result = parse_css(css);
        assert!(
            result.is_err(),
            "Selector alone should not be a valid"
        );
        Ok(())
    }

    #[test]
    fn test_declarations_parsing() -> anyhow::Result<()> {
        let css = "{ color: black; }";
        let result = parse_css(css);
        assert!(
            result.is_err(),
            "Declarations without selector should not be valid"
        );
        Ok(())
    }
    #[test]
    fn test_declaration_parsing() -> anyhow::Result<()> {
        let css = "color: black;";
        let result = parse_css(css);
        assert!(
            result.is_err(),
            "Declaration alone should not be a valid"
        );
        Ok(())
    }

    #[test]
    fn test_property_parsing() -> anyhow::Result<()> {
        let css = "{ color: ; }"; // Missing value
        let result = parse_css(css);
        assert!(
            result.is_err(),
            "Property without value should fail to parse"
        );
        Ok(())
    }
    #[test]
    fn test_property_value_parsing() -> anyhow::Result<()> {
        let css = "body { color: black; }";
        assert!(
            parse_css(css).is_ok(),
            "Valid declaration with property and value in CSS rule failed to parse"
        );
        Ok(())
    }
}
