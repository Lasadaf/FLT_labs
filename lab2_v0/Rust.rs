#![allow(non_snake_case)]
#![allow(while_true)]
use std::process;

#[derive(Clone, PartialEq)]
struct Term {
    value: String,
    var: i32,
    left: Option<Box<Term>>,
    right: Option<Box<Term>>
}

#[derive(Clone)]
struct EqTerm {
    instances: i32,
    vars: Vec<Term>,
    terms: Vec<Term>
}

#[derive(Clone)]
struct Rule {
    left: Term,
    right: Term
}

fn strip_spaces(expr: String) -> String {
    let mut nexpr = "".to_string();
    for i in 0..expr.len() {
        if expr.chars().nth(i).unwrap() != ' ' {
            nexpr.push(expr.chars().nth(i).unwrap().clone());
        }
    }
    nexpr
}

fn parse_regex(expr: String, ptr: usize, consts: Vec<Term>, rule: bool, eps: Term) -> (Term, Vec<Term>, usize) {
    if ptr >= expr.len() || expr.chars().nth(ptr).unwrap() == '=' || expr.chars().nth(ptr).unwrap() == ')' || expr.chars().nth(ptr).unwrap() == '*' || expr.chars().nth(ptr).unwrap() == '|' || expr.chars().nth(ptr).unwrap() == '\n'{
        return (eps.clone(), consts, ptr);
    }
    if expr.chars().nth(ptr).unwrap() == '(' {
        let (left, consts, ptr) = parse_regex(expr.clone(), ptr.clone() + 1, consts.clone(), rule.clone(), eps.clone());
        if ptr >= expr.len() {
            println!("Нарушение синтаксиса: несбалансированные скобки");
            process::exit(0x0100);
        }
        if expr.chars().nth(ptr).unwrap() == ')' {
            let (mut unary, ptr) = parse_unary(expr.clone(), ptr.clone() + 1, eps.clone());
            if unary.clone() != eps {
                unary.left = Some(Box::<Term>::new(left.clone()));
                return (unary, consts, ptr);
            }
            return (left, consts, ptr);
        }
        let (mut binary, ptr) = parse_binary(expr.clone(), ptr.clone(), eps.clone());
        let (right, consts, ptr) = parse_regex(expr.clone(), ptr.clone(), consts.clone(), rule.clone(), eps.clone());
        binary.left = Some(Box::<Term>::new(left.clone()));
        binary.right = Some(Box::<Term>::new(right.clone()));
        if ptr >= expr.len() || expr.chars().nth(ptr).unwrap() != ')' {
            println!("Нарушение синтаксиса: несбалансированные скобки");
            process::exit(0x0100);
        }
        let (mut unary, ptr) = parse_unary(expr.clone(), ptr.clone() + 1, eps.clone());
        if unary.clone() != eps {
            unary.left = Some(Box::<Term>::new(binary.clone()));
            return (unary, consts, ptr);
        }
        return (binary, consts, ptr);
    }
    let (symbol, consts, ptr) = parse_symbol(expr.clone(), ptr.clone(), consts.clone(), rule, eps.clone());
    let (mut unary, ptr) = parse_unary(expr.clone(), ptr.clone(), eps.clone());
    if unary.clone() != eps {
        unary.left = Some(Box::<Term>::new(symbol.clone()));
        return (unary, consts, ptr);
    }
    (symbol, consts, ptr)
}

fn parse_binary(expr: String, ptr: usize, eps: Term) -> (Term, usize) {
    if ptr.clone() < expr.len() {
        if expr.chars().nth(ptr).unwrap() == '|' {
            let n = Term {
                value: "|".to_string(),
                var: 2,
                left: None,
                right: None
            };
            return (n, ptr + 1);
        }
        let n = Term {
            value: "++".to_string(),
            var: 2,
            left: None,
            right: None
        };
        return (n, ptr);
    }
    (eps.clone(), ptr)
}

