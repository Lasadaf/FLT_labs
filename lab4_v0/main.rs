#![allow(non_snake_case)]
#![allow(while_true)]
#![allow(non_camel_case_types)]
use std::process;
use std::convert::TryInto;
#[derive(Clone, PartialEq)]
struct Rule_body {
    pre: String,
    post: String
}

#[derive(Clone)]
struct Rule {
    head: String,
    bodies: Vec<Rule_body>
}

#[derive(Clone)]
struct Conflict {
    sides: Vec<Rule>,
    resolved: usize,
    done: bool
}

#[derive(Clone)]
struct State {
    id: i32,
    roots: Vec<Rule>,
    rules: Vec<Rule>
}

#[derive(Clone)]
struct Shift {
    value: String,
    fr: i32,
    to: i32
}

#[derive(Clone)]
struct FOLLOW {
    head: String,
    bodies: Vec<String>
}

fn del_whitespaces(rule: String) -> String {
    let mut new_rule = "".to_string();
    for i in 0..rule.len() {
        if rule.chars().nth(i).unwrap() != ' ' && rule.chars().nth(i).unwrap() != '\t' && rule.chars().nth(i).unwrap() != '\r' && rule.chars().nth(i).unwrap() != '\n' {
            new_rule.push(rule.chars().nth(i).unwrap());
        }
    }
    new_rule
}

fn append_rule(mut output: Vec<Rule>, head: String, bodies: Vec<Rule_body>) -> Vec<Rule> {
    for i in 0..output.len() {
        if output[i].head == head {
            for j in 0..bodies.len() {
                output[i].bodies.push(bodies[j].clone());
            }
            return output;
        }
    }
    let n = Rule {
        head: head,
        bodies: bodies
    };
    output.push(n);
    output
}

fn parse_rules(rules: Vec<String>) -> Vec<Rule> {
    let mut output = Vec::<Rule>::new();
    for i in 0..rules.len() {
        if rules[i].chars().nth(0).unwrap() != '[' {
            println!("Синтаксическая ошибка: начальный нетерминал не найден");
            process::exit(0x0100);
        }
        let mut ii = 1;
        let mut new_head = "".to_string();
        while ii < rules[i].len() && rules[i].chars().nth(ii).unwrap() != ']' {
            new_head.push(rules[i].chars().nth(ii).unwrap());
            ii += 1;
        }
        if ii == rules[i].len() {
            println!("Синтаксическая ошибка: начальный нетерминал не закрыт");
            process::exit(0x0100);
        }
        ii += 1;
        if rules[i].chars().nth(ii).unwrap() != '-' || rules[i].chars().nth(ii + 1).unwrap() != '>' {
            println!("Синтаксическая ошибка: не найден разделитель в правиле");
            process::exit(0x0100);
        }
        ii += 2;
        let mut bodies = Vec::<String>::new();
        let mut new_body = "".to_string();
        while ii < rules[i].len() {
            if rules[i].chars().nth(ii).unwrap() == '|' {
                bodies.push(new_body.clone().to_string());
                new_body = "".to_string();
                if ii == rules[i].len() - 1 {
                    println!("Синтаксическая ошибка: после знака | нет выражения");
                    process::exit(0x0100);
                }
                ii += 1;
                continue;
            }
            if rules[i].chars().nth(ii).unwrap() == '[' {
                while ii < rules[i].len() && rules[i].chars().nth(ii).unwrap() != ']' {
                    new_body.push(rules[i].chars().nth(ii).unwrap());
                    ii += 1;
                }
                if ii == rules[i].len() {
                    println!("Синтаксическая ошибка: нетерминал не закрыт");
                    process::exit(0x0100);
                }
                new_body = new_body + &"]".to_string();
                ii += 1;
                continue;
            }
            if !(rules[i].chars().nth(ii).unwrap().is_alphabetic()) && !(rules[i].chars().nth(ii).unwrap().is_digit(10)) {
                println!("Синтаксическая ошибка: неопознанный символ");
                process::exit(0x0100);
            }
            new_body.push(rules[i].chars().nth(ii).unwrap());
            ii += 1;
        }
        bodies.push(new_body.clone().to_string());
        let mut new_bodies = Vec::<Rule_body>::new();
        for j in 0..bodies.len() {
            new_bodies.push(Rule_body {
                pre: "".to_string(),
                post: ".".to_string() + &bodies[j].to_string() + &"$".to_string()
            });
        }
        output = append_rule(output.clone(), new_head.to_string(), new_bodies);
    }
    output
}

