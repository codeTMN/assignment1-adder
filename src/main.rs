use im::HashMap;
use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    defns: Vec<Definition>,
    main: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Definition {
    name: String,
    params: Vec<String>,
    body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnOp { Add1, Sub1, Negate, IsNum, IsBool, Print }

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
    Call(String, Vec<Expr>),
}

fn new_label(l: &mut i32, s: &str) -> String {
    let current = *l;
    *l += 1;
    format!("{}_{}", s, current)
}

fn try_parse_defn(s: &Sexp) -> Option<Definition> {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(fun)), Sexp::List(signature), body] if fun == "fun" => {
                match &signature[..] {
                    [Sexp::Atom(S(name)), params @ ..] => {
                        let param_names: Vec<String> = params.iter().map(|p| {
                            match p {
                                Sexp::Atom(S(p_name)) => {
                                    if matches!(p_name.as_str(), "let"|"add1"|"sub1"|"if"|"loop"|"break"|"set!"|"block"|"true"|"false"|"input"|"fun"|"print") {
                                        panic!("Invalid parameter name");
                                    }
                                    p_name.clone()
                                },
                                _ => panic!("Invalid parameter"),
                            }
                        }).collect();
                        
                        let mut seen = std::collections::HashSet::new();
                        for p in &param_names {
                            if !seen.insert(p) { panic!("Duplicate parameter"); }
                        }
                        
                        Some(Definition {
                            name: name.clone(),
                            params: param_names,
                            body: parse_expr(body),
                        })
                    }
                    _ => panic!("Invalid function signature"),
                }
            }
            _ => None,
        },
        _ => None,
    }
}