fn parse_symbol(expr: String, ptr: usize, mut consts: Vec<Term>, rule: bool, eps: Term) -> (Term, Vec<Term>, usize) {
    if ptr < expr.len() {
        if !expr.chars().nth(ptr).unwrap().is_alphabetic() {
            println!("Нарушение синтаксиса: запрещенный символ");
            process::exit(0x0100);
        }
        let mut n: Term;
        if !rule {
            n = Term {
                value: expr.chars().nth(ptr).unwrap().to_string(),
                var: 0,
                left: None,
                right: None
            };
            consts.push(n.clone());
        }
        else {
            n = Term {
                value: expr.chars().nth(ptr).unwrap().to_string(),
                var: 1,
                left: None,
                right: None
            };
            for i in 0..consts.len() {
                if consts[i].value == expr.chars().nth(ptr).unwrap().to_string() {
                    n = Term {
                        value: expr.chars().nth(ptr).unwrap().to_string(),
                        var: 0,
                        left: None,
                        right: None
                    };
                }
            }
        }
        return (n, consts, ptr + 1);
    }
    (eps.clone(), consts, ptr)
}

fn parse_unary(expr: String, ptr: usize, eps: Term) -> (Term, usize) {
    if ptr < expr.len() {
        if expr.chars().nth(ptr).unwrap() == '*' {
            let n = Term {
                value: "*".to_string(),
                var: 2,
                left: None,
                right: None
            };
            return (n, ptr + 1);
        }
        return (eps.clone(), ptr);
    }
    (eps.clone(), ptr)
}

fn parse_rule(expr: String, ptr: usize, consts: Vec<Term>, rule: bool, eps: Term) -> (Term, Term) {
    let (left, _, ptr) = parse_regex(expr.clone(), ptr.clone(), consts.clone(), rule, eps.clone());
    if expr.chars().nth(ptr).unwrap() != '=' {
        println!("Нарушение синтаксиса: в одном из правил отсутствует знак равенства");
        process::exit(0x0100);
    }
    let (right, _, _) = parse_regex(expr.clone(), ptr.clone() + 1, consts.clone(), rule, eps.clone());
    (left, right)
}

fn reg_to_string(reg: Term) -> String {
    let mut out = "".to_string();
    if reg.value == "|" || reg.value == "++" || (reg.value == "*" && reg.left != None && *(reg.left.clone().unwrap()).value == "*".to_string()) {
        out.push('(');
    }
    if reg.left != None {
        out += &(reg_to_string(*reg.left.clone().unwrap()));
    }
    if reg.value == "*" && reg.left != None && *(reg.left.unwrap()).value == "*".to_string() {
        out.push(')');
    }
    out += &(reg.value.clone().to_string());
    if reg.right != None {
        out += &(reg_to_string(*reg.right.clone().unwrap()));
    }
    if reg.value == "|" || reg.value == "++" {
        out.push(')');
    }
    out
}

/*
fn print_U(U: Vec<EqTerm>) {
    for i in 0..U.len() {
        let mut out = "[".to_string();
        out += &(U[i].instances).to_string();
        out += &("] {").to_string();
        for j in 0..U[i].vars.len() {
            out += &(U[i].vars[j].clone().value).to_string();
            out.push(',');
            out.push(' ');
        }
        out.pop();
        out.pop();
        out += &("} = (").to_string();
        let mut empty = 1;
        for j in 0..U[i].terms.len() {
            out += &(reg_to_string(U[i].terms[j].clone())).to_string();
            out.push(',');
            out.push(' ');
            empty = 0;
        }
        if empty == 0 {
            out.pop();
            out.pop();
        }
        out.push(')');
        println!("{}", out);
    }
}
*/

fn insert_carefully (terms: Vec<Term>, consts: Vec<Term>) -> (Vec<Term>, bool) {
    if terms.len() == 0 {
        //println!("this is new");
        let first = reg_to_string(consts[0].clone());
        for i in 0..consts.len() {
            if reg_to_string(consts[i].clone()) != first {
                //println!("they were different");
                return (terms, true);
            }
        }
        //println!("new but OK");
        return (consts, false);
    }
    //println!("this is new to old");
    for i in 0..consts.len() {
        for j in 0..terms.len() {
            if reg_to_string(consts[i].clone()) != reg_to_string(terms[j].clone()) {
                //println!("new has conflict with old {} != {}", reg_to_string(consts[i].clone()), reg_to_string(terms[j].clone()));
                return (terms, true);
            }
        }
    }
    //println!("new is old");
    (terms, false)
}

