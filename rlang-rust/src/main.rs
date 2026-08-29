use crate::nodes::{NodeType, Operator};
use std::{
    error::Error,
    io::{self, Write},
};
use tree_sitter::Parser;
mod fmt;
mod nodes;
mod rlang_std;

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_rlang::LANGUAGE.into();

    parser.set_language(&lang)?;

    let mut source = String::new();
    loop {
        source.clear();

        // Get the input
        print!("> ");
        io::stdout().flush()?;

        io::stdin()
            .read_line(&mut source)
            .expect("Failed to read line");

        let source = source.trim();
        if source == "exit" || source.len() == 0 {
            break;
        }

        let tree = parser.parse(source, None).unwrap();

        // let fmt = fmt::format_sexp(tree.root_node().to_sexp());
        // println!("{}", fmt);

        // run
        let root = tree.root_node();
        if root.has_error() {
            println!("Invalid input");
            continue;
        }

        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            // println!("-> {}: {}", child.kind(), &source[child.byte_range()]);
            // println!("ans: {}", eval(child, source)?);
            let res = eval(child, source);
            match res {
                Ok(r) => println!(" = {}\n", r),
                Err(s) => println!("{}", s),
            }
        }
    }

    Ok(())
}

fn eval(node: tree_sitter::Node, source: &str) -> Result<f64, String> {
    if let Some(t) = nodes::resolve_kind(node) {
        match t {
            NodeType::Number => {
                return Ok(source[node.byte_range()].parse::<f64>().unwrap());
            }
            NodeType::BinaryExpression => {
                return eval_binary(node, source);
            }
            NodeType::ParenthesizedExpression => {
                return eval(node.child_by_field_name("expression").unwrap(), source);
            }
            NodeType::CallExpression => {
                return eval_call(node, source);
            }
            _ => {
                let e = format!("node not impl: {}", node.kind());
                return Err(e);
            }
        }
    }
    return Err("Node not implemented.".to_string());
}

fn eval_binary(node: tree_sitter::Node, source: &str) -> Result<f64, String> {
    // println!("BIN: {}", &source[node.byte_range()]);
    let left = node.child_by_field_name("left").unwrap();
    let l = eval(left, source)?; //.expect("issue evaluating expression");

    let right = node.child_by_field_name("right").unwrap();
    let r = eval(right, source)?; //.expect("issue evaluating expression");

    let op = node.child_by_field_name("operator").unwrap();
    if let Some(t) = nodes::resolve_operator(op) {
        match t {
            Operator::AddOperator => {
                return Ok(l + r);
            }
            Operator::SubtractOperator => {
                return Ok(l - r);
            }
            Operator::MultiplyOperator => {
                return Ok(l * r);
            }
            Operator::DivideOperator => {
                if r == 0.0 {
                    return Err("Cannot divide by 0.".to_string());
                }
                return Ok(l / r);
            }
        }
    }
    return Err("Illeagal binary node".to_string());
}

fn eval_call(node: tree_sitter::Node, source: &str) -> Result<f64, String> {
    let func = node.child_by_field_name("function").unwrap();
    let name = &source[func.byte_range()];
    match name {
        "pow" => {
            validate_arg_count(node, "pow", 2)?;
            let mut cursor = node.walk();
            let mut children = node.children_by_field_name("argument", &mut cursor);

            let base = eval(children.next().unwrap(), source)?; //.expect("unable to evaluate base argument");
            let exp = eval(children.next().unwrap(), source)?; //.expect("unable to evaluate exp argument");
            Ok(rlang_std::pow(base, exp))
        }
        "ftoc" => {
            validate_arg_count(node, "ftoc", 1)?;

            let arg = eval(node.child_by_field_name("argument").unwrap(), source)?; // .expect("unable to evaluate an argument");
            Ok(rlang_std::ftoc(arg))
        }
        "ctof" => {
            validate_arg_count(node, "ctof", 1)?;
            let arg = eval(node.child_by_field_name("argument").unwrap(), source)?; // .expect("unable to evaluate an argument");
            Ok(rlang_std::ctof(arg))
        }
        _ => Err("function not found".to_string()),
    }
}

fn validate_arg_count(
    node: tree_sitter::Node,
    fname: &str,
    expected: usize,
) -> Result<usize, String> {
    let mut cursor = node.walk();
    let n_args = node.children_by_field_name("argument", &mut cursor).count();

    let arg = if expected > 1 { "args" } else { "arg" };
    if n_args != expected {
        return Err(format!(
            "{} expects {} {}. Got {}",
            fname, expected, arg, n_args
        ));
    }

    Ok(n_args)
}
