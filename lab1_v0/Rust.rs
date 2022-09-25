#![allow(non_snake_case)]
use std::process;

#[derive(Clone, PartialEq)]
struct Term {
    value: String,
    var: i32,
    children: Vec<Term>
}

fn full_term(term: Term) -> String {
    let mut out = "".to_string();
    out += &term.value;
    if term.var == 0 {
        out.push('(');
        let mut empty = 1;
        for child in term.children {
            out += &(full_term(child));
            out.push(',');
            out.push(' ');
            empty = 0;
        }
        if empty == 0 {
            out.pop();
            out.pop();
        }
        out.push(')');
    }
    out
}

fn parse_constructor(expr: String, mut ptr: usize) -> (Vec<Term>, usize) {
    let mut children = Vec::<Term>::new();
    let mut curr = "".to_string();
    let mut saw_const = 0;
    while ptr <= expr.chars().count() - 1 && expr.chars().nth(ptr).unwrap() != ')' {
        if ptr <= expr.chars().count() - 1 && expr.chars().nth(ptr).unwrap() == ' ' {
            ptr += 1;
            continue;
        }
        if ptr == expr.chars().count() - 1 {
            println!("Нарушение синтаксиса: скобки не сбалансированы -> {}", expr);
            process::exit(0x0100);
        }
        if expr.chars().nth(ptr).unwrap() == '(' {
            if curr == "" {
                println!("Нарушение синтаксиса: встречен безымянный конструктор -> {}", expr);
                process::exit(0x0100);
            }
            let mut n = Term{
                value: curr.clone(),
                var: 0,
                children: Vec::<Term>::new()
            };
            ptr += 1;
            (n.children, ptr) = parse_constructor(expr.clone(), ptr);
            children.push(n);
            saw_const = 1;
            continue;
        }
        if expr.chars().nth(ptr).unwrap() == ',' {
            if curr == "" {
                println!("Нарушение синтаксиса: встречен пустой терм -> {}", expr);
                process::exit(0x0100);
            }
            if saw_const == 0 {
                let n = Term{
                    value: curr.clone(),
                    var: 1,
                    children: Vec::<Term>::new()
                };
                children.push(n);
            }
            curr = "".to_string();
            ptr += 1;
            saw_const = 0;
            continue;
        }
        if expr.chars().nth(ptr).unwrap().is_digit(10) && curr == "".to_string() {
            println!("Нарушение синтаксиса: имя терма начинается с цифры -> {}", expr);
            process::exit(0x0100);
        }
        if !expr.chars().nth(ptr).unwrap().is_digit(10) && !expr.chars().nth(ptr).unwrap().is_alphabetic() {
            println!("Нарушение синтаксиса: встречен запрещённый символ -> {}", expr);
            process::exit(0x0100);
        }
        curr.push(expr.chars().nth(ptr).unwrap());
        ptr += 1;
    }
    if curr == "" && expr.chars().nth(ptr - 1).unwrap() != '(' {
        println!("Нарушение синтаксиса: пустое выражение -> {}", expr);
        process::exit(0x0100);
    }
    if ptr == expr.chars().count() {
        println!("Нарушение синтаксиса: скобки не сбалансированы -> {}", expr);
        process::exit(0x0100);
    }
    if curr != "" && saw_const == 0 {
        let n = Term{
            value: curr.clone(),
            var: 1,
            children: Vec::<Term>::new()
        };
        children.push(n);
    }
    ptr += 1;
    (children, ptr)
}

fn start_parse(expr: String) -> Term {
    let mut curr = "".to_string();
    let mut ptr: usize = 0;
    while ptr <= expr.chars().count() - 1 && expr.chars().nth(ptr).unwrap() != '(' {
        while ptr <= expr.chars().count() - 1 && expr.chars().nth(ptr).unwrap() == ' ' {
            ptr += 1;
        }
        if expr.chars().nth(ptr).unwrap().is_digit(10) && curr == "".to_string() {
            println!("Нарушение синтаксиса: имя терма начинается с цифры -> {}", expr);
            process::exit(0x0100);
        }
        if !expr.chars().nth(ptr).unwrap().is_digit(10) && !expr.chars().nth(ptr).unwrap().is_alphabetic() {
            println!("Нарушение синтаксиса: встречен запрещённый символ -> {}", expr);
            process::exit(0x0100);
        }
        curr.push(expr.chars().nth(ptr).unwrap());
        ptr += 1;
    }
    if ptr <= expr.chars().count() - 1 && expr.chars().nth(ptr).unwrap() == '(' {
        let mut n = Term{
            value: curr.clone(),
            var: 0,
            children: Vec::<Term>::new()
        };
        (n.children, _) = parse_constructor(expr.clone(), ptr + 1);
        return n;
    }
    if curr == "" {
        println!("Нарушение синтаксиса: пустое выражение -> {}", expr);
        process::exit(0x0100);
    }
    let n = Term{
        value: curr.clone(),
        var: 1,
        children: Vec::<Term>::new()
    };
    return n;
}

