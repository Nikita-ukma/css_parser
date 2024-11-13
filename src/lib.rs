//! CSS Parser CLI
//! 
//! This parser allows you to parse CSS files, identifying the main selector types
//! (class, id, or element selector) and primary attributes with their values.

use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CSSParser;pub fn parse_css(input: &str) -> anyhow::Result<()> {
    let pairs = CSSParser::parse(Rule::CSS_RULE, input)?;

    for pair in pairs {
        println!("Rule: {:?}", pair.as_rule());
        println!("Span: {:?}", pair.as_span());
        println!("Text: {}", pair.as_str());

        for inner_pair in pair.clone().into_inner() {
            println!("  Inner Rule: {:?}", inner_pair.as_rule());
            println!("  Inner Span: {:?}", inner_pair.as_span());
            println!("  Inner Text: {}", inner_pair.as_str());
        }
    }

    Ok(())
}