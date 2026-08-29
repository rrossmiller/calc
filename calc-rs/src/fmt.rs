pub fn format_sexp(input: String) -> String {
    let mut result = String::new();
    let mut depth = 0;
    // Split tokens while keeping parenthesis, then filter out empty spaces
    let tokens = input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for (i, token) in tokens.iter().enumerate() {
        match token.as_str() {
            "(" => {
                if i > 0 && tokens[i - 1] != "(" {
                    result.push('\n');
                    result.push_str(&"  ".repeat(depth));
                }
                result.push('(');
                depth += 1;
            }
            ")" => {
                if depth > 0 {
                    depth -= 1;
                }
                result.push(')');
            }
            _ => {
                // Add a space between sibling atoms, but newline for nested items
                if i > 0 && tokens[i - 1] != "(" && tokens[i - 1] != ")" {
                    result.push(' ');
                } else if i > 0 && tokens[i - 1] == ")" {
                    result.push('\n');
                    result.push_str(&"  ".repeat(depth));
                }
                result.push_str(token);
            }
        }
    }
    result
}
