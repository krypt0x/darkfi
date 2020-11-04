#![allow(non_snake_case)]
use bls12_381::Scalar;
use sapvi::bls_extensions::BlsStringConversion;
use sapvi::serial::{Decodable, Encodable};
use simplelog::*;
use std::fs;
use std::fs::File;
use std::rc::Rc;
use std::time::Instant;
//use std::collections::HashMap;
use fnv::FnvHashMap;
use itertools::Itertools;
use sapvi::vm::{
    AllocType, ConstraintInstruction, CryptoOperation, VariableIndex, VariableRef, ZKVirtualMachine,
};

#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate fnv;
extern crate itertools;
extern crate regex;

#[macro_use]
mod types;
use crate::types::MalErr::{ErrMalVal, ErrString};
use crate::types::MalVal::{
    Add, Bool, Func, Hash, Lc0, Lc1, Lc2, List, MalFunc, Nil, Str, Sub, Sym, Vector, Zk,
};
use crate::types::ZKCircuit;
use crate::types::{error, format_error, MalArgs, MalErr, MalRet, MalVal};
mod env;
mod printer;
mod reader;
use crate::env::{env_bind, env_find, env_get, env_new, env_set, env_sets, Env};
#[macro_use]
mod core;

// read
fn read(str: &str) -> MalRet {
    reader::read_str(str.to_string())
}

// eval

fn qq_iter(elts: &MalArgs) -> MalVal {
    let mut acc = list![];
    for elt in elts.iter().rev() {
        if let List(v, _) = elt {
            if v.len() == 2 {
                if let Sym(ref s) = v[0] {
                    if s == "splice-unquote" {
                        acc = list![Sym("concat".to_string()), v[1].clone(), acc];
                        continue;
                    }
                }
            }
        }
        acc = list![Sym("cons".to_string()), quasiquote(&elt), acc];
    }
    return acc;
}

fn quasiquote(ast: &MalVal) -> MalVal {
    match ast {
        List(v, _) => {
            if v.len() == 2 {
                if let Sym(ref s) = v[0] {
                    if s == "unquote" {
                        return v[1].clone();
                    }
                }
            }
            return qq_iter(&v);
        }
        Vector(v, _) => return list![Sym("vec".to_string()), qq_iter(&v)],
        Hash(_, _) | Sym(_) => return list![Sym("quote".to_string()), ast.clone()],
        _ => ast.clone(),
    }
}