#[derive(Clone, PartialEq)]
struct EqTerm {
    instances: i32,
    equal: Vec<Term>,
    terms: Vec<Term>
}

fn fill_eq(mut U: Vec<EqTerm>, terms: Vec<Term>) -> Vec<EqTerm> {
    for i in 0..terms.len() {
        if terms[i].var == 1 {
            let mut exists = 0;
            for j in 0..U.len() {
                for k in 0..U[j].equal.len() {
                    if U[j].equal[k].value == terms[i].value {
                        U[j].instances += 1;
                        exists = 1;
                        continue;
                    }
                }
            }
            if exists == 0 {
                let mut n = EqTerm {
                    instances: 1,
                    equal: Vec::<Term>::new(),
                    terms: Vec::<Term>::new()
                };
                n.equal.push(terms[i].clone());
            }
        }
        else {
            U = fill_eq(U.clone(), terms[i].children.clone());
        }
    }
    U
}

fn add_vars(mut eqs: Vec<Term>, neqs: Vec<Term>) -> Vec<Term> {
    for neq in neqs {
        let mut present = 0;
        for eq in eqs.clone() {
            if eq.value == neq.value {
                present = 1;
            }
        }
        if present == 0 {
            eqs.push(neq);
        }
    }
    eqs
}

fn add_terms(mut eqs: Vec<Term>, neqs: Vec<Term>) -> Vec<Term> {
    for neq in neqs {
        let mut present = 0;
        for eq in eqs.clone() {
            if full_term(eq) == full_term(neq.clone()) {
                present = 1;
            }
        }
        if present == 0 {
            eqs.push(neq);
        }
    }
    eqs
}

fn append_F(mut F: Vec<EqTerm>, v_t: EqTerm) -> Vec<EqTerm> {
    for i in 0..F.len() {
        for eq in F[i].equal.clone() {
            for neq in v_t.equal.clone() {
                if neq.value == eq.value {
                    F[i].equal = add_vars(F[i].equal.clone(), v_t.equal);
                    F[i].terms = add_terms(F[i].terms.clone(), v_t.terms);
                    return F;
                }
            }
        }
    }
    F.push(v_t);
    F
}

fn extract_vars(mut F: Vec<EqTerm>, list: Vec<Term>) -> Vec<EqTerm> {
    for term in list {
        if term.var == 1 {
            let mut n = EqTerm {
                instances: 0,
                equal: Vec::<Term>::new(),
                terms: Vec::<Term>::new()
            };
            n.equal.push(term);
            F = append_F(F, n);
        }
        else {
            F = extract_vars(F, term.children);
        }
    }
    F
}

fn separateCF(list: Vec<Term>, mut F: Vec<EqTerm>) -> (Term, Vec<EqTerm>) {
    let mut C  = Term {
        value: "".to_string(),
        var: 0,
        children: Vec::<Term>::new()
    };
    let mut const_name = "".to_string();
    let mut const_len = 0;
    let mut consts = Vec::<Term>::new();
    let mut vars = Vec::<Term>::new();
    for i in 0..list.len() {
        if list[i].var == 0 {
            if const_name != "" {
                if list[i].value != const_name || list[i].children.len() != const_len {
                    println!("Унификация невозможна: расхождение в конструкторах");
                    process::exit(0x0100);
                }
            }
            else {
                const_name = (list[i].value).clone();
                const_len = list[i].children.len();
            }
            consts.push(list[i].clone());
        }
        else {
            vars.push(list[i].clone());
        }
    }
    if vars.len() == 0 {
        C.value = (consts[0].value).clone();
        if consts.len() > 1 {
            for i in 0..const_len {
                let mut n = Vec::<Term>::new();
                for j in 0..consts.len() {
                    n.push((consts[j].children[i]).clone());
                }
                let (a, b) = separateCF(n, F.clone());
                C.children.push(a);
                for entry in b {
                    F = append_F(F, entry);
                }
            }
        }
        else {
            F = extract_vars(F, (consts[0].children).clone());
            C.children = (consts[0].children).clone();
        }
    }
    else {
        C.value = (vars[0].value).clone();
        C.var = 1;
        if const_len != 0 {
            for i in 0..const_len {
                let mut n = Vec::<Term>::new();
                for j in 0..consts.len() {
                    n.push((consts[j].children[i]).clone());
                }
                if n.len() > 1 {
                    let (_, b) = separateCF(n, F.clone());
                    for entry in b {
                        F = append_F(F, entry);
                    }
                }
            }
        }
        let n = EqTerm {
            instances: 0,
            equal: vars,
            terms: consts
        };
        F = append_F(F.clone(), n);
    }
    (C, F)
}