fn conflicts (current: Vec<EqTerm>, vars: Vec<Term>, consts: Vec<Term>) -> bool {
    for i in 0..vars.len() {
        for j in 0..current.len() {
            for jj in 0..current[j].vars.len() {
                if vars[i].value == current[j].vars[jj].value {
                    for k in 0..consts.len() {
                        for jjj in 0..current[j].terms.len() {
                            if reg_to_string(current[j].terms[jjj].clone()) != reg_to_string(consts[k].clone()) {
                                //println!("conflict: {} != {}", reg_to_string(current[j].terms[jjj].clone()), reg_to_string(consts[k].clone()));
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn insert_terms(mut F: Vec<EqTerm>, vars: Vec<Term>, consts: Vec<Term>, mut current: Vec<EqTerm>) -> (Vec<EqTerm>, Vec<EqTerm>, bool) {
    let mut inserted = false;
    for i in 0..vars.len() {
        for j in 0..F.len() {
            for jj in 0..F[j].vars.len() {
                if F[j].vars[jj].value == vars[i].value {
                    //println!("found {} == {}", F[j].vars[jj].value.clone(), vars[i].value.clone());
                    if inserted {
                        //println!("double insert");
                        return (F, current, true);
                    }
                    let (a, err) = insert_carefully(F[j].terms.clone(), consts.clone());
                    F[j].terms = a.clone();
                    if err {
                        //println!("insert carefully failed");
                        return (F, current, true);
                    }
                    let err = conflicts(current.clone(), vars.clone(), consts.clone());
                    if err {
                        //println!("conflicting information");
                        return (F, current, true);
                    }
                    current.push(EqTerm {
                        instances: 0,
                        vars: vars.clone(),
                        terms: consts.clone()
                    });
                    inserted = true;
                }
            }
        }
    }
    if !inserted {
        //println!("this is a new entry");
        let err = conflicts(current.clone(), vars.clone(), consts.clone());
        if err {
            //println!("it was new, but had conflict");
            return (F, current, true);
        }
        current.push(EqTerm {
            instances: 0,
            vars: vars.clone(),
            terms: consts.clone()
        });
        F.push(EqTerm {
            instances: 0,
            vars: vars.clone(),
            terms: consts.clone()
        });
    }
    //println!("insert_terms passed");
    //println!("now we have F");
    //print_U(F.clone());
    //println!("now we have current");
    //print_U(current.clone());
    (F.clone(), current.clone(), false)
}

fn separateCF(terms: Vec<Term>, mut F: Vec<EqTerm>, mut current: Vec<EqTerm>) -> (Term, Vec<EqTerm>, Vec<EqTerm>, bool) {
    let mut C = Term {
        value: "(eps)".to_string(),
        var: 2,
        left: None,
        right: None
    };
    //println!("I want to separate");
    //for i in 0..terms.len() {
    //    println!("{}", reg_to_string(terms[i].clone()));
    //}
    let mut op_name = "".to_string();
    let mut ops = Vec::<Term>::new();
    let mut vars = Vec::<Term>::new();
    let mut consts = Vec::<Term>::new();
    for i in 0..terms.len() {
        if terms[i].var == 2 && terms[i].value != "(eps)".to_string() {
            if op_name != "".to_string() {
                if terms[i].value != op_name {
                    return (C, F, current, true);
                }
            }
            else {
                op_name = terms[i].value.clone();
            }
            ops.push(terms[i].clone());
        }
        else {
            if terms[i].var == 1 {
                vars.push(terms[i].clone());
            }
            else {
                consts.push(terms[i].clone());
            }
        }
    }
    if vars.len() == 0 && consts.len() == 0 {
        C = ops[0].clone();
        let mut n = Vec::<Term>::new();
        for i in 0..ops.len() {
            if ops[i].left != None {
                n.push(*(ops[i].clone().left.unwrap()).clone());
            }
            else {
                for j in 0..ops.len() {
                    if ops[j].left != None {
                        return (C, F, current, true);
                    }
                }
                return (C, F, current, false);
            }
        }
        let (l, new, A1, err) = separateCF(n.clone(), F.clone(), current.clone());
        current = A1;
        if err {
            return (C, F, current, true);
        }
        C.left = Some(Box::<Term>::new(l.clone()));
        for i in 0..new.len() {
            let (A, B, err) = insert_terms(F.clone(), new[i].vars.clone(), new[i].terms.clone(), current.clone());
            F = A;
            current = B;
            if err {
                return (C, F, current, true);
            }
        }
        let mut n = Vec::<Term>::new();
        for i in 0..ops.len() {
            if ops[i].right != None {
                n.push(*(ops[i].clone().right.unwrap()).clone());
            }
            else {
                for j in 0..ops.len() {
                    if ops[j].right != None {
                        return (C, F, current, true);
                    }
                }
                return (C, F, current, false);
            }
        }
        let (r, new, A1, err) = separateCF(n.clone(), F.clone(), current);
        current = A1;
        if err {
            return (C, F, current, true);
        }
        C.right = Some(Box::<Term>::new(r.clone()));
        for i in 0..new.len() {
            let (A1, A2, err) = insert_terms(F.clone(), new[i].vars.clone(), new[i].terms.clone(), current.clone());
            F = A1;
            current = A2;
            if err {
                return (C, F, current, true);
            }
        }
    }
    else if vars.len() == 0 {
        if ops.len() > 0 {
            return (C, F, current, true);
        }
        let curr = consts[0].clone();
        C = curr.clone();
        for i in 0..consts.len() {
            if consts[i].value != curr.value {
                return (C, F, current, true);
            }
        }
    }
    else {
        if consts.len() != 0 && ops.len() != 0 {
            return (C, F, current, true);
        }
        if consts.len() != 0 {
            C = consts[0].clone();
            let (A1, A2, err) = insert_terms(F.clone(), vars.clone(), consts.clone(), current.clone());
            F = A1;
            current = A2;
            if err {
                return (C, F, current, true);
            }
        }
        else {
            C = vars[0].clone();
            let (A1, A2, err) = insert_terms(F.clone(), vec![C.clone()], ops.clone(), current.clone());
            F = A1;
            current = A2;
            if err {
                return (C, F, current, true);
            }
            let mut n = Vec::<Term>::new();
            for i in 0..ops.len() {
                if ops[i].left != None {
                    n.push(*(ops[i].clone().left.unwrap()).clone());
                }
                else {
                    for j in 0..ops.len() {
                        if ops[j].left != None {
                            return (C, F, current, true);
                        }
                    }
                }
            }
            if n.len() > 1 {
                let (_, new, A1, err) = separateCF(n.clone(), F.clone(), current.clone());
                current = A1;
                if err {
                    return (C, F, current, true);
                }
                for i in 0..new.len() {
                    let (A1, A2, err) = insert_terms(F.clone(), new[i].vars.clone(), new[i].terms.clone(), current.clone());
                    F = A1;
                    current = A2;
                    if err {
                        return (C, F, current, true);
                    }
                }
            }
            let mut n = Vec::<Term>::new();
            for i in 0..ops.len() {
                if ops[i].right != None {
                    n.push(*(ops[i].clone().right.unwrap()).clone());
                }
                else {
                    for j in 0..ops.len() {
                        if ops[j].right != None {
                            return (C, F, current, true);
                        }
                    }
                }
            }
            if n.len() > 1 {
                let (_, new, A1, err) = separateCF(n.clone(), F.clone(), current.clone());
                current = A1;
                if err {
                    return (C, F, current, true);
                }
                for i in 0..new.len() {
                    let (A1, A2, err) = insert_terms(F.clone(), new[i].vars.clone(), new[i].terms.clone(), current.clone());
                    F = A1;
                    current = A2;
                    if err {
                        return (C, F, current, true);
                    }
                }
            }
        }
    }
    (C, F, current, false)
}

fn fill_eq(mut U: Vec<EqTerm>, term: Term) -> Vec<EqTerm> {
    if term.var == 1 {
        let mut exists = 0;
        for i in 0..U.len() {
            for j in 0..U[i].vars.len() {
                if U[i].vars[j].value == term.value {
                    U[i].instances += 1;
                    exists = 1;
                    continue;
                }
            }
        }
        if exists == 0 {
            U.push(EqTerm {
                instances: 1,
                vars: vec![term.clone()],
                terms: Vec::<Term>::new()
            });
        }
    }
    else if term.var == 2 {
        if term.left != None {
            U = fill_eq(U.clone(), *(term.left.unwrap()).clone());
        }
        if term.right != None {
            U = fill_eq(U.clone(), *(term.right.unwrap()).clone());
        }
    }
    U
}

fn remove_from_EqTerm(mut F: Vec<EqTerm>, term: EqTerm) -> Vec<EqTerm> {
    let kill = Term {
        value: "💀".to_string(),
        var: 1,
        left: None,
        right: None
    };
    for i in 0..F.len() {
        if F[i].vars == term.vars {
            F[i].vars = vec![kill.clone()];
        }
    }
    let mut F1 = Vec::<EqTerm>::new();
    for item in F {
        if item.vars[0].value != "💀".to_string() {
            F1.push(item);
        }
    }
    F1
}

fn getCF(mut U: Vec<EqTerm>, mut F: Vec<EqTerm>, mut T: Vec<EqTerm>, mut current: Vec<EqTerm>) -> (Vec<EqTerm>, Vec<EqTerm>, Vec<EqTerm>, bool) {
    //println!("before before");
    //println!("------------U---------");
    //print_U(U.clone());
    for i in 0..U.len() {
        U[i].instances = 0;
    }
    for i in 0..U.len() {
        for j in 0..U[i].terms.len() {
            U = fill_eq(U.clone(), U[i].terms[j].clone());
        }
    }
    let mut failstate = U.len();

    //println!("before");
    //println!("------------U---------");
    //print_U(U.clone());
    //println!("------------T---------");
    //print_U(T.clone());
    //println!("------------F---------");
    //print_U(F.clone());
    //println!("------current---------");
    //print_U(current.clone());

    for i in 0..U.len() {
        if U[i].instances == 0 {
            if U[i].terms.len() == 1 {
                T.push(U[i].clone());
                F = remove_from_EqTerm(F.clone(), U[i].clone());
                return (F, T, current, false);
            }
            failstate = 0;
            let (A1, A2, A3, err) = separateCF(U[i].terms.clone(), F.clone(), current.clone());
            let C = A1;
            F = A2;
            current = A3;
            if err {
                return (F, T, current, true);
            }
            T.push(EqTerm {
                instances: 0,
                vars: U[i].vars.clone(),
                terms: vec![C]
            });
            _ = remove_from_EqTerm(U.clone(), U[i].clone());
            break;
        }
    }
    if failstate > 0 {
        return (F, T, current, true);
    }
    //println!("after");
    //println!("------------U---------");
    //print_U(U.clone());
    //println!("------------T---------");
    //print_U(T.clone());
    //println!("------------F---------");
    //print_U(F.clone());
    //println!("------current---------");
    //print_U(current.clone());
    (F, T, current, false)
}

fn sub_unify(to: Term, vars: Vec<Term>, mut what: Term) -> Option<Box<Term>> {
    if what.var == 1 {
        for i in 0..vars.len() {
            if what.value == vars[i].value {
                what = to.clone();
            }
        }
    }
    else if what.var == 2 {
        if what.left != None {
            what.left = sub_unify(to.clone(), vars.clone(), *(what.left.unwrap()).clone());
        }
        if what.right != None {
            what.right = sub_unify(to.clone(), vars.clone(), *(what.right.unwrap()).clone());
        }
    }
    Some(Box::<Term>::new(what))
}

fn magic(mut T: Vec<EqTerm>) -> Vec<EqTerm> {
    for i in (0..T.len()).rev() {
        for j in (0..i).rev() {
            let A = sub_unify(T[i].terms[0].clone(), T[i].vars.clone(), T[j].terms[0].clone());
            let B = *(A.unwrap()).clone();
            T[j].terms = vec![B];
        }
    }
    T
}

fn try_to_unify(reg: Term, nrm: Rule) -> (Term, bool) {
    let mut U = vec![EqTerm {
        instances: 0,
        vars: vec![Term {
            value: "♥".to_string(),
            var: 0,
            left: None,
            right: None
        }],
        terms: Vec::<Term>::new()
    }];
    U[0].terms.push(reg.clone());
    U[0].terms.push(nrm.left.clone());
    //println!("{}", reg_to_string(reg.clone()));
    //println!("{} = {}", reg_to_string(nrm.left.clone()), reg_to_string(nrm.right.clone()));

    let mut T = Vec::<EqTerm>::new();
    let mut F = Vec::<EqTerm>::new();
    let mut current = Vec::<EqTerm>::new();
    while U.len() > 0 {
        let (A, B, C, err) = getCF(U.clone(), F.clone(), T.clone(), current.clone());
        F = A;
        T = B;
        current = C;
        if err {
            return (reg, true);
        }
        U = F.clone();
    }
    let A = vec![nrm.right.clone()];
    T[0].terms = A;
    T = magic(T.clone());
    (T[0].terms[0].clone(), false)
}

fn unify_or_ignore(mut reg: Term, nrm: Rule) -> (Term, bool) {
    let mut smth = false;
    let (new, err) = try_to_unify(reg.clone(), nrm.clone());
    if !err {
        reg = new.clone();
        smth = true;
    }
    (reg, smth)
}

fn bfs_is_cancer(mut head: Term, norms: Vec<Rule>) -> (Term, bool) {
    for i in 0..norms.len() {
        let (head, cont) = unify_or_ignore(head.clone(), norms[i].clone());
        if cont {
            return (head, true);
        }
    }
    if head.left != None {
        let (A, cont) = bfs_is_cancer(*(head.left.unwrap()).clone(), norms.clone());
        head.left = Some(Box::<Term>::new(A));
        if cont {
            return (head, true);
        }
    }
    if head.right != None {
        let (A, cont) = bfs_is_cancer(*(head.right.unwrap()).clone(), norms.clone());
        head.right = Some(Box::<Term>::new(A));
        if cont {
            return (head, true);
        }
    }
    (head, false)
}

fn main() {

    let eps: Term = Term {
        value: "(eps)".to_string(),
        var: 2,
        left: None,
        right: None
    };

    let mut expr = String::new();
    let mut rules = Vec::<String>::new();
    println!("Выражение:");
    _ = std::io::stdin().read_line(&mut expr);
    let mut dummy = String::new();
    println!("Пустая строка:");
    _ = std::io::stdin().read_line(&mut dummy);
    if dummy != "\n" {
        println!("Нарушение синтаксиса: пустая строка не пустая");
        process::exit(0x0100);
    }
    let mut temp = String::new();
    while true {
        println!("Правило/пустая строка для окончания ввода:");
        _ = std::io::stdin().read_line(&mut temp);
        if temp != "\n" {
            rules.push(temp.clone());
            temp = String::new();
            continue;
        }
        break;
    }
    expr = strip_spaces(expr.clone());
    for i in 0..rules.len() {
        rules[i] = strip_spaces(rules[i].clone());
    }
    let consts = Vec::<Term>::new();
    let (mut reg, consts, _) = parse_regex(expr.clone(), 0, consts.clone(), false, eps.clone());
    let mut norms = Vec::<Rule>::new();
    for i in 0..rules.len() {
        let (l, r) = parse_rule(rules[i].clone(), 0, consts.clone(), true, eps.clone());
        let n = Rule {
            right: r,
            left: l
        };
        norms.push(n);
    }
    let mut cancer = true;
    let mut iter = 0;
    while cancer {
        let mut Y = String::new();
        let (A1, A2) = bfs_is_cancer(reg.clone(), norms.clone());
        reg = A1;
        cancer = A2;
        iter += 1;
        if iter == 500 {
            println!("Прошло 500 итераций. Скорее всего произошло зацикливание. Текущий результат:");
            println!("{}", reg_to_string(reg.clone()));
            println!("Продолжить? [Y/N]");
            _ = std::io::stdin().read_line(&mut Y);
            if Y == "Y\n" {
                iter = 0;
                continue;
            }
            break;
        }
    }
    println!("Нормализованное выражение:");
    println!("{}", reg_to_string(reg.clone()));
}
