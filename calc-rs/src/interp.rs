use std::collections::HashMap;

use crate::{
    calc_std,
    nodes::{self, NodeType, Operator},
};

pub struct Interp {
    env: HashMap<String, f64>,
    source: String,
}

impl Interp {
    pub fn new() -> Interp {
        Interp {
            env: HashMap::new(),
            source: String::new(),
        }
    }

    pub fn run(
        self: &mut Interp,
        source: String,
        root: tree_sitter::Node,
        is_repl: bool,
        debug: bool,
    ) -> Result<(), String> {
        if root.has_error() {
            return Err("Invalid input".to_string());
        }

        self.source = source;

        let mut cursor = root.walk();
        for child in root.children(&mut cursor) {
            let res = self.eval(child);
            match res {
                Ok(r) => {
                    if debug {
                        println!("-> {}: {}", child.kind(), &self.source[child.byte_range()]);
                        println!("ans: {}", r);
                    }
                    let mut pref = "";
                    let mut suf = "";
                    if is_repl {
                        pref = " = ";
                        suf = "\n";
                    }
                    println!("{}{}{}", pref, r, suf);
                }
                Err(s) => println!("{}", s),
            }
        }
        Ok(())
    }

    fn eval(self: &mut Interp, node: tree_sitter::Node) -> Result<f64, String> {
        if let Some(t) = nodes::resolve_kind(node) {
            match t {
                NodeType::Number => {
                    return Ok(self.source[node.byte_range()].parse::<f64>().unwrap());
                }
                NodeType::BinaryExpression => {
                    return self.eval_binary(node);
                }
                NodeType::ParenthesizedExpression => {
                    return self.eval(node.child_by_field_name("expression").unwrap());
                }
                NodeType::CallExpression => {
                    return self.eval_call(node);
                }
                NodeType::Assignment => {
                    return self.eval_assign(node);
                }
                NodeType::Identifier => {
                    let k = self.source[node.byte_range()].trim();
                    if let Some(val) = self.env.get(k) {
                        return Ok(*val);
                    }
                    return Ok(0.0);
                }
            }
        }
        return Err("Node not implemented.".to_string());
    }

    fn eval_binary(self: &mut Interp, node: tree_sitter::Node) -> Result<f64, String> {
        // println!("BIN: {}", &source[node.byte_range()]);
        let left = node.child_by_field_name("left").unwrap();
        let l = self.eval(left)?; //.expect("issue evaluating expression");

        let right = node.child_by_field_name("right").unwrap();
        let r = self.eval(right)?; //.expect("issue evaluating expression");

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

    fn eval_call(self: &mut Interp, node: tree_sitter::Node) -> Result<f64, String> {
        let func = node.child_by_field_name("function").unwrap();
        let name = &self.source[func.byte_range()];
        match name {
            "pow" => {
                self.validate_arg_count(node, "pow", 2)?;
                let mut cursor = node.walk();
                let mut children = node.children_by_field_name("argument", &mut cursor);

                let base = self.eval(children.next().unwrap())?; //.expect("unable to evaluate base argument");
                let exp = self.eval(children.next().unwrap())?; //.expect("unable to evaluate exp argument");
                Ok(base.powf(exp))
            }
            "sqrt" => {
                self.validate_arg_count(node, "sqrt", 1)?;
                let mut cursor = node.walk();
                let mut children = node.children_by_field_name("argument", &mut cursor);

                let arg = self.eval(children.next().unwrap())?; //.expect("unable to evaluate base argument");
                Ok(arg.sqrt())
            }
            "abs" => {
                self.validate_arg_count(node, "abs", 1)?;
                let mut cursor = node.walk();
                let mut children = node.children_by_field_name("argument", &mut cursor);

                let arg = self.eval(children.next().unwrap())?; //.expect("unable to evaluate base argument");
                Ok(arg.abs())
            }
            "round" => {
                self.validate_arg_count(node, "round", 1)?;
                let mut cursor = node.walk();
                let mut children = node.children_by_field_name("argument", &mut cursor);

                let arg = self.eval(children.next().unwrap())?; //.expect("unable to evaluate base argument");
                Ok(arg.round())
            }
            "ftoc" => {
                self.validate_arg_count(node, "ftoc", 1)?;

                let arg = self.eval(node.child_by_field_name("argument").unwrap())?; // .expect("unable to evaluate an argument");
                Ok(calc_std::ftoc(arg))
            }
            "ctof" => {
                self.validate_arg_count(node, "ctof", 1)?;
                let arg = self.eval(node.child_by_field_name("argument").unwrap())?; // .expect("unable to evaluate an argument");
                Ok(calc_std::ctof(arg))
            }
            "cad_usd" => {
                self.validate_arg_count(node, "cad_usd", 1)?;
                let arg = self.eval(node.child_by_field_name("argument").unwrap())?; // .expect("unable to evaluate an argument");
                calc_std::cad_usd(arg)
            }
            "usd_cad" => {
                self.validate_arg_count(node, "usd_cad", 1)?;
                let arg = self.eval(node.child_by_field_name("argument").unwrap())?; // .expect("unable to evaluate an argument");
                calc_std::usd_cad(arg)
            }
            _ => Err("function not found".to_string()),
        }
    }

    fn eval_assign(self: &mut Interp, node: tree_sitter::Node) -> Result<f64, String> {
        let id_node = node.child_by_field_name("identifier").unwrap();
        let id = self.source[id_node.byte_range()].to_string();

        let val = node.child_by_field_name("expression").unwrap();
        let val = self.eval(val)?;

        self.env.insert(id.to_string(), val);

        Ok(val)
    }

    fn validate_arg_count(
        self: &Interp,
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
}