fn rule_to_string(rule: Rule) -> String {
    let mut out = "[".to_string() + &rule.head.to_string() + &"] -> ".to_string();
    for body in rule.bodies {
        out = out + &body.pre.to_string() + &(body.post[..body.post.len() - 1]).to_string() + &" | ".to_string();
    }
    out = out[..out.len() - 3].to_string();
    out
}

fn filter_rules(rules: Vec<Rule>) -> Vec<Rule> {
    let mut new_rules = Vec::<Rule>::new();
    for rule in rules {
        new_rules = append_rule(new_rules.clone(), rule.head, rule.bodies);
    }
    new_rules
}

fn present(body: Rule_body, bodies: Vec<Rule_body>) -> bool {
    for b in bodies {
        if b == body {
            return true;
        }
    }
    false
}

fn formalize_roots(roots: Vec<Rule>) -> Vec<Rule> {
    let mut new_roots = Vec::<Rule>::new();
    for i in 0..roots.len() {
        let head = roots[i].head.clone();
        let mut done = false;
        for j in 0..new_roots.len() {
            if new_roots[j].head == head {
                done = true;
                for k in 0..roots[i].bodies.len() {
                    if !present(roots[i].bodies[k].clone(), new_roots[j].bodies.clone()) {
                        new_roots[j].bodies.push(roots[i].bodies[k].clone());
                    }
                }
            }
        }
        if !done {
            new_roots.push(Rule {
                head: head.clone(),
                bodies: roots[i].bodies.clone()
            });
        }
    }
    new_roots
}

fn inside(head: String, heads: Vec<String>) -> bool {
    for h in heads {
        if h == head {
            return true;
        }
    }
    false
}

fn same_roots(roots: Vec<Rule>, state: Vec<Rule>) -> bool {
    if roots.len() != state.len() {
        return false;
    }
    for i in 0..roots.len() {
        for _j in 0..state.len() {
            if rule_to_string(roots[i].clone()) != rule_to_string(state[i].clone()) {
                return false;
            }
        }
    }
    true
}