fn parse_program(s: &Sexp) -> Program {
    match s {
        Sexp::List(items) => {
            let mut defns = vec![];
            let mut main_expr = None;
            for item in items {
                if let Some(defn) = try_parse_defn(item) {
                    if main_expr.is_some() { panic!("Function definition after main expression"); }
                    defns.push(defn);
                } else if main_expr.is_none() {
                    main_expr = Some(parse_expr(item));
                } else {
                    panic!("Multiple main expressions");
                }
            }
            Program {
                defns,
                main: main_expr.expect("No main expression"),
            }
        }
        _ => panic!("Invalid program"),
    }
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(name)), e] => {
                if matches!(name.as_str(), "let"|"add1"|"sub1"|"if"|"loop"|"break"|"set!"|"block"|"true"|"false"|"input"|"print") {
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
            "let"|"add1"|"sub1"|"if"|"loop"|"break"|"set!"|"block"|"print" => panic!("Invalid"),
            _ => Expr::Var(name.to_string()),
        },
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(UnOp::Add1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(UnOp::Sub1, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "negate" => Expr::UnOp(UnOp::Negate, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(UnOp::IsNum, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(UnOp::IsBool, Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "print" => Expr::UnOp(UnOp::Print, Box::new(parse_expr(e))),
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
            [Sexp::Atom(S(op)), cond, thn, els] if op == "if" => Expr::If(Box::new(parse_expr(cond)), Box::new(parse_expr(thn)), Box::new(parse_expr(els))),
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => {
                if exprs.is_empty() { panic!("Invalid"); }
                Expr::Block(exprs.iter().map(parse_expr).collect())
            }
            [Sexp::Atom(S(op)), body] if op == "loop" => Expr::Loop(Box::new(parse_expr(body))),
            [Sexp::Atom(S(op)), val] if op == "break" => Expr::Break(Box::new(parse_expr(val))),
            [Sexp::Atom(S(op)), Sexp::Atom(S(name)), val] if op == "set!" => Expr::Set(name.to_string(), Box::new(parse_expr(val))),
            [Sexp::Atom(S(name)), args @ ..] => {
                if matches!(name.as_str(), "let"|"add1"|"sub1"|"negate"|"isnum"|"isbool"|"if"|"loop"|"break"|"set!"|"block"|"true"|"false"|"input"|"print"|"+"|"-"|"*"|"<"|">"|"<="|">="|"=") {
                    panic!("Invalid function call name");
                }
                Expr::Call(name.to_string(), args.iter().map(parse_expr).collect())
            }
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

fn compile_expr(e: &Expr, env: &HashMap<String, i32>, si: i32, l: &mut i32, break_target: &Option<String>, funs: &HashMap<String, usize>) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", (*n as i64) << 1),
        Expr::Bool(b) => format!("mov rax, {}", if *b { 3 } else { 1 }),
        Expr::Input => "mov rax, rdi".to_string(),
        Expr::Var(name) => match env.get(name) {
            Some(offset) => {
                if *offset < 0 { format!("mov rax, [rbp + {}]", -offset) } // Parameter
                else { format!("mov rax, [rsp - {}]", offset) }             // Local
            }
            None => panic!("Unbound variable identifier {}", name),
        },
        Expr::Set(name, val) => {
            let val_code = compile_expr(val, env, si, l, break_target, funs);
            match env.get(name) {
                Some(offset) => {
                    if *offset < 0 { format!("{}\nmov [rbp + {}], rax", val_code, -offset) }
                    else { format!("{}\nmov [rsp - {}], rax", val_code, offset) }
                }
                None => panic!("Unbound variable identifier {}", name),
            }
        }
        Expr::UnOp(op, subexpr) => {
            let expr_code = compile_expr(subexpr, env, si, l, break_target, funs);
            if *op == UnOp::Print {
                // Protect memory by shifting rsp down before calling the C function
                let protect = si * 8;
                return format!("{}\nsub rsp, {}\nmov rdi, rax\nmov rbx, rsp\nand rsp, -16\ncall snek_print\nmov rsp, rbx\nadd rsp, {}", expr_code, protect, protect);
            }
            let check_num = "mov rbx, rax\nand rbx, 1\ncmp rbx, 0\njne error_not_num";
            let op_code = match op {
                UnOp::Add1 => format!("{}\nadd rax, 2\njo error_overflow", check_num),
                UnOp::Sub1 => format!("{}\nsub rax, 2\njo error_overflow", check_num),
                UnOp::Negate => format!("{}\nneg rax\njo error_overflow", check_num),
                UnOp::IsNum => "mov rbx, rax\nand rbx, 1\nshl rbx, 1\nxor rbx, 3\nmov rax, rbx".to_string(),
                UnOp::IsBool => "mov rbx, rax\nand rbx, 1\nshl rbx, 1\nadd rbx, 1\nmov rax, rbx".to_string(),
                _ => unreachable!(),
            };
            format!("{}\n{}", expr_code, op_code)
        }
        Expr::BinOp(op, e1, e2) => {
            let e1_code = compile_expr(e1, env, si, l, break_target, funs);
            let offset = si * 8;
            let save_e1 = format!("mov [rsp - {}], rax", offset);
            let e2_code = compile_expr(e2, env, si + 1, l, break_target, funs);
            let check_nums = format!("mov rbx, rax\nor rbx, [rsp - {}]\nand rbx, 1\ncmp rbx, 0\njne error_not_num", offset);
            let op_code = match op {
                BinOp::Plus => format!("{}\nadd rax, [rsp - {}]\njo error_overflow", check_nums, offset),
                BinOp::Minus => format!("{}\nmov rbx, rax\nmov rax, [rsp - {}]\nsub rax, rbx\njo error_overflow", check_nums, offset),
                BinOp::Times => format!("{}\nsar rax, 1\nimul rax, [rsp - {}]\njo error_overflow", check_nums, offset),
                BinOp::Equal => format!("mov rbx, rax\nxor rbx, [rsp - {}]\ntest rbx, 1\njne error_not_num\ncmp rax, [rsp - {}]\nmov rax, 1\nmov rbx, 3\ncmove rax, rbx", offset, offset),
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
                
                let val_code = compile_expr(val_expr, &current_env, current_si, l, break_target, funs);
                let offset = current_si * 8;
                instrs.push_str(&format!("{}\nmov [rsp - {}], rax\n", val_code, offset));
                current_env = current_env.update(name.clone(), offset);
                current_si += 1;
            }
            format!("{}{}", instrs, compile_expr(body, &current_env, current_si, l, break_target, funs))
        }
        Expr::If(cond, thn, els) => {
            let end_label = new_label(l, "if_end");
            let else_label = new_label(l, "if_else");
            format!("{}\ncmp rax, 1\nje {}\n{}\njmp {}\n{}:\n{}\n{}:", 
                compile_expr(cond, env, si, l, break_target, funs), else_label, 
                compile_expr(thn, env, si, l, break_target, funs), end_label, 
                else_label, compile_expr(els, env, si, l, break_target, funs), end_label)
        }
        Expr::Block(exprs) => exprs.iter().map(|e| compile_expr(e, env, si, l, break_target, funs)).collect::<Vec<_>>().join("\n"),
        Expr::Loop(body) => {
            let loop_start = new_label(l, "loop_start");
            let loop_end = new_label(l, "loop_end");
            format!("{}:\n{}\njmp {}\n{}:", loop_start, compile_expr(body, env, si, l, &Some(loop_end.clone()), funs), loop_start, loop_end)
        }
        Expr::Break(val) => match break_target {
            Some(label) => format!("{}\njmp {}", compile_expr(val, env, si, l, break_target, funs), label),
            None => panic!("break outside of loop"),
        },
        Expr::Call(name, args) => {
            match funs.get(name) {
                Some(arity) => if args.len() != *arity { panic!("Wrong number of arguments"); },
                None => panic!("Calling undefined function"),
            }
            
            let mut instrs = vec![];
            let mut current_si = si;
            
            for arg in args.iter() {
                instrs.push(compile_expr(arg, env, current_si, l, break_target, funs));
                instrs.push(format!("mov [rsp - {}], rax", current_si * 8));
                current_si += 1;
            }
            
            // Protect local variables by moving rsp completely out of the way before pushing
            let protect_size = current_si * 8;
            instrs.push(format!("sub rsp, {}", protect_size));
            
            let padding = if args.len() % 2 != 0 { 8 } else { 0 };
            if padding != 0 { instrs.push(format!("sub rsp, {}", padding)); }
            
            // Push arguments right-to-left dynamically
            let mut pushes = 0;
            for i in (0..args.len()).rev() {
                let offset = protect_size as i32 + padding as i32 + pushes - ((si + i as i32) * 8);
                instrs.push(format!("mov rax, [rsp + {}]", offset));
                instrs.push("push rax".to_string());
                pushes += 8;
            }
            
            instrs.push(format!("call fun_{}", name));
            
            // Cleanup the stack pointer completely
            let total_cleanup = (args.len() as i32 * 8) + padding as i32 + protect_size as i32;
            if total_cleanup > 0 { instrs.push(format!("add rsp, {}", total_cleanup)); }
            
            instrs.join("\n")
        }
    }
}

fn compile_defn(defn: &Definition, l: &mut i32, funs: &HashMap<String, usize>) -> String {
    let mut instrs = vec![];
    instrs.push(format!("fun_{}:", defn.name));
    instrs.push("push rbp".to_string());
    instrs.push("mov rbp, rsp".to_string());

    let mut env = HashMap::new();
    for (i, param) in defn.params.iter().enumerate() {
        env = env.update(param.clone(), -(16 + i as i32 * 8)); // Parameters map to negative offsets
    }

    instrs.push(compile_expr(&defn.body, &env, 2, l, &None, funs));
    instrs.push("mov rsp, rbp".to_string());
    instrs.push("pop rbp".to_string());
    instrs.push("ret".to_string());
    instrs.join("\n")
}

fn compile_program(prog: &Program) -> String {
    let mut l = 0;
    let mut asm = vec![];
    let mut funs = HashMap::new();

    for defn in &prog.defns {
        if funs.contains_key(&defn.name) { panic!("Duplicate function"); }
        funs.insert(defn.name.clone(), defn.params.len());
    }

    for defn in &prog.defns {
        asm.push(compile_defn(defn, &mut l, &funs));
    }

    asm.push("our_code_starts_here:".to_string());
    asm.push("push rbp".to_string());
    asm.push("mov rbp, rsp".to_string());

    asm.push(compile_expr(&prog.main, &HashMap::new(), 2, &mut l, &None, &funs));

    asm.push("mov rsp, rbp".to_string());
    asm.push("pop rbp".to_string());
    asm.push("ret".to_string());

    format!("
section .text
extern snek_error
extern snek_print
global our_code_starts_here

error_not_num:
  mov rdi, 1
  mov rbx, rsp
  and rsp, -16
  call snek_error
  mov rsp, rbx

error_overflow:
  mov rdi, 2
  mov rbx, rsp
  and rsp, -16
  call snek_error
  mov rsp, rbx

{}
", asm.join("\n\n"))
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
        std::process::exit(1);
    }
    
    let mut in_contents = String::new();
    File::open(&args[1])?.read_to_string(&mut in_contents)?;

    let wrapped = format!("({})", in_contents);
    let prog = parse_program(&parse(&wrapped).unwrap());
    
    let result = compile_program(&prog);
    File::create(&args[2])?.write_all(result.as_bytes())?;
    Ok(())
}