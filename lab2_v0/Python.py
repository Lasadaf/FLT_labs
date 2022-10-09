import copy

expr = input("Выражение: ")
E = input("Пустая строка: ")
if E != "" :
    print("Нарушение синтаксиса: пустая строка не пустая")
    exit()
rules = []
while True:
    rule = input("Правило/пустая строка для окончания ввода: ")
    if rule != "" :
        rules.append(rule)
        continue
    break

def strip_spaces(expr):
    nexpr = ""
    for char in expr:
        if char != ' ':
            nexpr += char
    return nexpr

expr = strip_spaces(expr)
for r in range(0, len(rules)):
    rules[r] = strip_spaces(rules[r])

class Term:
    def __init__(self, value, var):
        self.value = value
        self.var = var
        self.left = None
        self.right = None

eps = Term('(eps)', 2)
consts = []

def parse_regex(expr, ptr, rule):
    if ptr >= len(expr):
        return (eps, ptr)
    if expr[ptr] == ')':
        return (eps, ptr)
    if expr[ptr] == '=':
        return (eps, ptr)
    if expr[ptr] == '*':
        return (eps, ptr)
    if expr[ptr] == '|':
        return (eps, ptr)
    if expr[ptr] == '(':
        left, ptr = parse_regex(expr, ptr + 1, rule)
        if ptr >= len(expr):
            print("Нарушение синтаксиса: несбалансированные скобки")
            exit()
        if expr[ptr] == ')':
            unary, ptr = parse_unary(expr, ptr + 1)
            if unary != eps:
                unary.left = left
                return (unary, ptr)
            return (left, ptr)
        binary, ptr = parse_binary(expr, ptr)
        right, ptr = parse_regex(expr, ptr, rule)
        binary.left = left
        binary.right = right
        if ptr >= len(expr) or expr[ptr] != ')':
            print("Нарушение синтаксиса: несбалансированные скобки")
            exit()
        unary, ptr = parse_unary(expr, ptr + 1)
        if unary != eps:
            unary.left = binary
            return(unary, ptr)
        return (binary, ptr)
    symbol, ptr = parse_symbol(expr, ptr, rule)
    unary, ptr = parse_unary(expr, ptr)
    if unary != eps:
        unary.left = symbol
        return (unary, ptr)
    return (symbol, ptr)


def parse_binary(expr, ptr):
    if ptr < len(expr):
        if expr[ptr] == '|':
            n = Term('|', 2)
            return (n, ptr + 1)
        n = Term('++', 2)
        return (n, ptr)
    return (eps, ptr)

def parse_symbol(expr, ptr, rule):
    if ptr < len(expr):
        if not expr[ptr].isalpha():
            print("Нарушение синтаксиса: запрещенный символ: ", expr[ptr])
            exit()
        n = None
        if not rule:
            n = Term(expr[ptr], 0)
            consts.append(n)
        else:
            n = Term(expr[ptr], 1)
            for c in consts:
                if c.value == expr[ptr]:
                    n = Term(expr[ptr], 0)
        return (n, ptr + 1)
    return (eps, ptr)

def parse_unary(expr, ptr):
    if ptr < len(expr):
        if expr[ptr] == '*':
            n = Term('*', 2)
            return (n, ptr + 1)
        return (eps, ptr)
    return (eps, ptr)

def parse_rule(expr, ptr, rule):
    left, ptr = parse_regex(expr, ptr, rule)
    if expr[ptr] != '=':
        print("Нарушение синтаксиса: в одном из правил отсутствует знак равенства")
        exit()
    right, ptr = parse_regex(expr, ptr + 1, rule)
    return (left, right)

reg, _ = parse_regex(expr, 0, False)
head = Term('null', 2)
head.left = reg

class Rule:
    def __init__(self, right, left):
        self.right = right
        self.left = left

norms = []

for rule in rules:
    l, r = parse_rule(rule, 0, True)
    norms.append(Rule(r, l))

def reg_to_string(reg):
    out = ""
    if reg.value == '|' or reg.value == '++' or (reg.value == '*' and reg.left is not None and reg.left.value == '*'):
        out += "("
    if reg.left is not None:
        out += reg_to_string(reg.left)
    if (reg.value == '*' and reg.left is not None and reg.left.value == '*'):
        out += ")"
    out += reg.value
    if reg.right is not None:
        out += reg_to_string(reg.right)
    if reg.value == '|' or reg.value == '++':
        out += ")"
    return out

class EqTerm:
    def __init__(self, instances, vars, terms):
        self.instances = instances
        self.vars = vars
        self.terms = terms

