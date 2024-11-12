# tieliehin_css_parser

This CSS parser is built using the `pest` parsing library in Rust. It parses simple CSS rules into components such as selectors, properties, and values.

## Grammar

The .pest grammar file defines the rules for parsing a CSS declaration block. Below is a breakdown of each rule:

- WHITESPACE: Matches whitespace characters, including spaces, tabs, newlines, and carriage returns, to be ignored in parsing.
- COMMENT: Matches CSS comments enclosed by /* and */.
- CSS_RULE: The main rule for parsing a single CSS rule, consisting of optional comments/whitespace, a selector, and declarations within { ... }.
- selector: Matches a CSS selector, which may include multiple simple_selector items separated by commas.
- simple_selector: Matches a basic CSS selector, such as an identifier, ID selector (#header), class selector (.button), or alphanumeric sequence.
- declarations: Matches multiple declaration entries inside a CSS rule.
- declaration: Parses an individual CSS declaration with a property, value, and trailing semicolon.
- property: Matches a CSS property name, including alphanumeric characters and hyphens.
- values: Matches one or more values for a property, with items separated by commas or whitespace.
- value: Matches various CSS value types, including color codes, lengths, percentages, strings, and numeric values.
- COLOR: Matches hexadecimal color codes with 3 to 6 hexadecimal digits.
- LENGTH: Matches length units in px, em, rem, or %.
- PERCENTAGE: Matches percentage values.
- STRING: Matches text in single or double quotes.
- ZERO: Matches the literal value 0.
- FLOAT: Matches floating-point numbers.
- IDENTIFIER: Matches alphanumeric keywords and names with optional hyphens.

### Example of a Simple CSS Rule

This parser can handle basic CSS declarations, such as:

```css
footer {
    background-color: #333;
    color: #f4f4f9;
    text-align: center;
    padding: 10px 0;
    margin-top: 20px;
}
```

```pest
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
COMMENT = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
CSS_RULE = _{ (COMMENT | WHITESPACE)* ~ selector ~ "{" ~ WHITESPACE* ~ declarations ~ WHITESPACE* ~ "}" }
selector = @{ (simple_selector ~ (WHITESPACE* ~ "," ~ WHITESPACE* ~ simple_selector)*) }
simple_selector = @{ IDENTIFIER | ("#" ~ IDENTIFIER+)
                   | ("." ~ IDENTIFIER+)
                   | ASCII_ALPHANUMERIC+ }
declarations = _{ (declaration ~ WHITESPACE*)+ }
declaration = _{ property ~ ":" ~ WHITESPACE* ~ values ~ ";" ~ WHITESPACE* }
property = @{ (ASCII_ALPHANUMERIC | "-")+ }  
values = _{ value ~ (WHITESPACE* ~ "," ~ WHITESPACE* ~ value)* }

value = @{ COLOR | LENGTH | PERCENTAGE | STRING | ZERO | FLOAT |  IDENTIFIER | ASCII_ALPHANUMERIC+}

COLOR = @{ "#" ~ ASCII_HEX_DIGIT{3,6} }
LENGTH = @{ ASCII_DIGIT+ ~ ("px" | "em" | "rem" | "%") }
PERCENTAGE = @{ ASCII_DIGIT+ ~ "%" }
STRING = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\'" | "\'" ~ (!"\'" ~ ANY)* ~ "\'" }
ZERO = @{ "0" }
FLOAT = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ } 
IDENTIFIER = @{ (ASCII_ALPHANUMERIC | "-")+ }
