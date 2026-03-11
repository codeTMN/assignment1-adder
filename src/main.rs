use im::HashMap;
use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op1 { Add1, Sub1 }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op2 { Plus, Minus, Times }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i32),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
}

// Helper function to parse a single let binding, e.g., (x 5)
fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(name)), e] => {
                // The assignment strictly forbids using reserved words as variable names
                if name == "let" || name == "add1" || name == "sub1" {
                    panic!("Invalid"); 
                }
                (name.to_string(), parse_expr(e))
            }
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

// 2. Parser: Converts S-expressions into our AST
fn parse_expr(s: &Sexp) -> Expr {
    match s {
        // Parse numbers
        Sexp::Atom(I(n)) => Expr::Number(i32::try_from(*n).unwrap()),
        
        // Parse identifiers (variables)
        Sexp::Atom(S(name)) => {
            if name == "let" || name == "add1" || name == "sub1" {
                panic!("Invalid");
            }
            Expr::Id(name.to_string())
        }
        
        // Parse lists (operations and let bindings)
        Sexp::List(vec) => match &vec[..] {
            // Unary operators
            [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
            
            // Binary operators
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
            
            // Let bindings
            [Sexp::Atom(S(op)), Sexp::List(bindings), body] if op == "let" => {
                if bindings.is_empty() {
                    panic!("Invalid"); // Must have at least one binding
                }
                let mut parsed_bindings = Vec::new();
                for b in bindings {
                    parsed_bindings.push(parse_bind(b));
                }
                Expr::Let(parsed_bindings, Box::new(parse_expr(body)))
            }
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

// 3. Code Generator: Converts AST into x86-64 assembly using the Stack
fn compile_to_instrs(e: &Expr, env: &HashMap<String, i32>, si: i32) -> String {
    match e {
        Expr::Number(n) => format!("mov rax, {}", n),
        
        Expr::Id(name) => {
            // Look up the variable in our environment HashMap
            match env.get(name) {
                Some(offset) => format!("mov rax, [rsp - {}]", offset),
                None => panic!("Unbound variable identifier {}", name),
            }
        }
        
        Expr::UnOp(op, subexpr) => {
            let expr_code = compile_to_instrs(subexpr, env, si);
            let op_code = match op {
                Op1::Add1 => "add rax, 1",
                Op1::Sub1 => "sub rax, 1",
            };
            format!("{}\n{}", expr_code, op_code)
        }
        
        Expr::BinOp(op, e1, e2) => {
            let e1_code = compile_to_instrs(e1, env, si);
            let offset = si * 8; // Calculate byte offset for this stack index
            let save_e1 = format!("mov [rsp - {}], rax", offset); // Save left side to memory
            let e2_code = compile_to_instrs(e2, env, si + 1); // Compile right side with next stack index
            
            let op_code = match op {
                Op2::Plus => format!("add rax, [rsp - {}]", offset),
                Op2::Minus => format!("mov rbx, rax\nmov rax, [rsp - {}]\nsub rax, rbx", offset),
                Op2::Times => format!("imul rax, [rsp - {}]", offset),
            };
            
            format!("{}\n{}\n{}\n{}", e1_code, save_e1, e2_code, op_code)
        }
        
        Expr::Let(bindings, body) => {
            let mut current_env = env.clone();
            let mut current_si = si;
            let mut instrs = String::new();
            let mut seen_names = Vec::new();
            
            for (name, val_expr) in bindings {
                // Check for duplicates in the same let block
                if seen_names.contains(name) {
                    panic!("Duplicate binding");
                }
                seen_names.push(name.clone());
                
                let val_code = compile_to_instrs(val_expr, &current_env, current_si);
                let offset = current_si * 8;
                let save_val = format!("mov [rsp - {}], rax", offset);
                
                if !instrs.is_empty() { instrs.push_str("\n"); }
                instrs.push_str(&val_code);
                instrs.push_str("\n");
                instrs.push_str(&save_val);
                
                // Update environment with the new variable and its memory offset
                current_env = current_env.update(name.clone(), offset);
                current_si += 1;
            }
            
            let body_code = compile_to_instrs(body, &current_env, current_si);
            if !instrs.is_empty() { instrs.push_str("\n"); }
            instrs.push_str(&body_code);
            
            instrs
        }
    }
}

// 4. Main function: Orchestrates file reading, compiling, and file writing
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
        std::process::exit(1);
    }
    
    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = parse_expr(&parse(&in_contents).unwrap());
    
    // Initialize an empty environment and start stack index at 2 (offset 16)
    let empty_env = HashMap::new();
    let result = compile_to_instrs(&expr, &empty_env, 2); 
    
    let asm_program = format!("
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
", result);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}