fn remove_from_EqTerm(mut F: Vec<EqTerm>, term: EqTerm) -> Vec<EqTerm> {
    let kill = Term {
        value: "💀".to_string(),
        var: 1,
        children: Vec::<Term>::new()
    };
    for i in 0..F.len() {
        if F[i].equal == term.equal {
            F[i].equal = vec![kill.clone()];
        }
    }
    let mut F1 = Vec::<EqTerm>::new();
    for item in F {
        if item.equal[0].value != "💀".to_string() {
            F1.push(item);
        }
    }
    F1
}

fn getCF(mut U: Vec<EqTerm>, mut F: Vec<EqTerm>, mut T: Vec<EqTerm>, ind: i32) -> (i32, Vec<EqTerm>, Vec<EqTerm>) {
    for i in 0..U.len() {
        U[i].instances = 0;
    }
    for i in 0..U.len() {
        U = fill_eq(U.clone(), (U[i].clone()).terms);
    }
    let mut failstate = U.len();
    for i in 0..U.len() {
        if U[i].instances == 0 {
            failstate = 0;
            if U[i].terms.len() == 0 {
                let n = Term {
                    value: "T".to_string() + &(ind.to_string()),
                    var: 1,
                    children: Vec::<Term>::new()
                };
                U[i].terms.push(n);
                let out = EqTerm {
                    instances: 0,
                    equal: (U[i].clone()).equal,
                    terms: (U[i].clone()).terms
                };
                F = remove_from_EqTerm(F.clone(), U[i].clone());
                T.push(out.clone());
                return (ind + 1, F.clone(), T.clone());
            }
            let C: Term;
            (C, F) = separateCF((U[i].clone()).terms, F.clone());
            let mut n = EqTerm {
                instances: 0,
                equal: (U[i].clone()).equal,
                terms: Vec::<Term>::new()
            };
            n.terms.push(C);
            T.push(n);
            F = remove_from_EqTerm(F.clone(), U[i].clone());
            //U = remove_from_EqTerm(U.clone(), U[i].clone());
            break;
        }
    }
    if failstate > 0 {
        println!("Унификация невозможна: цикл");
        process::exit(0x0100);
    }
    (ind, F, T)
}

fn complete_match(eq1: Vec<Term>, eq2: Vec<Term>) -> bool {
    if eq1.len() != eq2.len() {
        return false;
    }
    for i in 0..eq1.len() {
        if eq1[i].value != eq2[i].value {
            return false;
        }
    }
    true
}

