rules = []
log = []
n_str = input("N = ")
in_str = ""
while True:
    in_str = input("Правило: ")
    if in_str == "":
        break
    rules.append(in_str)

def del_whitespaces(rule):
    new_rule = ""
    for char in rule:
        if char != ' ' and char != "\t" and char != "\r" and char != "\n":
            new_rule += char
    return new_rule

for i in range(0, len(rules)):
    rules[i] = del_whitespaces(rules[i])

class Rule_body:
    def __init__(self, pre, post):
        self.pre = pre
        self.post = post

class Rule:
    def __init__(self, head, bodies):
        self.head = head
        self.bodies = bodies

class Conflict:
    def __init__(self, sides):
        self.sides = sides
        self.resolved = -1
        self.done = False

class State:
    def __init__(self, id, roots):
        self.id = id
        self.roots = roots
        self.rules = []

class Shift:
    def __init__(self, value, fr, to):
        self.value = value
        self.fr = fr
        self.to = to

class FOLLOW:
    def __init__(self, head):
        self.head = head
        self.bodies = []

def append_rule(output, head, bodies):
    for rule in output:
        if rule.head == head:
            for body in bodies:
                rule.bodies.append(body)
            return output
    output.append(Rule(head, bodies))
    return output


def parse_rules(rules):
    output = []
    for rule in rules:
        if rule[0] != '[':
            print("Синтаксическая ошибка: начальный нетерминал не найден: ", rule)
            exit()
        i = 1
        new_head = ""
        while i < len(rule) and rule[i] != ']':
            new_head += rule[i]
            i += 1
        if i == len(rule):
            print("Синтаксическая ошибка: начальный нетерминал не закрыт: ", rule)
            exit()
        i += 1
        if rule[i] != '-' or rule[i + 1] != '>':
            print("Синтаксическая ошибка: не найден разделитель в правиле: ", rule)
            exit()
        i += 2
        bodies = []
        new_body = ""
        while i < len(rule):
            if rule[i] == '|':
                bodies.append(new_body)
                new_body = ""
                if i == len(rule) - 1:
                    print("Синтаксическая ошибка: после знака | нет выражения: ", rule)
                    exit()
                i += 1
                continue
            if rule[i] == '[':
                while i < len(rule) and rule[i] != ']':
                    new_body += rule[i]
                    i += 1
                if i == len(rule):
                    print("Синтаксическая ошибка: нетерминал не закрыт: ", rule)
                    exit()
                new_body += ']'
                i += 1
                continue
            if not rule[i].isalpha() and not rule[i].isnumeric() and rule[i] != '(' and rule[i] != ')' and rule[i] != '*' and rule[i] != '+' and rule[i] != '-':
                print("Синтаксическая ошибка: неопознанный символ: ", rule)
                exit()
            new_body += rule[i]
            i += 1
        bodies.append(new_body)
        new_bodies = []
        for body in bodies:
            new_bodies.append(Rule_body("", '.' + body + '$'))
        output = append_rule(output, new_head, new_bodies)
    return output

def rule_to_string(rule):
    out = '[' + rule.head + "] -> "
    for body in rule.bodies:
        out += body.pre + body.post[:-1] + ' | '
    out = out[ : -3]
    return(out)

def filter_rules(rules):
    new_rules = []
    for rule in rules:
        new_rules = append_rule(new_rules, rule.head, rule.bodies)
    return new_rules

rules = parse_rules(rules)
#for rule in rules:
#    print_rule(rule)

def print_state(state):
    print("------------------------")
    for root in state.roots:
        print_rule(root)
    print("++++++++++++++++++++++++")
    for rule in state.rules:
        print_rule(rule)
    print("------------------------")
    print("")

def contained(head, heads):
    for h in heads:
        if h == head:
            return True
    return False

def inside(a, list):
    for b in list:
        if a == b:
            return True
    return False

def print_states(states):
    for state in states:
        label = ""
        for rule in state.rules:
            label += rule_to_string(rule) + "\n"
        print(str(state.id) + ' [shape = "rectangle"' + (' peripheries = 2' if state.id == 2  else '') + ' label = "' + label[:-1] + '"]')

def print_conflicts(conflicts):
    for conflict in conflicts:
        for side in conflict.sides:
            print(rule_to_string(side))
        print("--------------")

def present(body, bodies):
    for b in bodies:
        if b == body:
            return True
    return False

def formalize_roots(roots):
    new_roots = []
    for root in roots:
        head = root.head
        done = False
        for r in new_roots:
            if r.head == head:
                done = True
                for body in root.bodies:
                    if not present(body, r.bodies):
                        r.bodies.append(body)
        if not done:
            new_roots.append(Rule(head, root.bodies))
    return new_roots