fn build_LR(rules: Vec<Rule>, roots: Vec<Rule>, mut states: Vec<State>, mut shifts: Vec<Shift>, mut id: i32) -> (Vec<State>, Vec<Shift>, i32) {
    let mut state_rules = Vec::<Rule>::new();
    let mut exits = Vec::<String>::new();
    for i in 0..roots.len() {
        for j in 0..roots[i].bodies.len() {
            let mut term = "".to_string();
            if roots[i].bodies[j].post.chars().nth(1).unwrap() != '$' {
                if roots[i].bodies[j].post.chars().nth(1).unwrap() == '[' {
                    let mut ii = 2;
                    while roots[i].bodies[j].post.chars().nth(ii).unwrap() != ']' {
                        term += &roots[i].bodies[j].post.chars().nth(ii).unwrap().to_string();
                        ii += 1;
                    }
                } else {
                    term.push(roots[i].bodies[j].post.chars().nth(1).unwrap());
                }
                if !inside(term.clone(), exits.clone()) {
                    exits.push(term.clone());
                }
            }
        }
    }
    let mut j = 0;
    while j < rules.len() {
        if inside(rules[j].head.clone(), exits.clone()) {
            state_rules.push(rules[j].clone());
            let mut reset = false;
            for jj in 0..rules[j].bodies.len() {
                let mut term = "".to_string();
                if rules[j].bodies[jj].post.chars().nth(1).unwrap() != '$' {
                    if rules[j].bodies[jj].post.chars().nth(1).unwrap() == '[' {
                        let mut i = 2;
                        while rules[j].bodies[jj].post.chars().nth(i).unwrap() != ']' {
                            term.push(rules[j].bodies[jj].post.chars().nth(i).unwrap().clone());
                            i += 1;
                        }
                    } else {
                        term.push(rules[j].bodies[jj].post.chars().nth(1).unwrap().clone());
                    }
                    if !inside(term.clone(), exits.clone()) {
                        exits.push(term.clone().to_string());
                        j = 0;
                        state_rules = Vec::<Rule>::new();
                        reset = true;
                        break;
                    }
                }
            }
            if reset {
                continue;
            }
        }
        j += 1;
    }
    let mut new_r = roots.clone();
    new_r.extend(state_rules.clone());
    state_rules = new_r.clone();
    let new_state = State {
        id: id.clone(),
        roots: roots.clone(),
        rules: state_rules.clone()
    };
    id += 1;
    states.push(new_state.clone());
    for i in 0..exits.len() {
        let mut new_roots = Vec::<Rule>::new();
        for j in 0..state_rules.len() {
            for k in 0..state_rules[j].bodies.len() {
                if state_rules[j].bodies[k].post.chars().nth(1).unwrap() != '$' {
                    let mut affected = true;
                    if state_rules[j].bodies[k].post.chars().nth(1).unwrap() == '[' {
                        let mut ii = 2;
                        let mut cur_ex = "".to_string();
                        while state_rules[j].bodies[k].post.chars().nth(ii).unwrap() != ']' {
                            cur_ex.push(state_rules[j].bodies[k].post.chars().nth(ii).unwrap().clone());
                            ii += 1;
                        }
                        //println!("{}", exits[i].clone());
                        //println!("{}", cur_ex);
                        if exits[i] != cur_ex {
                            affected = false;
                        }
                    } else {
                        if state_rules[j].bodies[k].post.chars().nth(1).unwrap().to_string() != exits[i] {
                            affected = false;
                        }
                    }
                    if affected {
                        if state_rules[j].bodies[k].post.chars().nth(1).unwrap() == '[' {
                            let mut ii = 1;
                            while state_rules[j].bodies[k].post.chars().nth(ii - 1).unwrap() != ']' {
                                ii += 1;
                            }
                            let n = Rule_body {
                                pre: state_rules[j].bodies[k].pre.clone() + &((state_rules[j].bodies[k].post.clone())[1..ii]).to_string(),
                                post: ".".to_string() + &((state_rules[j].bodies[k].post.clone())[ii..]).to_string()
                            };
                            let mut nn = Rule {
                                head: state_rules[j].head.clone(),
                                bodies: Vec::<Rule_body>::new()
                            };
                            nn.bodies.push(n);
                            new_roots.push(nn);
                        } else {
                            let n = Rule_body {
                                pre: state_rules[j].bodies[k].pre.clone() + &((state_rules[j].bodies[k].post.clone()).chars().nth(1).unwrap()).to_string(),
                                post: ".".to_string() + &((state_rules[j].bodies[k].post.clone())[2..]).to_string()
                            };
                            let mut nn = Rule {
                                head: state_rules[j].head.clone(),
                                bodies: Vec::<Rule_body>::new()
                            };
                            nn.bodies.push(n);
                            new_roots.push(nn);
                        }
                    }
                }
            }
        }
        let mut new_to = id.clone();
        let mut looop = false;
        new_roots = filter_rules(new_roots.clone());
        for ii in 0..states.len() {
            if same_roots(new_roots.clone(), states[ii].roots.clone()) {
                new_to = states[ii].id.clone();
                looop = true;
                break;
            }
        }
        if !looop {
            let new_shift = Shift {
                value: exits[i].clone(),
                fr: new_state.id.clone(),
                to: id.clone()
            };
            shifts.push(new_shift);
            (states, shifts, id) = build_LR(rules.clone(), formalize_roots(new_roots.clone()), states.clone(), shifts.clone(), id.clone());
            continue;
        } else {
            let new_shift = Shift {
                value: exits[i].clone(),
                fr: new_state.id.clone(),
                to: new_to.clone()
            };
            shifts.push(new_shift);
            continue;
        }
    }
    (states, shifts, id)
}