fn is_macro_call(ast: &MalVal, env: &Env) -> Option<(MalVal, MalArgs)> {
    match ast {
        List(v, _) => match v[0] {
            Sym(ref s) => match env_find(env, s) {
                Some(e) => match env_get(&e, &v[0]) {
                    Ok(f @ MalFunc { is_macro: true, .. }) => Some((f, v[1..].to_vec())),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn macroexpand(mut ast: MalVal, env: &Env) -> (bool, MalRet) {
    let mut was_expanded = false;
    while let Some((mf, args)) = is_macro_call(&ast, env) {
        //println!("macroexpand 1: {:?}", ast);
        ast = match mf.apply(args) {
            Err(e) => return (false, Err(e)),
            Ok(a) => a,
        };
        //println!("macroexpand 2: {:?}", ast);
        was_expanded = true;
    }
    ((was_expanded, Ok(ast)))
}

fn eval_ast(ast: &MalVal, env: &Env) -> MalRet {
    match ast {
        Sym(_) => Ok(env_get(&env, &ast)?),
        List(v, _) => {
            let mut lst: MalArgs = vec![];
            for a in v.iter() {
                lst.push(eval(a.clone(), env.clone())?)
            }
            Ok(list!(lst))
        }
        Vector(v, _) => {
            let mut lst: MalArgs = vec![];
            for a in v.iter() {
                lst.push(eval(a.clone(), env.clone())?)
            }
            Ok(vector!(lst))
        }
        Hash(hm, _) => {
            let mut new_hm: FnvHashMap<String, MalVal> = FnvHashMap::default();
            for (k, v) in hm.iter() {
                new_hm.insert(k.to_string(), eval(v.clone(), env.clone())?);
            }
            Ok(Hash(Rc::new(new_hm), Rc::new(Nil)))
        }
        _ => Ok(ast.clone()),
    }
}

fn eval(mut ast: MalVal, mut env: Env) -> MalRet {
    let ret: MalRet;

    'tco: loop {
        ret = match ast.clone() {
            List(l, _) => {
                if l.len() == 0 {
                    return Ok(ast);
                }
                match macroexpand(ast.clone(), &env) {
                    (true, Ok(new_ast)) => {
                        ast = new_ast;
                        continue 'tco;
                    }
                    (_, Err(e)) => return Err(e),
                    _ => (),
                }

                if l.len() == 0 {
                    return Ok(ast);
                }
                let a0 = &l[0];
                match a0 {
                    Sym(ref a0sym) if a0sym == "def!" => {
                        env_set(&env, l[1].clone(), eval(l[2].clone(), env.clone())?)
                    }
                    Sym(ref a0sym) if a0sym == "let*" => {
                        env = env_new(Some(env.clone()));
                        let (a1, a2) = (l[1].clone(), l[2].clone());
                        match a1 {
                            List(ref binds, _) | Vector(ref binds, _) => {
                                for (b, e) in binds.iter().tuples() {
                                    match b {
                                        Sym(_) => {
                                            let _ = env_set(
                                                &env,
                                                b.clone(),
                                                eval(e.clone(), env.clone())?,
                                            );
                                        }
                                        _ => {
                                            return error("let* with non-Sym binding");
                                        }
                                    }
                                }
                            }
                            _ => {
                                return error("let* with non-List bindings");
                            }
                        };
                        ast = a2;
                        continue 'tco;
                    }
                    Sym(ref a0sym) if a0sym == "quote" => Ok(l[1].clone()),
                    Sym(ref a0sym) if a0sym == "quasiquoteexpand" => Ok(quasiquote(&l[1])),
                    Sym(ref a0sym) if a0sym == "quasiquote" => {
                        ast = quasiquote(&l[1]);
                        continue 'tco;
                    }
                    Sym(ref a0sym) if a0sym == "defmacro!" => {
                        let (a1, a2) = (l[1].clone(), l[2].clone());
                        let r = eval(a2, env.clone())?;
                        match r {
                            MalFunc {
                                eval,
                                ast,
                                env,
                                params,
                                ..
                            } => Ok(env_set(
                                &env,
                                a1.clone(),
                                MalFunc {
                                    eval: eval,
                                    ast: ast.clone(),
                                    env: env.clone(),
                                    params: params.clone(),
                                    is_macro: true,
                                    meta: Rc::new(Nil),
                                },
                            )?),
                            _ => error("set_macro on non-function"),
                        }
                    }
                    Sym(ref a0sym) if a0sym == "macroexpand" => {
                        match macroexpand(l[1].clone(), &env) {
                            (_, Ok(new_ast)) => Ok(new_ast),
                            (_, e) => return e,
                        }
                    }
                    Sym(ref a0sym) if a0sym == "try*" => match eval(l[1].clone(), env.clone()) {
                        Err(ref e) if l.len() >= 3 => {
                            let exc = match e {
                                ErrMalVal(mv) => mv.clone(),
                                ErrString(s) => Str(s.to_string()),
                            };
                            match l[2].clone() {
                                List(c, _) => {
                                    let catch_env = env_bind(
                                        Some(env.clone()),
                                        list!(vec![c[1].clone()]),
                                        vec![exc],
                                    )?;
                                    eval(c[2].clone(), catch_env)
                                }
                                _ => error("invalid catch block"),
                            }
                        }
                        res => res,
                    },
                    Sym(ref a0sym) if a0sym == "do" => {
                        match eval_ast(&list!(l[1..l.len() - 1].to_vec()), &env)? {
                            List(_, _) => {
                                ast = l.last().unwrap_or(&Nil).clone();
                                continue 'tco;
                            }
                            _ => error("invalid do form"),
                        }
                    }
                    Sym(ref a0sym) if a0sym == "if" => {
                        let cond = eval(l[1].clone(), env.clone())?;
                        match cond {
                            Bool(false) | Nil if l.len() >= 4 => {
                                ast = l[3].clone();
                                continue 'tco;
                            }
                            Bool(false) | Nil => Ok(Nil),
                            _ if l.len() >= 3 => {
                                ast = l[2].clone();
                                continue 'tco;
                            }
                            _ => Ok(Nil),
                        }
                    }
                    Sym(ref a0sym) if a0sym == "zkcons!" => {
                        let (a1, a2) = (l[1].clone(), l[2].clone());
                        let value = eval_ast(&a2, &env)?;
                        match value {
                            List(ref el, _)  => {
                              zkcons_eval(el.to_vec(), &a1, &env);
                            }
                            _ => println!("invalid format"),
                        }
                        Ok(Nil)
                    }
                    Sym(ref a0sym) if a0sym == "defzk!" => {
                        let (a1, a2) = (l[1].clone(), l[2].clone());
                        let circuit = zk_circuit_create(&a1, &env);
                        let val = types::MalVal::Zk(circuit.clone());
                        env_set(&env, a1.clone(), val.clone());
                        Ok(val.clone())
                    }
                    Sym(ref a0sym) if a0sym == "fn*" => {
                        let (a1, a2) = (l[1].clone(), l[2].clone());
                        Ok(MalFunc {
                            eval: eval,
                            ast: Rc::new(a2),
                            env: env,
                            params: Rc::new(a1),
                            is_macro: false,
                            meta: Rc::new(Nil),
                        })
                    }
                    Sym(ref a0sym) if a0sym == "eval" => {
                        ast = eval(l[1].clone(), env.clone())?;
                        while let Some(ref e) = env.clone().outer {
                            env = e.clone();
                        }
                        continue 'tco;
                    }
                    _ => match eval_ast(&ast, &env)? {
                        List(ref el, _) => {
                            let ref f = el[0].clone();
                            let args = el[1..].to_vec();
                            match f {
                                Func(_, _) => f.apply(args),
                                MalFunc {
                                    ast: mast,
                                    env: menv,
                                    params,
                                    ..
                                } => {
                                    let a = &**mast;
                                    let p = &**params;
                                    env = env_bind(Some(menv.clone()), p.clone(), args)?;
                                    ast = a.clone();
                                    continue 'tco;
                                }
                                _ => {
                                    Ok(Nil)
                                    //error("call non-function")
                                }
                            }
                        }
                        _ => error("expected a list"),
                    },
                }
            }
            _ => eval_ast(&ast, &env),
        };

        break;
    } // end 'tco loop

    ret
}

fn zk_circuit_create(a1: &MalVal, env: &Env) -> ZKCircuit {
    let zk_circuit = ZKCircuit {
        name: a1.pr_str(true),
        constraints: Vec::new(),
        private: Vec::new(),
        public: Vec::new(),
    };
    zk_circuit
}

fn zkcons_eval(elements: Vec<MalVal>, a1: &MalVal, env: &Env) -> MalRet {
    let mut zk = match env_get(&env, &a1).ok().unwrap() {
        Zk(v) => v,
        n => zk_circuit_create(a1, env),
    };
    
    for b in elements.iter() {
        match b {
            Add(b1, b2) => {
                zk.private
                    .push(Scalar::from_string(&b2.pr_str(false).to_string()));
                let const_a: ConstraintInstruction = match b1.as_ref() {
                    Lc0 => ConstraintInstruction::Lc0Add(zk.private.len()),
                    Lc1 => ConstraintInstruction::Lc1Add(zk.private.len()),
                    Lc2 => ConstraintInstruction::Lc2Add(zk.private.len()),
                    _ => ConstraintInstruction::Lc0Add(0),
                };
                zk.constraints.push(const_a);
                env_set(&env, a1.clone(), types::MalVal::Zk(zk.clone()));
                println!("{:?}", a1.clone());
                println!("{:?}", zk.clone());
            }
            Sub(b1, b2) => {
                zk.private
                    .push(Scalar::from_string(&b2.pr_str(false).to_string()));
                let const_a: ConstraintInstruction = match b1.as_ref() {
                    Lc0 => ConstraintInstruction::Lc0Sub(zk.private.len()),
                    Lc1 => ConstraintInstruction::Lc1Add(zk.private.len()),
                    Lc2 => ConstraintInstruction::Lc2Add(zk.private.len()),
                    _ => ConstraintInstruction::Lc0Add(0),
                };
                zk.constraints.push(const_a);
                env_set(&env, a1.clone(), types::MalVal::Zk(zk.clone()));
            }
            val => println!("not match"),
        }
    }
    Ok(Nil)
}

// print
fn print(ast: &MalVal) -> String {
    ast.pr_str(true)
}

fn rep(str: &str, env: &Env) -> Result<String, MalErr> {
    let ast = read(str)?;
    let exp = eval(ast, env.clone())?;
    Ok(print(&exp))
}

fn main() -> Result<(), ()> {
    let matches = clap_app!(zklisp =>
        (version: "0.1.0")
        (author: "Roberto Santacroce Martins <miles.chet@gmail.com>")
        (about: "A Lisp Interpreter for Zero Knowledge Virtual Machine")
        (@subcommand load =>
            (about: "Load the file into the interpreter")
            (@arg FILE: +required "Lisp Contract filename")
        )
    )
    .get_matches();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();

    match matches.subcommand() {
        Some(("load", matches)) => {
            let file: String = matches.value_of("FILE").unwrap().parse().unwrap();
            repl_load(file);
        }
        _ => {
            eprintln!("error: Invalid subcommand invoked");
            std::process::exit(-1);
        }
    }

    Ok(())
}

fn repl_load(file: String) -> Result<(), ()> {
    let repl_env = env_new(None);
    for (k, v) in core::ns() {
        env_sets(&repl_env, k, v);
    }
    let _ = rep("(def! *host-language* \"rust\")", &repl_env);
    let _ = rep("(def! not (fn* (a) (if a false true)))", &repl_env);
    let _ = rep(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\nnil)\")))))",
        &repl_env,
    );
    let _ = rep("(defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw \"odd number of forms to cond\")) (cons 'cond (rest (rest xs)))))))", &repl_env);
    match rep(&format!("(load-file \"{}\")", file), &repl_env) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("Error: {}", format_error(e));
            std::process::exit(1);
        }
    }
    Ok(())
}
