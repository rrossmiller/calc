pub enum NodeType {
    Number,
    // IDENTIFIER,
    CallExpression,
    BinaryExpression,
    ParenthesizedExpression,
    Assignment,
    Identifier,
}

pub enum Operator {
    AddOperator,
    SubtractOperator,
    MultiplyOperator,
    DivideOperator,
}

pub fn resolve_kind(node: tree_sitter::Node) -> Option<NodeType> {
    match node.kind() {
        "number" => Some(NodeType::Number),
        "identifier" => Some(NodeType::Identifier),
        "call_expression" => Some(NodeType::CallExpression),
        "binary_expression" => Some(NodeType::BinaryExpression),
        "parenthesized_expression" => Some(NodeType::ParenthesizedExpression),
        "assignment_expression" => Some(NodeType::Assignment),
        _ => None,
    }
}
pub fn resolve_operator(node: tree_sitter::Node) -> Option<Operator> {
    match node.kind() {
        "add_operator" => Some(Operator::AddOperator),
        "subtract_operator" => Some(Operator::SubtractOperator),
        "multiply_operator" => Some(Operator::MultiplyOperator),
        "divide_operator" => Some(Operator::DivideOperator),
        _ => None,
    }
}