def print_U(U):
    out = ''
    for item in U:
        out = '[' + str(item.instances) + '] {'
        for eqs in item.vars:
            out += eqs.value + ', '
        out = out[:-2] + '} = ('
        empty = 1
        for term in item.terms:
            if term is not None:
                out += reg_to_string(term) + ', '
                empty = 0
        if empty == 0:
            out = out[:-2]
        out += ')'
        print(out)

def insert_carefully(terms, consts):
    if len(terms) == 0:
        first = reg_to_string(consts[0])
        for const in consts:
            if reg_to_string(const) != first:
                return (terms, True)
        return (consts, False)
    for const in consts:
        for term in terms:
            if reg_to_string(const) != reg_to_string(term):
                return (terms, True)
    return (terms, False)

def conflicts(current, vars, consts):
    for var in vars:
        for curr in current:
            for eq in curr.vars:
                if eq.value == var.value:
                    for const in consts:
                        for term in curr.terms:
                            if reg_to_string(term) != reg_to_string(const):
                                return True
    return False

def insert_terms(F, vars, consts, current):
    inserted = False
    for var in vars:
        for i in range(0, len(F)):
            for eq in F[i].vars:
                if eq.value == var.value:
                    if inserted:
                        return (F, current, True)
                    F[i].terms, err = insert_carefully(F[i].terms, consts)
                    if err:
                        return (F, current, True)
                    err = conflicts(current, vars, consts)
                    if err:
                        return (F, current, True)
                    current.append(EqTerm(0, vars, consts))
                    inserted = True
    if not inserted:
        err = conflicts(current, vars, consts)
        if err:
            return (F, current, True)
        current.append(EqTerm(0, vars, consts))
        F.append(EqTerm(0, vars, consts))
    return (F, current, False)

def separateCF(terms, F, current):
    #print("i want to separate:")
    #for term in terms:
        #if term is not None:
            #print(reg_to_string(term))
        #else:
            #print("None")
    C = None
    op_name = None
    ops = []
    vars = []
    consts = []
    for term in terms:
        if term is None:
            for t in terms:
                if t is not None:
                    #print("one term was None, but not all")
                    return (C, F, current, True)
            #print("all terms were None")
            return (C, F, current, False)
        if term.var == 2 and term.value != eps.value:
            if op_name is not None:
                if term.value != op_name:
                    #print("ops were different")
                    return (C, F, current, True)
            else:
                op_name = term.value
            ops.append(term)
        else:
            if term.var == 1:
                vars.append(term)
            else:
                consts.append(term)
    if len(vars) == 0 and len(consts) == 0:
        #print("len vars == len consts == 0")
        C = ops[0]
        n = []
        for op in ops:
            n.append(op.left)
        l, new, current, err = separateCF(n, F, current)
        if err:
            #print("separate for left failed")
            return (C, F, current, True)
        C.left = l
        for entry in new:
            F, current, err = insert_terms(F, entry.vars, entry.terms, current)
            if err:
                #print("insert for left failed")
                return (C, F, current, True)
        n = []
        for op in ops:
            n.append(op.right)
        l, new, current, err = separateCF(n, F, current)
        if err:
            #print("separate for right failed")
            return (C, F, current, True)
        C.right = l
        for entry in new:
            F, current, err = insert_terms(F, entry.vars, entry.terms, current)
            if err:
                #print("insert for right failed")
                return (C, F, current, True)
    elif len(vars) == 0:
        if len(ops) > 0:
            #print("ops with no vars")
            return (C, F, current, True)
        curr = consts[0]
        C = curr
        for con in consts:
            if con.value != curr.value:
                #print(con.value + " != " + curr.value)
                return (C, F, current, True)
    else:
        if len(consts) != 0 and len(ops) != 0:
            #print("const will be equal to op")
            return (C, F, current, True)
        if len(consts) != 0:
            #print("vars and consts")
            C = consts[0]
            F, current, err = insert_terms(F, vars, consts, current)
            if err:
                #print("couldn't insert consts")
                return (C, F, current, True)
        else:
            #print("vars and ops")
            C = vars[0]
            F, current, err = insert_terms(F, [C], ops, current)
            if err:
                #print("couldn't insert vars and consts")
                return (C, F, current, True)
            n = []
            for op in ops:
                n.append(op.left)
            if len(n) > 1:
                _, new, current, err = separateCF(n, F, current)
                if err:
                    #print("separate with _ left failed")
                    return (C, F, current, True)
                for entry in new:
                    F, current, err = insert_terms(F, entry.vars, entry.terms, current)
                    if err:
                        #print("insert with _ left failed")
                        return (C, F, current, True)
            n = []
            for op in ops:
                n.append(op.right)
            if len(n) > 1:
                _, new, current, err = separateCF(n, F, current)
                if err:
                    #print("separate with _ right failed")
                    return (C, F, current, True)
                for entry in new:
                    F, current, err = insert_terms(F, entry.vars, entry.terms, current)
                    if err:
                        #print("insert with _ right failed")
                        return (C, F, current, True)
    #print("separated")
    return (C, F, current, False)