def same_roots(roots, state):
    for i in range(0, len(roots)):
        for j in range(0, len(roots)):
            #print("checking", rule_to_string(roots[i]), "against", rule_to_string(state[i]))
            if rule_to_string(roots[i]) != rule_to_string(state[i]):
                #print("different -> False")
                return False
    return True

def build_LR(rules, roots, states, shifts, id):
    #time.sleep(1)
    state_rules = []
    exits = []
    for root in roots:
        for body in root.bodies:
            term = ""
            if body.post[1] != '$':
                if body.post[1] == '[':
                    i = 2
                    while body.post[i] != ']':
                        term += body.post[i]
                        i += 1
                else:
                    term += body.post[1]
                if not inside(term, exits):
                    exits.append(term)
    j = 0
    while j < len(rules):
        #print(rules[j].head)
        if inside(rules[j].head, exits) and not contained(rules[j].head, state_rules):
            #print(rules[j].head)
            state_rules.append(rules[j])
            reset = False
            for body in rules[j].bodies:
                term = ""
                if body.post[1] != '$':
                    if body.post[1] == '[':
                        i = 2
                        while body.post[i] != ']':
                            term += body.post[i]
                            i += 1
                    else:
                        term += body.post[1]
                    if not inside(term, exits):
                        exits.append(term)
                        j = 0
                        state_rules = []
                        reset = True
                        break
            if reset:
                continue
        j += 1
    state_rules = roots + state_rules
    new_state = State(id, roots)
    id += 1
    new_state.rules = state_rules
    states.append(new_state)
    for exit in exits:
        new_roots = []
        for rule in state_rules:
            for body in rule.bodies:
                if body.post[1] != '$':
                    affected = True
                    if body.post[1] == '[':
                        i = 2
                        cur_ex = ""
                        while body.post[i] != ']':
                            #if exit[i - 1] != body.post[1 + i]:
                                #affected = False
                                #break
                            cur_ex += body.post[i]
                            i += 1
                        if exit != cur_ex:
                            affected = False
                    else:
                        if body.post[1] != exit:
                            affected = False
                    if affected:
                        if body.post[1] == '[':
                            j = 1
                            while body.post[j - 1] != ']':
                                j += 1
                            new_roots.append(Rule(rule.head, [Rule_body(body.pre + body.post[1 : j], '.' + body.post[j:])]))
                        else:
                            new_roots.append(Rule(rule.head, [Rule_body(body.pre + body.post[1], '.' + body.post[2:])]))
        new_to = id
        loop = False
        new_roots = filter_rules(new_roots)
        #print("--entering same_roots with-----")
        #for root in new_roots:
        #    print(rule_to_string(root))
        #print("-------------------------------")
        for state in states:
            #label = ""
            #for rule in state.rules:
            #    label += rule_to_string(rule) + "\n"
            #print(str(state.id) + ' [shape = "rectangle"' + (' peripheries = 2' if state.id == 2  else '') + ' label = "' + label[:-1] + '"]')
            if same_roots(new_roots, state.roots):
                new_to = state.id
                loop = True
                break
        if not loop:
            #print("found new branch with exit", exit)
            new_roots = filter_rules(new_roots)
            new_shift = Shift(exit, new_state.id, id)
            shifts.append(new_shift)
            #print(str(new_shift.fr) + " -> " + str(new_shift.to) + ' [label = "' + new_shift.value + '"]')
            states, shifts, id = build_LR(rules, formalize_roots(new_roots), states, shifts, id)
            #print("New states:")
            #print_states(states)
            continue
        else:
            #print("found existing branch with exit", exit, "and id", new_to)
            new_shift = Shift(exit, new_state.id, new_to)
            shifts.append(new_shift)
            #print(str(new_shift.fr) + " -> " + str(new_shift.to) + ' [label = "' + new_shift.value + '"]')
            continue
    return states, shifts, id

def get_rule(rules, head):
    for rule in rules:
        if rule.head == head:
            return rule