fn get_rule(rules: Vec<Rule>, head: String) -> Rule {
    for i in 0.. rules.len() {
        if rules[i].head == head {
            return rules[i].clone();
        }
    }
    Rule {
        head: "".to_string(),
        bodies: Vec::<Rule_body>::new()
    }
}

fn first_k(rules: Vec<Rule>, rule: Rule, k: i32, have1: usize, mut cycle: usize) -> Vec<String> {
    let mut out = Vec::<String>::new();
    let mut r_bodies = Vec::<Rule_body>::new();
    //println!("this rule has {} as head and {} bodies", rule.head.clone(), rule.bodies.len());
    //for b in rule.bodies.clone() {
    //    println!("{}", b.post);
    //}
    for i in 0..rule.bodies.len() {
        r_bodies.push(Rule_body {
            pre: rule.bodies[i].pre.clone(),
            post: rule.bodies[i].post.clone()
        });
    }
    //println!("working for {}", rule.head.clone());
    let mut rule1 = Rule{
        head: rule.head.clone(),
        bodies: Vec::<Rule_body>::new()
    };
    //println!("r_bodies is now:");
    //for b in r_bodies.clone() {
    //    println!("{}", b.post);
    //}
    rule1.bodies = r_bodies.clone();
    //println!("rule1.bodies is now length {}", rule1.bodies.len());
    //for b in rule1.bodies.clone() {
    //    println!("{}", b.post);
    //}
    let mut lens = rule1.bodies.len();
    let mut i = 0;
    while i < lens {
        //println!("{}", rule1.bodies.len().clone());
        let mut have = have1.clone();
        let mut n_char = "".to_string();
        let mut terminal = true;
        let mut ii = 1;
        //println!("i am outside while with i = {}, ii = {}, k = {}", i.clone(), ii.clone(), k.clone());
        let mut danger = 0;
        while rule1.bodies[i].post.chars().nth(ii).unwrap() != '$' && ii - 1 < k.try_into().unwrap() {
            if rule1.bodies[i].post.chars().nth(ii).unwrap() == '[' {
                if ii == 1 {
                    danger = 1;
                }
                terminal = false;
                ii += 1;
                let mut char = "".to_string();
                while rule1.bodies[i].post.chars().nth(ii).unwrap() != ']' {
                    char.push(rule1.bodies[i].post.chars().nth(ii).unwrap());
                    ii += 1;
                }
                ii += 1;
                let new_rule = get_rule(rules.clone(), char.clone());
                if new_rule.head == rule1.head && danger == 1 {
                    //println!("{}", new_rule.head);
                    danger = 2;
                    cycle += 1;
                }
                if (new_rule.head != rule1.head || have <= k.try_into().unwrap()) && (danger != 2 || cycle <= k.try_into().unwrap()) {
                    //println!("{} will try into {}", rule1.head.clone(), new_rule.head.clone());
                    let pref = first_k(rules.clone(), new_rule.clone(), k.clone(), have.clone(), cycle.clone());
                    //println!("Got back prefs:");
                    for j in 0..pref.len() {
                        //println!("{} might become {} ", pref[j], ".".to_string() + &n_char.clone().to_string() + &pref[j].clone() + &((rule1.bodies[i].post.clone())[ii..]).to_string());
                        rule1.bodies.push(Rule_body{
                            pre: rule1.bodies[i].pre.clone(),
                            post: ".".to_string() + &n_char.clone().to_string() + &pref[j].clone() + &((rule1.bodies[i].post.clone())[ii..]).to_string()
                        });
                        lens += 1;
                    }
                    break;
                }
            } else {
                //println!("i am in else and i want to push {}", rule1.bodies[i].post.chars().nth(ii).unwrap());
                n_char.push(rule1.bodies[i].post.chars().nth(ii).unwrap());
                have += 1;
                ii += 1;
            }
        }
        if terminal {
            //println!("found {}", n_char.clone());
            if !inside(n_char.clone(), out.clone()) {
                out.push(n_char.clone());
                //println!("out is now:");
                //for o in out.clone() {
                //    println!("{}", o.clone());
                //}
            }
        }
        i += 1;
    }
    out
}