fn unique(F: Vec<EqTerm>, eqs: Vec<Term>) -> bool {
    for i in 0..F.len() {
        if !complete_match(F[i].equal.clone(), eqs.clone()) {
            for neq in eqs.clone() {
                for eq in F[i].equal.clone() {
                    if neq.value == eq.value {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn clear_empty(mut F: Vec<EqTerm>) -> Vec<EqTerm> {
    for i in (0..F.len()).rev() {
        F[i].instances = 0;
        if F[i].terms.len() == 0 && !unique(F.clone(), F[i].equal.clone()) {
            F[i].equal = vec![];
        }
    }
    let mut F1 = Vec::<EqTerm>::new();
    for item in F {
        if item.equal.len() != 0 {
            F1.push(item.clone());
        }
    }
    F1
}

fn sub_unify(to: Term, eqs: Vec<Term>, mut what: Vec<Term>) -> Vec<Term> {
    for i in 0..what.len() {
        if what[i].var == 1 {
            for eq in eqs.clone() {
                if what[i].value == eq.value {
                    what[i] = to.clone();
                }
            }
        }
        else {
            what[i].children = sub_unify(to.clone(), eqs.clone(), what[i].children.clone());
        }
    }
    what
}

fn merge_equal(mut A: Vec<EqTerm>) -> Vec<EqTerm> {
    for i in (0..A.len()).rev() {
        for j in (0..i).rev() {
            if full_term(A[i].terms[0].clone()) == full_term(A[j].terms[0].clone()) {
                A[j].equal = add_vars(A[j].equal.clone(), A[i].equal.clone());
                A[i].equal = vec![];
                break;
            }
        }
    }
    A
}

fn already_present(have: String, comp: Vec<Term>) -> bool {
    for item in comp {
        if item.value == have {
            return true;
        }
    }
    false
}

fn merge_equals(A: Vec<EqTerm>) -> Vec<EqTerm> {
    let mut A1 = Vec::<EqTerm>::new();
    for mut item in A.clone() {
        let mut neqs = Vec::<Term>::new();
        for eq in item.equal {
            if !already_present(eq.value.clone(), neqs.clone()) {
                neqs.push(eq.clone());
            }
        }
        item.equal = neqs.clone();
        A1.push(item.clone());
    }
    A1
}

fn unify_system(mut A: Vec<EqTerm>) -> Vec<EqTerm> {
    for i in (0..A.len()).rev() {
        for j in (0..i).rev() {
            A[j].terms = sub_unify(A[i].terms[0].clone(), A[i].equal.clone(), A[j].terms.clone());
        }
    }
    A = merge_equal(A.clone());
    let mut A1 = Vec::<EqTerm>::new();
    for item in A {
        if item.equal.len() != 0 {
            A1.push(item.clone());
        }
    }
    A1
}

fn clean_F(F: Vec<EqTerm>) -> Vec<EqTerm> {
    let mut F1 = Vec::<EqTerm>::new();
    for item in F {
        if item.equal[0].value != "💀".to_string() {
            F1.push(item);
        }
    }
    F1
}

fn MartMont(mut U: Vec<EqTerm>, mut F: Vec<EqTerm>, mut T: Vec<EqTerm>, mut ind: i32) -> Vec<EqTerm> {
    if U.len() > 0 {
        (ind, F, T) = getCF(U.clone(), F, T, ind);
        F = clean_F(F.clone());
        F = clear_empty(F.clone());
        U = F.clone();
        return MartMont(U, F, T, ind);
    }
    T
}

fn print_T(T: Vec<EqTerm>) {
    println!("Унифицированное выражение: ");
    println!("{}", "{".to_string() + &(T[0].equal[0].value).to_string() + &"} = (".to_string() + &(full_term(T[0].terms[0].clone())).to_string());
    println!("Правила унификации: ");
    for i in 1..T.len() {
        let mut out = "{".to_string();
        for eq in T[i].equal.clone() {
            out += &eq.value.to_string();
            out.push(',');
            out.push(' ');
        }
        out.pop();
        out.pop();
        out += &"} = (".to_string();
        for term in T[i].terms.clone() {
            out += &full_term(term).to_string();
            out += &", ".to_string();
        }
        out.pop();
        out.pop();
        out.push(')');
        println!("{}", out);
    }
}

fn main() {
    let mut expr1 = String::new();
    let mut expr2 = String::new();
    println!("Expression 1: ");
    _ = std::io::stdin().read_line(&mut expr1);
    println!("Expression 2: ");
    _ = std::io::stdin().read_line(&mut expr2);
    let terms1 = start_parse(expr1);
    let terms2 = start_parse(expr2);
    let mut U = Vec::<EqTerm>::new();
    let mut n = EqTerm{
        instances: 0,
        equal: Vec::<Term>::new(),
        terms: Vec::<Term>::new()
    };
    let nn = Term {
        value: "♥".to_string(),
        var: 1,
        children: Vec::<Term>::new()
    };
    n.equal.push(nn);
    n.terms.push(terms1);
    n.terms.push(terms2);
    let mut T = Vec::<EqTerm>::new();
    U.push(n);
    let F = Vec::<EqTerm>::new();
    T = MartMont(U, F, T, 1);
    T = unify_system(T.clone());
    T = merge_equals(T.clone());
    print_T(T);
}
