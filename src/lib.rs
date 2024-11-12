use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CSSParser;
pub fn parse_css(input: &str) -> Result<(), Box<pest::error::Error<Rule>>> {
    match CSSParser::parse(Rule::CSS_RULE, input) {
        Ok(pairs) => {
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
        Err(e) => {
            eprintln!("Parsing error: {:?}", e);
            Err(Box::new(e))
        }
    }
}
