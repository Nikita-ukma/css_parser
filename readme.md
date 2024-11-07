# CSS Parser

This CSS parser is built using the `pest` parsing library in Rust. It parses simple CSS rules into components such as selectors, properties, and values.

## Grammar

The `.pest` grammar file defines the rules for parsing a CSS declaration block. Below is a breakdown of each rule:

- `CSS_RULE`: The main rule for parsing a single CSS rule, consisting of a selector followed by a declaration block.
- `selector`: Matches a simple CSS selector, such as a tag name (`body`, `div`).
- `declarations`: Matches multiple declarations within `{ ... }`.
- `declaration`: Parses individual declarations like `property: value;`.
- `property`: Matches the CSS property name.
- `value`: Matches the value assigned to a CSS property.

### Example of a Simple CSS Rule

This parser can handle basic CSS declarations, such as:

```css
div {
    color: red;
    width: 100px;
}