fn append_follow(head: String, follows: Vec<String>, mut out: Vec<FOLLOW>) -> Vec<FOLLOW> {
    //println!("out is now:");
    //for o in out.clone() {
    //    println!("----{}----", o.head);
    //    for oo in o.bodies {
    //        println!("{}", oo);
    //    }
    //}
    //println!("and i'm trying to add {} with bodies:", head.clone());
    //for b in follows.clone() {
    //    println!("{}", b);
    //}
    for i in 0..out.len() {
        if out[i].head == head {
            for j in 0..follows.len() {
                if !inside(follows[j].clone(), out[i].bodies.clone()) {
                    out[i].bodies.push(follows[j].clone());
                }
            }
            return out
        }
    }
    let mut new_follow = FOLLOW {
        head: head.clone(),
        bodies: Vec::<String>::new()
    };
    new_follow.bodies = follows.clone();
    out.push(new_follow.clone());
    out
}

fn append_follows(to_head: String, from_head: String, mut out: Vec<FOLLOW>, k: i32) -> Vec<FOLLOW> {
    for i in 0..out.len() {
        if out[i].head == to_head {
            for j in 0..out.len() {
                if out[j].head == from_head {
                    let mut new_bodies = Vec::<String>::new();
                    for ii in 0..out[i].bodies.len() {
                        for jj in 0..out[j].bodies.len() {
                            let mut new_body = "".to_string();
                            let mut iii = 0;
                            while iii < k && iii < out[i].bodies[ii].len().try_into().unwrap() {
                                new_body.push(out[i].bodies[ii].chars().nth(iii.try_into().unwrap()).unwrap());
                                iii += 1;
                            }
                            let mut jjj = 0;
                            while iii + jjj < k.try_into().unwrap() && jjj < out[j].bodies[jj].len().try_into().unwrap() {
                                new_body.push(out[j].bodies[jj].chars().nth(jjj.try_into().unwrap()).unwrap());
                                jjj += 1;
                            }
                            new_body.push('$');
                            if !inside(new_body.clone(), new_bodies.clone()) {
                                new_bodies.push(new_body.clone());
                            }
                        }
                    }
                    out[i].bodies = new_bodies.clone();
                    break;
                }
            }
        }
    }
    out
}