def first_k(rules, rule, k, have1, cycle):
    #print(rule_to_string(rule))
    out = []
    r_bodies = []
    for body in rule.bodies:
        r_bodies.append(Rule_body(body.pre, body.post))
    rule1 = Rule(rule.head, r_bodies)
    for body in rule1.bodies:
        #print("looking at", body.pre + body.post)
        have = have1
        n_char = ""
        terminal = True
        i = 1
        danger = 0
        while body.post[i] != '$' and i - 1 < k:
            if body.post[i] == '[':
                if i == 1:
                    danger = 1
                terminal = False
                i = i + 1
                char = ""
                while body.post[i] != ']':
                    char += body.post[i]
                    i += 1
                i += 1
                #time.sleep(2)
                #print("for first i am in", rule_to_string(rule1), "and want to go to", char)
                new_rule = get_rule(rules, char)
                if new_rule.head == rule1.head and danger == 1:
                    danger = 2
                    #print("wants to loop at first symbol")
                    cycle += 1
                if (new_rule.head != rule1.head or have <= k) and (danger != 2 or cycle <=k):
                    #print("ok")
                    #print("now i am in", rule_to_string(new_rule))
                    pref = first_k(rules, new_rule, k, have, cycle)
                    #print(pref)
                    for p in pref:
                        rule1.bodies.append(Rule_body(body.pre, '.' + n_char + p + body.post[i : ]))
                    #print("Now rule is", rule_to_string(rule1))
                    break
            else:
                n_char += body.post[i]
                have += 1
                i += 1
        if terminal:
            if not inside(n_char, out):
                out.append(n_char)
    #print("in the end have", out)
    return out

def append_follow(head, follows, out):
    for p in out:
        if p.head == head:
            for f in follows:
                if not inside(f, p.bodies):
                    p.bodies.append(f)
            return out
    new_follow = FOLLOW(head)
    new_follow.bodies = follows
    out.append(new_follow)
    return out

def append_follows(to_head, from_head, out, k):
    #print("addind", from_head, "to", to_head)
    for p in out:
        if p.head == to_head:
            for p1 in out:
                if p1.head == from_head:
                    new_bodies = []
                    for b in p.bodies:
                        for b1 in p1.bodies:
                            new_body = ""
                            i = 0
                            while i < k and i < len(b):
                                new_body += b[i]
                                i += 1
                            j = 0
                            while i + j < k and j < len(b1):
                                new_body += b1[j]
                                j += 1
                            new_body += '$'
                            if not inside(new_body, new_bodies):
                                new_bodies.append(new_body)
                    p.bodies = new_bodies
                    #print("now", to_head, "has bodies", p.bodies)
                    break
    return out

def full(out, head, k):
    for o in out:
        if o.head == head:
            for body in o.bodies:
                if len(body) < k:
                    passed = False
                    i = 0
                    while i < len(body):
                        if body[i] == '$':
                            passed = True
                        i += 1
                    if not passed:
                        return False
    return True

def follow_k(rules, k):
    out = []
    S = FOLLOW('S')
    S.bodies.append('$')
    out.append(S)
    for rule in rules:
        for r in rules:
            for body in r.bodies:
                i = 1
                while body.post[i] != '$':
                    danger = 0
                    if body.post[i] == '[':
                        danger = i
                        new_head = ""
                        i += 1
                        while body.post[i] != ']':
                            new_head += body.post[i]
                            i += 1
                        #print("i will add", first_k(rules, Rule(r.head, [Rule_body('', body.post[i : ])]), k, 0, 0), "to follows of", new_head)
                        out = append_follow(new_head, first_k(rules, Rule(r.head, [Rule_body('', body.post[i : ])]), k, 0, 0), out)
                        continue
                    i += 1
    for rule in rules:
        #print("working with", rule_to_string(rule))
        for body in rule.bodies:
            i = 1
            while body.post[i] != '$':
                if body.post[i] == '[':
                    new_head = ""
                    i += 1
                    while body.post[i] != ']':
                        new_head += body.post[i]
                        i += 1
                    #print("filling", new_head)
                    i += 1
                    if rule.head != new_head or not full(out, rule.head, k):
                        out = append_follows(new_head, rule.head, out, k)
                    continue
                i += 1
    #print("in the end follows are for k = ", k)
    #for o in out:
    #    print("---", o.head, "---")
    #    print(o.bodies)
    return out

def print_follow(follow):
    print(follow.head)
    print(follow.bodies)

shifts = []
states = []
states, shifts, _ = build_LR(rules, [Rule('♥', [Rule_body('', '.[S]$')])], states, shifts, 1)
print("https://dreampuf.github.io/GraphvizOnline/")
print("digraph {")
for state in states:
    label = ""
    for rule in state.rules:
        label += rule_to_string(rule) + "\n"
    print(str(state.id) + ' [shape = "rectangle"' + (' peripheries = 2' if state.id == 2  else '') + ' label = "' + label[:-1] + '"]')
for shift in shifts:
    print(str(shift.fr) + " -> " + str(shift.to) + ' [label = "' + shift.value + '"]')
print("}")