def fill_eq(U, term):
    if term.var == 1:
        exists = 0
        for eq in U:
            for val in eq.vars:
                if val.value == term.value:
                    eq.instances += 1
                    exists = 1
                    continue
        if exists == 0:
            U.append(EqTerm(1, [term], []))
    elif term.var == 2:
        if term.left is not None:
            U = fill_eq(U, term.left)
        if term.right is not None:
            U = fill_eq(U, term.right)
    return U

def getCF(U, F, T, current):
    for i in range(0, len(U)):
        U[i].instances = 0
    for i in range(0, len(U)):
        for j in range(0, len(U[i].terms)):
            U = fill_eq(U, U[i].terms[j])
    #print("filled U:")
    #print_U(U)
    #print("--------U------------")
    failstate = len(U)
    for item in U:
        if item.instances == 0:
            if len(item.terms) == 1:
                T.append(item)
                F.remove(item)
                return(F, T, current, False)
            failstate = 0
            C, F, current, err = separateCF(item.terms, F, current)
            if err:
                return(F, T, current, True)
            U.remove(item)
            T.append(EqTerm(0, item.vars, [C]))
            break
    if failstate > 0:
        return (F, T, current, True)
    return (F, T, current, False)

def sub_unify(to, eqs, what):
    if what.var == 1:
        for eq in eqs:
            if what.value == eq.value:
                what = to
    elif what.var == 2:
        if what.left is not None:
            what.left = sub_unify(to, eqs, what.left)
        if what.right is not None:
            what.right = sub_unify(to, eqs, what.right)
    return what

def magic(T):
    #print("my T is cool:")
    #print_U(T)
    #print("----------T----------")
    for i in range(len(T) - 1, -1, -1):
        for j in range (i - 1, -1, -1):
            T[j].terms = [sub_unify(T[i].terms[0], T[i].vars, T[j].terms[0])]
    return T

def try_to_unify(reg, nrm):
    #print("lets change: " + reg_to_string(nrm.left) + " to " + reg_to_string(nrm.right))
    U = [EqTerm(0, [Term('♥', 0)], [reg, nrm.left])]
    T = []
    F = []
    current = []
    while len(U) > 0:
        #print("I'm about to call getCF with:")
        #print("--------U---------")
        #print_U(U)
        #print("--------F---------")
        #print_U(F)
        #print("--------T---------")
        #print_U(T)
        #print("--------current---------")
        #print_U(current)
        F, T, current, err = getCF(U, F, T, current)
        if err:
            #print("getCF returned error")
            return (reg, True)
        #print("Finished getCF with the following:")
        #print("--------U---------")
        #print_U(U)
        #print("--------F---------")
        #print_U(F)
        #print("--------T---------")
        #print_U(T)
        #print("--------current---------")
        #print_U(current)
        U = F
    A = []
    A.append(nrm.right)
    T[0].terms = A
    T = magic(T)
    #print(reg_to_string(T[0].terms[0]))
    #print("i unified")
    return (T[0].terms[0], False)

def unify_or_ignore(reg, nrm):
    somethinghappened = False
    new, err = try_to_unify(reg, nrm)
    if not err:
        #print("i didn't ignore")
        #print(reg_to_string(new))
        reg = new
        somethinghappened = True
    return (reg, somethinghappened)

def bfs_in_python_is_cancer(head, nrms):
    for nrm in nrms:
        head, cont = unify_or_ignore(head, copy.deepcopy(nrm))
        if cont:
            return (head, True)
    if head.left is not None:
        head.left, cont = bfs_in_python_is_cancer(head.left, nrms)
        if cont:
            return (head, True)
    if head.right is not None:
        head.right, cont = bfs_in_python_is_cancer(head.right, nrms)
        if cont:
            return (head, True)
    return (head, False)

##print(reg_to_string(head.left))

cancer = True
iter = 0
while cancer:
    head.left, cancer = bfs_in_python_is_cancer(head.left, norms)
    iter += 1
    if iter % 500 == 0:
        print("Прошло 500 итераций. Скорее всего произошло зацикливание. Текущий результат:")
        print(reg_to_string(head.left))
        Y = input("Продолжить? [Y/N] ")
        if Y == 'Y':
            continue
        break

print("Нормализованное выражение:")
print(reg_to_string(head.left))