fn full(out: Vec<FOLLOW>, head: String, k: i32) -> bool {
    for i in 0..out.len() {
        if out[i].head == head {
            for j in 0..out[i].bodies.len() {
                if out[i].bodies[j].len() < k.try_into().unwrap() {
                    let mut passed = false;
                    let mut ii = 0;
                    while ii < out[i].bodies[j].len() {
                        if out[i].bodies[j].chars().nth(ii).unwrap() == '$' {
                            passed = true;
                        }
                        ii += 1;
                    }
                    if !passed {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn follow_k(rules: Vec<Rule>, kk: i32) -> Vec<FOLLOW> {
    let mut out = Vec::<FOLLOW>::new();
    let mut S = FOLLOW {
        head: "S".to_string(),
        bodies: Vec::<String>::new()
    };
    S.bodies.push("$".to_string());
    out.push(S.clone());
    for _i in 0.. rules.len() {
        for j in 0..rules.len() {
            for k in 0..rules[j].bodies.len() {
                let mut ii = 1;
                while rules[j].bodies[k].post.chars().nth(ii).unwrap() != '$' {
                    if rules[j].bodies[k].post.chars().nth(ii).unwrap() == '[' {
                        let mut new_head = "".to_string();
                        ii += 1;
                        while rules[j].bodies[k].post.chars().nth(ii).unwrap() != ']' {
                            new_head.push(rules[j].bodies[k].post.chars().nth(ii).unwrap());
                            ii += 1;
                        }
                        ii += 1;
                        //println!("found {} in {}", new_head.clone(), rules[j].head.clone());
                        //println!("let's add {}", ((rules[j].bodies[k].post.clone())[ii..]).to_string());
                        let n = Rule_body {
                            pre: "".to_string(),
                            post: ".".to_string() + &((rules[j].bodies[k].post.clone())[ii..]).to_string()
                        };
                        //println!("{}", n.post.clone());
                        let mut nn = Rule {
                            head: rules[j].head.clone(),
                            bodies: Vec::<Rule_body>::new()
                        };
                        nn.bodies.push(n.clone());
                        //for nnn in nn.bodies.clone() {
                        //    println!("{}", nnn.post);
                        //}
                        //println!("{}", rule_to_string(nn.clone()));
                        let firsts = first_k(rules.clone(), nn.clone(), kk.clone().try_into().unwrap(), 0, 0);
                        out = append_follow(new_head.clone(), firsts, out.clone());
                        continue;
                    }
                    ii += 1;
                }
            }
        }
    }
    //for o in out.clone() {
    //    println!("----{}----", o.head);
    //    for oo in o.bodies {
    //        println!("{}", oo);
    //    }
    //}
    for i in 0..rules.len() {
        for j in 0..rules[i].bodies.len() {
            let mut ii = 1;
            while rules[i].bodies[j].post.chars().nth(ii).unwrap() != '$' {
                if rules[i].bodies[j].post.chars().nth(ii).unwrap() == '[' {
                    let mut new_head = "".to_string();
                    ii += 1;
                    while rules[i].bodies[j].post.chars().nth(ii).unwrap() != ']' {
                        new_head.push(rules[i].bodies[j].post.chars().nth(ii).unwrap());
                        ii += 1;
                    }
                    ii += 1;
                    if new_head != rules[i].head || !full(out.clone(), rules[i].head.clone(), kk.clone()) {
                        out = append_follows(new_head.clone(), rules[i].head.clone(), out.clone(), kk.clone());
                    }
                    continue;
                }
                ii += 1;
            }
        }
    }
    out
}

fn filter(follows: Vec<FOLLOW>, k: usize) -> Vec<FOLLOW> {
    let mut new_follows = Vec::<FOLLOW>::new();
    for i in 0..follows.len() {
        if follows[i].head == "S".to_string() {
            new_follows.push(follows[i].clone());
            continue;
        }
        let mut new_follow = FOLLOW {
            head: follows[i].head.clone(),
            bodies: Vec::<String>::new()
        };
        let mut new_bodies = Vec::<String>::new();
        for j in 0..follows[i].bodies.len() {
            let mut ii = 0;
            let mut new_body = "".to_string();
            while ii < k && follows[i].bodies[j].chars().nth(ii).unwrap() != '$' {
                new_body.push(follows[i].bodies[j].chars().nth(ii).unwrap());
                ii += 1;
            }
            new_body.push('$');
            if !inside(new_body.clone(), new_bodies.clone()) {
                new_bodies.push(new_body.clone());
            }
        }
        new_follow.bodies = new_bodies.clone();
        new_follows.push(new_follow.clone());
    }
    new_follows
}

fn collect_roots(state: State) -> Vec<Rule> {
    let mut roots = Vec::<Rule>::new();
    for i in 0..state.roots.len() {
        for j in 0..state.roots[i].bodies.len() {
            roots.push(Rule {
                head: state.roots[i].head.clone(),
                bodies: vec![state.roots[i].bodies[j].clone()]
            });
        }
    }
    roots
}

fn get_conflicts(states: Vec<State>) -> Vec<Conflict> {
    let mut conflicts = Vec::<Conflict>::new();
    for i in 0..states.len() {
        if states[i].roots.len() > 1 || states[i].roots[0].bodies.len() > 1 {
            for j in 0..states[i].roots.len() {
                for k in 0..states[i].roots[j].bodies.len() {
                    if states[i].roots[j].bodies[k].post.chars().nth(1).unwrap() == '$' {
                        let new_conflict = collect_roots(states[i].clone());
                        conflicts.push(Conflict {
                            sides: new_conflict.clone(),
                            resolved: 0,
                            done: false
                        });
                    }
                }
            }
        }
    }
    conflicts
}

fn get_follows(follows: Vec<FOLLOW>, head: String) -> Vec<String> {
    for i in 0..follows.len() {
        if follows[i].head == head {
            return follows[i].bodies.clone()
        }
    }
    vec!["".to_string()]
}

fn filter_strs(list: Vec<String>, k: usize) -> Vec<String> {
    let mut new_list = Vec::<String>::new();
    for i in 0..list.len() {
        let mut new_l = "".to_string();
        let mut j = 0;
        while j < k && list[i].chars().nth(j).unwrap() != '$' {
            new_l.push(list[i].chars().nth(j).unwrap());
            j += 1;
        }
        new_list.push(new_l.clone());
    }
    new_list
}

fn equal(f1: Vec<String>, f2: Vec<String>) -> usize {
    for i in 0..f1.len() {
        let mut danger = true;
        for j in 0..f2.len() {
            if f1[i] == f2[j] {
                //println!("found {} == {}", f1[i].clone(), f2[j].clone());
                danger = false;
            }
        }
        if danger {
            return 0;
        }
    }
    for i in 0..f2.len() {
        let mut danger = true;
        for j in 0..f1.len() {
            if f1[j] == f2[i] {
                //println!("found {} == {}", f2[i].clone(), f1[j].clone());
                danger = false;
            }
        }
        if danger {
            return 0;
        }
    }
    1
}

fn intersect(f1: Vec<String>, f2: Vec<String>) -> usize {
    for i in 0..f1.len() {
        for j in 0..f2.len() {
            if f1[i] == f2[j] {
                //println!("found intersection with {} == {}", f2[j], f1[i]);
                return 1;
            }
        }
    }
    0
}

fn resolve(rules: Vec<Rule>, follows: Vec<FOLLOW>, mut conflicts: Vec<Conflict>, kk: usize) -> (Vec<Conflict>, bool) {
    let mut ok = true;
    for i in 0..conflicts.len() {
        let mut comp = Vec::<Vec<String>>::new();
        for j in 0..conflicts[i].sides.len() {
            let head = conflicts[i].sides[j].head.clone();
            for k in 0..conflicts[i].sides[j].bodies.len() {
                if conflicts[i].sides[j].bodies[k].post.chars().nth(1).unwrap() == '$' {
                    let mut cands = Vec::<String>::new();
                    let fff = get_follows(follows.clone(), head.clone());
                    for l in 0..fff.len() {
                        if !inside(fff[l].clone(), cands.clone()) {
                            cands.push(fff[l].clone());
                        }
                    }
                    cands = filter_strs(cands.clone(), kk.clone());
                    comp.push(cands.clone());
                } else {
                    let firsts = first_k(rules.clone(), conflicts[i].sides[j].clone(), kk.clone().try_into().unwrap(), 0, 0);
                    let mut cands = Vec::<String>::new();
                    for l in 0..firsts.len() {
                        let fff = get_follows(follows.clone(), head.clone());
                        for fl in 0..fff.len() {
                            if !inside(firsts[l].clone() + &fff[fl].clone(), cands.clone()) {
                                cands.push(firsts[l].clone() + &fff[fl].clone());
                            }
                        }
                    }
                    cands = filter_strs(cands.clone(), kk.clone());
                    comp.push(cands.clone());
                }
            }
        }
        //for j in 0..comp.len() {
        //    println!("---{}---", conflicts[i].sides[j].head.clone());
        //    for cc in comp[j].clone() {
        //        println!("{}", cc);
        //    }
        //}
        //println!("+++++++++");
        let mut equality = 0;
        let mut intersection = 0;
        for j in 0..comp.len() {
            for k in (j.clone() + 1)..comp.len() {
                equality += equal(comp[j].clone(), comp[k].clone());
                intersection += intersect(comp[j].clone(), comp[k].clone());
            }
        }
        //println!("intersection = {}, equality = {}", intersection.clone(), equality.clone());
        if intersection == 0 {
            if !conflicts[i].done {
                conflicts[i].resolved = kk.clone();
                //println!("setting resolved to {}", kk.clone());
            }
            conflicts[i].done = true;
            //println!("this conflict is done");
            continue;
        } else if equality == comp.len() {
            conflicts[i].resolved = kk.clone();
            ok = false;
            //println!("this conflict is hanging");
            continue;
        } else {
            conflicts[i].resolved = 0;
            conflicts[i].done = false;
            ok = false;
            //println!("this conflict is dead");
        }
    }
    (conflicts, ok)
}

fn main() {
    let mut n_str = String::new();
    let mut temp = String::new();
    let mut rules = Vec::<String>::new();
    println!("N = ");
    _ = std::io::stdin().read_line(&mut n_str);
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
    for i in 0..rules.len() {
        rules[i] = del_whitespaces(rules[i].clone());
    }
    let parsed = parse_rules(rules);
    let mut shifts = Vec::<Shift>::new();
    let mut states = Vec::<State>::new();
    (states, shifts, _) = build_LR(parsed.clone(), vec![Rule {
        head: "♥".to_string(),
        bodies: vec![Rule_body {
            pre: "".to_string(),
            post: ".[S]$".to_string()
        }]
    }], states.clone(), shifts.clone(), 1);
    println!("https://dreampuf.github.io/GraphvizOnline/");
    println!("digraph {{");
    for i in 0..states.len() {
        let mut label = "".to_string();
        for j in 0..states[i].rules.len() {
            label += &((rule_to_string(states[i].rules[j].clone())).to_string() + &"\n".to_string());
        }
        label.pop();
        let mut addendum = "".to_string();
        if states[i].id == 2 {
            addendum = "peripheries = 2".to_string();
        }
        println!("{} [shape = \"rectangle\" {} label = \"{}\"]", states[i].id.clone(), addendum, label.clone());
    }
    for i in 0..shifts.len() {
        println!("{} -> {} [label = \"{}\"]", shifts[i].fr.clone(), shifts[i].to.clone(), shifts[i].value.clone());
    }
    println!("}}");
    n_str.pop();
    let k: i32 = n_str.to_string().parse().unwrap();
    let mut conflicts = get_conflicts(states.clone());

    //for conf in conflicts.clone() {
    //    println!("conflict between");
    //    for c in conf.sides {
    //        println!("{}", rule_to_string(c));
    //    }
    //}

    //for i in 1..1 + k.clone() {
    //    let follows = filter(follow_k(parsed.clone(), i.clone().try_into().unwrap()), i.clone().try_into().unwrap());
    //    for f in follows.clone() {
    //        println!("---{}, {}---", f.head, i.clone());
    //        let mut ooo = "".to_string();
    //        for ff in f.bodies {
    //            ooo += &(ff.to_string() + &", ".to_string());
    //        }
    //        println!("[{}]", ooo);
    //    }
    //}

    for i in 1..1 + k.clone() {
        let follows = filter(follow_k(parsed.clone(), i.clone().try_into().unwrap()), i.clone().try_into().unwrap());
        let ok: bool;
        (conflicts, ok) = resolve(parsed.clone(), follows.clone(), conflicts.clone(), i.clone().try_into().unwrap());
        if ok {
            break;
        }
    }
    let mut happy = 0;
    for conflict in conflicts.clone() {
        println!("Конфликт между");
        for side in conflict.sides {
            println!("{}", rule_to_string(side));
        }
        if conflict.done {
            println!("разрешился при k = {}", conflict.resolved);
            happy += 1;
        } else if conflict.resolved != 0 {
            println!("в подвешенном состоянии при k = {}", conflict.resolved);
        } else {
            println!("не был разрешён при k = {}", k.clone());
        }
    }
    if happy == conflicts.len() {
        println!("Конфлитков нет (или все были разрешены)");
    }
}