def filter(follows, k):
    new_follows = []
    for follow in follows:
        if follow.head == 'S':
            new_follows.append(follow)
            continue
        new_follow = FOLLOW(follow.head)
        new_bodies = []
        for body in follow.bodies:
            i = 0
            new_body = ""
            while i < k and body[i] != '$':
                new_body += body[i]
                i += 1
            new_body += '$'
            if not inside (new_body, new_bodies):
                new_bodies.append(new_body)
        new_follow.bodies = new_bodies
        new_follows.append(new_follow)
    return new_follows

def collect_roots(state):
    roots = []
    for root in state.roots:
        for body in root.bodies:
            roots.append(Rule(root.head, [body]))
    return roots

def get_conflicts(states):
    conflict = []
    for state in states:
        if len(state.roots) > 1 or len(state.roots[0].bodies) > 1:
            for root in state.roots:
                for body in root.bodies:
                    if body.post[1] == '$':
                        new_conflict = collect_roots(state)
                        conflict.append(Conflict(new_conflict))
    return conflict

def get_follows(follows, head):
    for follow in follows:
        if follow.head == head:
            return follow.bodies

def filter_strs(list, k):
    new_list = []
    for l in list:
        #print(l)
        new_l = ""
        i = 0
        while i < k and l[i] != '$':
            new_l += l[i]
            i += 1
        new_list.append(new_l)
    return new_list

def equal(f1, f2):
    for f in f1:
        danger = True
        for ff in f2:
            if f == ff:
                #print("found", f, "==", ff)
                danger = False
        if danger:
            return 0
    for f in f2:
        danger = True
        for ff in f1:
            if f == ff:
                #print("found", f, "==", ff)
                danger = False
        if danger:
            return 0
    return 1

def intersect(f1, f2):
    for f in f1:
        for ff in f2:
            if f == ff:
                #print("intersection in", f, "==", ff)
                return 1
    return 0

def resolve(rules, follows, conflicts, k):
    ok = True
    for conflict in conflicts:
        comp = []
        for side in conflict.sides:
            head = side.head
            for body in side.bodies:
                if body.post[1] == '$':
                    cands = []
                    for f1 in get_follows(follows, head):
                        if not inside(f1, cands):
                            cands.append(f1)
                    cands = filter_strs(cands, k)
                    comp.append(cands)
                else:
                    firsts = first_k(rules, side, k, 0, 0)
                    cands = []
                    for f in firsts:
                        for f1 in get_follows(follows, head):
                            if not inside(f + f1, cands):
                                cands.append(f + f1)
                    cands = filter_strs(cands, k)
                    comp.append(cands)
        #for c in comp:
        #    print("---", side.head,"---")
        #    for cc in c:
        #        print(cc)
        #print("++++++++")
        equality = 0
        intersection = 0
        #print(comp)
        for i in range(0, len(comp)):
            for j in range(i + 1, len(comp)):
                equality += equal(comp[i], comp[j])
                intersection += intersect(comp[i], comp[j])
        #print("intersection =", intersection, ", equality =", equality)
        if intersection == 0:
            #print("Conflict between")
            #for side in conflict.sides:
            #    print(rule_to_string(side))
            #print("got resolved at k =", k)
            if not conflict.done:
                conflict.resolved = k
            conflict.done = True
            #print("nice")
            continue
        elif equality == len(comp):
            #print("Conflict between")
            #for side in conflict.sides:
            #    print(rule_to_string(side))
            #print("is potentially avoidable at k =", k)
            conflict.resolved = k
            ok = False
            #print("mid")
            continue
        else:
            conflict.resolved = -1
            conflict.done = False
            ok = False
            #print("rip")
        #print("Conflict between")
        #for side in conflict.sides:
        #    print(rule_to_string(side))
        #print("is serious at k =", k)
    return conflicts, ok

conflicts = get_conflicts(states)
#print_conflicts(conflicts)

#for i in range(1, 1 + int(n_str)):
#    follows = filter(follow_k(rules, i), i)
#    for f in follows:
#        print("---", f.head, i, "---")
#        ooo = ""
#        for ff in f.bodies:
#            ooo += ff + ", "
#        print(ooo)

for i in range(1, 1 + int(n_str)):
    follows = filter(follow_k(rules, i), i)
    conflicts, ok = resolve(rules, follows, conflicts, i)
    if ok:
        break
happy = 0
for conflict in conflicts:
    print("Конфликт между")
    for side in conflict.sides:
        print(rule_to_string(side))
    if conflict.done:
        print("разрешился при k =", conflict.resolved)
        happy += 1
    elif conflict.resolved != -1:
        print("в подвешенном состоянии при k =", conflict.resolved)
    else:
        print("не был разрешён при k =", int(n_str))
if len(conflicts) == happy:
    print("Конфликтов нет (или все были разрешены)")
