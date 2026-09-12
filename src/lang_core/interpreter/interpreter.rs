use crate::lang_core::interpreter::runtime_value::RuntimeValue;
use crate::lang_core::parser::ast::{Node, NodeKind};

pub struct Interpreter {
    pub ast: Vec<Node>
}

impl Interpreter {
    pub fn new(ast: Vec<Node>) -> Interpreter {
        Interpreter { ast }
    }


    fn display_value(v: &RuntimeValue) -> String {
        match v {
            RuntimeValue::Int(i)   => i.to_string(),
            RuntimeValue::Float(f) => f.to_string(),
            RuntimeValue::String(s) => s.clone(),
        }
    }

    fn type_error(op: &str, a: &RuntimeValue, b: &RuntimeValue) -> RuntimeValue {
        eprintln!("Type error: cannot apply `{}` to {:?} and {:?}", op, a, b);
        RuntimeValue::Int(0)
    }

    fn apply_binop(&self, op: &str, l: RuntimeValue, r: RuntimeValue) -> RuntimeValue {
        use RuntimeValue::*;

        match op {
            "+" => match (l, r) {
                (Int(a), Int(b))       => Int(a + b),
                (Float(a), Float(b))   => Float(a + b),
                (Int(a), Float(b))     => Float(a as f64 + b),
                (Float(a), Int(b))     => Float(a + b as f64),
                (String(a), String(b)) => String(a + &b),
                (a, b) => Interpreter::type_error("+", &a, &b),
            },
            "-" => match (l, r) {
                (Int(a), Int(b))     => Int(a - b),
                (Float(a), Float(b)) => Float(a - b),
                (Int(a), Float(b))   => Float(a as f64 - b),
                (Float(a), Int(b))   => Float(a - b as f64),
                (a, b) => Interpreter::type_error("-", &a, &b),
            },
            "*" => match (l, r) {
                (Int(a), Int(b))     => Int(a * b),
                (Float(a), Float(b)) => Float(a * b),
                (Int(a), Float(b))   => Float(a as f64 * b),
                (Float(a), Int(b))   => Float(a * b as f64),
                // бонус: строка * число
                (String(s), Int(n)) if n >= 0 => String(s.repeat(n as usize)),
                (Int(n), String(s)) if n >= 0 => String(s.repeat(n as usize)),
                (a, b) => Interpreter::type_error("*", &a, &b),
            },
            "/" => match (l, r) {
                (Int(a), Int(b)) if b != 0     => Int(a / b),
                (Float(a), Float(b))           => Float(a / b),
                (Int(a), Float(b))             => Float(a as f64 / b),
                (Float(a), Int(b))             => Float(a / b as f64),
                (a, b) => Interpreter::type_error("/", &a, &b),
            },
            _ => RuntimeValue::Int(0),
        }
    }
    pub fn eval(&self, node: Node) -> RuntimeValue {
        match node {
            Node { kind: NodeKind::Int(val), .. } => RuntimeValue::Int(val),
            Node { kind: NodeKind::Float(val), .. } => RuntimeValue::Float(val),
            Node { kind: NodeKind::String(val), .. } => RuntimeValue::String(val),
            Node { kind: NodeKind::FuncCall(name, args), .. } => {
                if name != "cout" {
                    return RuntimeValue::Int(0);
                }
                let values: Vec<RuntimeValue> = args.into_iter()
                    .map(|v| Interpreter::eval(self, v))
                    .collect();

                let s = values.iter()
                    .map(|v| Interpreter::display_value(v))
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{}", s);

                RuntimeValue::Int(0)
            }
            Node { kind: NodeKind::BOP(op, left, right), .. } => {
                let l = Interpreter::eval(self, *left);
                let r = Interpreter::eval(self, *right);
                Interpreter::apply_binop(self, &op, l, r)
            }
        }
    }

    pub fn run(&self) {
        for node in &self.ast {
            Interpreter::eval(self, node.clone());
        }
    }
}