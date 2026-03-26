use im::HashMap;
use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnOp { Add1, Sub1, Negate, IsNum, IsBool }

#[derive(Debug, Clone, PartialEq, Eq)]
enum BinOp { Plus, Minus, Times, Less, Greater, LessEq, GreaterEq, Equal }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Num(i32),
    Bool(bool),
    Input,
    Var(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
}

fn new_label(l: &mut i32, s: &str) -> String {
    let current = *l;
    *l += 1;
    format!("{}_{}", s, current)
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(name)), e] => {
                if matches!(name.as_str(), "let" | "add1" | "sub1" | "if" | "loop" | "break" | "set!" | "block" | "true" | "false" | "input") {
                    panic!("Invalid");
                }
                (name.to_string(), parse_expr(e))
            }
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        Sexp::Atom(S(name)) => match name.as_str() {
            "true" => Expr::Bool(true),
            "false" => Expr::Bool(false),
            "input" => Expr::Input,
            "let" | "add1" | "sub1" | "if" | "loop" | "break" | "set!" | "block" => panic!("Invalid"),
            _ => Expr::Var(name.to_string()),
        },
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(UnOp::Add1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(UnOp::Sub1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "negate" => Expr::UnOp(UnOp::Negate, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(UnOp::IsNum, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(UnOp::IsBool, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(BinOp::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(BinOp::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(BinOp::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(BinOp::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(BinOp::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(BinOp::LessEq, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(BinOp::GreaterEq, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(BinOp::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), Sexp::List(bindings), body] if op == "let" => {
                if bindings.is_empty() { panic!("Invalid"); }
                let parsed_bindings: Vec<(String, Expr)> = bindings.iter().map(parse_bind).collect();
                Expr::Let(parsed_bindings, Box::new(parse_expr(body)))
            }
            [Sexp::Atom(S(op)), cond, thn, els] if op == "if" => {
                Expr::If(Box::new(parse_expr(cond)), Box::new(parse_expr(thn)), Box::new(parse_expr(els)))
            }
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => {
                if exprs.is_empty() { panic!("Invalid"); }
                Expr::Block(exprs.iter().map(parse_expr).collect())
            }
            [Sexp::Atom(S(op)), body] if op == "loop" => Expr::Loop(Box::new(parse_expr(body))),
            [Sexp::Atom(S(op)), val] if op == "break" => Expr::Break(Box::new(parse_expr(val))),
            [Sexp::Atom(S(op)), Sexp::Atom(S(name)), val] if op == "set!" => Expr::Set(name.to_string(), Box::new(parse_expr(val))),
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

fn compile_expr(e: &Expr, env: &HashMap<String, i32>, si: i32, l: &mut i32, break_target: &Option<String>) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", (*n as i64) << 1),
        Expr::Bool(b) => format!("mov rax, {}", if *b { 3 } else { 1 }),
        Expr::Input => "mov rax, rdi".to_string(),
        Expr::Var(name) => match env.get(name) {
            Some(offset) => format!("mov rax, [rsp - {}]", offset),
            None => panic!("Unbound variable identifier {}", name),
        },
        Expr::Set(name, val) => {
            let val_code = compile_expr(val, env, si, l, break_target);
            match env.get(name) {
                Some(offset) => format!("{}\nmov [rsp - {}], rax", val_code, offset),
                None => panic!("Unbound variable identifier {}", name),
            }
        }
        Expr::UnOp(op, subexpr) => {
            let expr_code = compile_expr(subexpr, env, si, l, break_target);
            let check_num = "mov rbx, rax\nand rbx, 1\ncmp rbx, 0\njne error_not_num";
            let op_code = match op {
                UnOp::Add1 => format!("{}\nadd rax, 2\njo error_overflow", check_num),
                UnOp::Sub1 => format!("{}\nsub rax, 2\njo error_overflow", check_num),
                UnOp::Negate => format!("{}\nneg rax\njo error_overflow", check_num),
                UnOp::IsNum => "mov rbx, rax\nand rbx, 1\nshl rbx, 1\nxor rbx, 3\nmov rax, rbx".to_string(),
                UnOp::IsBool => "mov rbx, rax\nand rbx, 1\nshl rbx, 1\nadd rbx, 1\nmov rax, rbx".to_string(),
            };
            format!("{}\n{}", expr_code, op_code)
        }
        Expr::BinOp(op, e1, e2) => {
            let e1_code = compile_expr(e1, env, si, l, break_target);
            let offset = si * 8;
            let save_e1 = format!("mov [rsp - {}], rax", offset);
            let e2_code = compile_expr(e2, env, si + 1, l, break_target);
            
            let check_nums = format!("mov rbx, rax\nor rbx, [rsp - {}]\nand rbx, 1\ncmp rbx, 0\njne error_not_num", offset);
            
            let op_code = match op {
                BinOp::Plus => format!("{}\nadd rax, [rsp - {}]\njo error_overflow", check_nums, offset),
                BinOp::Minus => format!("{}\nmov rbx, rax\nmov rax, [rsp - {}]\nsub rax, rbx\njo error_overflow", check_nums, offset),
                BinOp::Times => format!("{}\nsar rax, 1\nimul rax, [rsp - {}]\njo error_overflow", check_nums, offset),
                BinOp::Equal => {
                    let cmp = format!("mov rbx, rax\nxor rbx, [rsp - {}]\ntest rbx, 1\njne error_not_num", offset); // Error if types differ
                    format!("{}\ncmp rax, [rsp - {}]\nmov rax, 1\nmov rbx, 3\ncmove rax, rbx", cmp, offset)
                }
                BinOp::Less => format!("{}\ncmp [rsp - {}], rax\nmov rax, 1\nmov rbx, 3\ncmovl rax, rbx", check_nums, offset),
                BinOp::Greater => format!("{}\ncmp [rsp - {}], rax\nmov rax, 1\nmov rbx, 3\ncmovg rax, rbx", check_nums, offset),
                BinOp::LessEq => format!("{}\ncmp [rsp - {}], rax\nmov rax, 1\nmov rbx, 3\ncmovle rax, rbx", check_nums, offset),
                BinOp::GreaterEq => format!("{}\ncmp [rsp - {}], rax\nmov rax, 1\nmov rbx, 3\ncmovge rax, rbx", check_nums, offset),
            };
            format!("{}\n{}\n{}\n{}", e1_code, save_e1, e2_code, op_code)
        }
        Expr::Let(bindings, body) => {
            let mut current_env = env.clone();
            let mut current_si = si;
            let mut instrs = String::new();
            let mut seen_names = Vec::new();
            
            for (name, val_expr) in bindings {
                if seen_names.contains(name) { panic!("Duplicate binding"); }
                seen_names.push(name.clone());
                
                let val_code = compile_expr(val_expr, &current_env, current_si, l, break_target);
                let offset = current_si * 8;
                instrs.push_str(&format!("{}\nmov [rsp - {}], rax\n", val_code, offset));
                current_env = current_env.update(name.clone(), offset);
                current_si += 1;
            }
            let body_code = compile_expr(body, &current_env, current_si, l, break_target);
            format!("{}{}", instrs, body_code)
        }
        Expr::If(cond, thn, els) => {
            let end_label = new_label(l, "if_end");
            let else_label = new_label(l, "if_else");
            let cond_code = compile_expr(cond, env, si, l, break_target);
            let thn_code = compile_expr(thn, env, si, l, break_target);
            let els_code = compile_expr(els, env, si, l, break_target);
            
            format!("{}\ncmp rax, 1\nje {}\n{}\njmp {}\n{}:\n{}\n{}:", 
                    cond_code, else_label, thn_code, end_label, else_label, els_code, end_label)
        }
        Expr::Block(exprs) => {
            exprs.iter().map(|e| compile_expr(e, env, si, l, break_target)).collect::<Vec<_>>().join("\n")
        }
        Expr::Loop(body) => {
            let loop_start = new_label(l, "loop_start");
            let loop_end = new_label(l, "loop_end");
            let body_code = compile_expr(body, env, si, l, &Some(loop_end.clone()));
            format!("{}:\n{}\njmp {}\n{}:", loop_start, body_code, loop_start, loop_end)
        }
        Expr::Break(val) => {
            match break_target {
                Some(label) => {
                    let val_code = compile_expr(val, env, si, l, break_target);
                    format!("{}\njmp {}", val_code, label)
                }
                None => panic!("break outside of loop"),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
        std::process::exit(1);
    }
    
    let mut in_contents = String::new();
    File::open(&args[1])?.read_to_string(&mut in_contents)?;

    let expr = parse_expr(&parse(&in_contents).unwrap());
    let mut labels = 0;
    let result = compile_expr(&expr, &HashMap::new(), 2, &mut labels, &None); 
    
    let asm_program = format!("
section .text
extern snek_error
global our_code_starts_here

error_not_num:
  mov rdi, 1
  call snek_error

error_overflow:
  mov rdi, 2
  call snek_error

our_code_starts_here:
  {}
  ret
", result);

    File::create(&args[2])?.write_all(asm_program.as_bytes())?;
    Ok(())
}