use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] 
struct CSSParser;

pub fn parse_css(input: &str) -> Result<(), Box<pest::error::Error<Rule>>> {
    let pairs = CSSParser::parse(Rule::CSS_RULE, input)?;
    for pair in pairs {
        println!("Rule: {:?}", pair.as_rule());
        println!("Span: {:?}", pair.as_span());
        println!("Text: {}", pair.as_str());
    }
    Ok(())
}
