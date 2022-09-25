expr1 = input("Expression 1: ")
expr2 = input("Expression 2: ")

class term():
    def __init__(self, value, var):
        self.value = value
        self.var = var
        self.children = []

def parse_constructor(expr, ptr):
    children = []
    curr = ''
    saw_const = 0
    while ptr <= len(expr) - 1 and expr[ptr] != ')':
        if ptr <= len(expr) - 1 and expr[ptr] == ' ':
            ptr += 1
            continue
        if ptr == len(expr) - 1:
            print("Нарушение синтаксиса: скобки не сбалансированы -> ", expr)
            exit()
        if expr[ptr] == '(':
            if curr == '':
                print("Нарушение синтаксиса: встречен безымянный конструктор -> ", expr)
                exit()
            n = term(curr, 0)
            ptr += 1
            n.children, ptr = parse_constructor(expr, ptr)
            children.append(n)
            saw_const = 1
            continue
        if expr[ptr] == ',':
            if curr == '':
                print("Нарушение синтаксиса: встречен пустой терм -> ", expr)
                exit()
            if not saw_const:
                n = term(curr, 1)
                children.append(n)
            curr = ''
            ptr += 1
            saw_const = 0
            continue
        if expr[ptr].isdigit() and curr == '':
            print("Нарушение синтаксиса: имя терма начинается с цифры -> ", expr)
            exit()
        if not expr[ptr].isalpha() and not expr[ptr].isdigit():
            print("Нарушение синтаксиса: встречен запрещённый символ -> ", expr)
            exit()
        curr += expr[ptr]
        ptr += 1
    if curr == '' and expr[ptr - 1] != '(':
        print("Нарушение синтаксиса: пустое выражение -> ", expr)
        exit()
    if ptr == len(expr):
        print("Нарушение синтаксиса: скобки не сбалансированы -> ", expr)
        exit()
    if curr != '' and not saw_const:
        n = term(curr, 1)
        children.append(n)
    ptr += 1
    return (children, ptr)

def start_parse(expr):
    curr = ''
    ptr = 0
    while ptr <= len(expr) - 1 and expr[ptr] != '(':
        while ptr <= len(expr) - 1 and expr[ptr] == ' ':
            ptr += 1
        if curr == '' and expr[ptr].isdigit():
            print("Нарушение синтаксиса:", expr)
            exit()
        if not expr[ptr].isdigit() and not expr[ptr].isalpha():
            print("Нарушение синтаксиса:", expr)
            exit()
        curr += expr[ptr]
        ptr += 1
    if ptr <= len(expr) - 1 and expr[ptr] == '(':
        n = term(curr, 0)
        ptr += 1
        n.children, _ = parse_constructor(expr, ptr)
        return n
    if curr == '':
        print("Нарушение синтаксиса:", expr)
        exit()
    n = term(curr, 1)
    return n

terms1 = start_parse(expr1)
terms2 = start_parse(expr2)

class eq_term():
    def __init__(self, instances, eq, terms):
        self.instances = instances
        self.eq = eq
        self.terms = terms

U = [eq_term(0, [term('♥', 1)], [terms1, terms2])]
T = []

def fill_eq(U, terms):
    for item in terms:
        if item.var == 1:
            exists = 0
            for eq in U:
                for val in eq.eq:
                    if val.value == item.value:
                        eq.instances += 1
                        exists = 1
                        continue
            if exists == 0:
                U.append(eq_term(1, [item], []))
        else:
            U = fill_eq(U, item.children)
    return U

def full_term(term):
    out = ''
    out += term.value
    if term.var == 0:
        out += '('
        empty = 1
        for child in term.children:
            out += full_term(child)
            out += ', '
            empty = 0
        if empty == 0:
            out = out[:-2]
        out += ')'
    return out

def print_U(U):
    out = ''
    for item in U:
        out = '[' + str(item.instances) + '] {'
        for eqs in item.eq:
            out += eqs.value + ', '
        out = out[:-2] + '} = ('
        empty = 1
        for term in item.terms:
            out += full_term(term) + ', '
            empty = 0
        if empty == 0:
            out = out[:-2]
        out += ')'
        print(out)

def add_vars(eqs, neqs):
    for neq in neqs:
        present = 0
        for eq in eqs:
            if eq.value == neq.value:
                present = 1
        if present == 0:
            eqs.append(neq)
    return eqs

def add_terms(terms, nterms):
    for nterm in nterms:
        present = 0
        for term in terms:
            if full_term(term) == full_term(nterm):
                present = 1
        if present == 0:
            terms.append(nterm)
    return terms

def append_F(F, v_t):
    for item in F:
        for eq in item.eq:
            for neq in v_t.eq:
                if neq.value == eq.value:
                    item.eq = add_vars(item.eq, v_t.eq)
                    item.terms = add_terms(item.terms, v_t.terms)
                    return F
    F.append(v_t)
    return F

def extract_vars(F, list):
    for term in list:
        if term.var == 1:
            F = append_F(F, eq_term(0, [term], []))
        else:
            F = extract_vars(F, term.children)
    return F

def separateCF(list, F):
    C = None
    const_name = None
    const_len = None
    consts = []
    vars = []
    for trm in list:
        if trm.var == 0:
            if const_name is not None:
                if trm.value != const_name or len(trm.children) != const_len:
                    print("Унификация невозможна: расхождение в конструкторах")
                    exit()
            else:
                const_name = trm.value
                const_len = len(trm.children)
            consts.append(trm)
        else:
            vars.append(trm)
    if len(vars) == 0:
        C = term(consts[0].value, 0)
        if len(consts) > 1:
            for i in range(0, const_len):
                n = []
                for item in consts:
                    n.append(item.children[i])
                a, b = separateCF(n, F)
                C.children.append(a)
                for entry in b:
                    F = append_F(F, entry)
        else:
            F = extract_vars(F, consts[0].children)
            C.children = consts[0].children
    else:
        C = term(vars[0].value, 1)
        if const_len is not None:
            for i in range(0, const_len):
                n = []
                for item in consts:
                    n.append(item.children[i])
                if len(n) > 1:
                    _, b = separateCF(n, F)
                    for entry in b:
                        F = append_F(F, entry)
        F = append_F(F, eq_term(0, vars, consts))
    return (C, F)

def getCF(U, F, ind):
    for item in U:
        item.instances = 0
    for item in U:
        U = fill_eq(U, item.terms)
    failstate = len(U)
    for item in U:
        if item.instances == 0:
            failstate = 0
            if len(item.terms) == 0:
                item.terms.append(term('T' + str(ind), 1))
                out = eq_term(0, item.eq, item.terms)
                F.remove(item)
                T.append(out)
                return(ind + 1, F)
            C, F = separateCF(item.terms, F)
            U.remove(item)
            T.append(eq_term(0, item.eq, [C]))
            break
    if failstate > 0:
        print("Унификация невозможна: цикл")
        exit()
    return (ind, F)

F = []

def unique(F, eqs):
    for item in F:
        for eqq in item.eq:
            for eq in eqs:
                if eqs != item.eq and eq.value == eqq.value:
                    return False
    return True


def clear_empty(F):
    for item in reversed(F[:]):
        item.instances = 0
        if len(item.terms) == 0 and not unique(F, item.eq):
            F.remove(item)
    return F

def sub_unify(to, eqs, what):
    for i in range(0, len(what)):
        if what[i].var == 1:
            for eq in eqs:
                if what[i].value == eq.value:
                    what[i] = to
        else:
            what[i].children = sub_unify(to, eqs, what[i].children)
    return what

def merge_equal(A):
    for i in range(len(A) - 1, -1, -1):
        for j in range(i - 1, -1, -1):
            if full_term(A[i].terms[0]) == full_term(A[j].terms[0]):
                A[j].eq = add_vars(A[j].eq, A[i].eq)
                A[i].eq = []
                break
    return A

def unify_system(A):
    for i in range(len(A) - 1, -1, -1):
        for j in range (i - 1, -1, -1):
            A[j].terms = sub_unify(A[i].terms[0], A[i].eq, A[j].terms)
    A = merge_equal(A)
    for item in A[:]:
        if len(item.eq) == 0:
            A.remove(item)
    return A

def MartMont(U, F, ind):
    while len(U) > 0:
        ind, F = getCF(U, F, ind)
        F = clear_empty(F)
        U = F

def print_T(T):
    print("Унифицированное выражение:")
    print("{" + T[0].eq[0].value + "} = (" + full_term(T[0].terms[0]) + ")")
    print("Правила унификации:")
    for i in range(1, len(T)):
        out = '{'
        for eq in T[i].eq:
            out += eq.value + ', '
        out = out[:-2] + '} = ('
        for term in T[i].terms:
            out += full_term(term) + ', '
        out = out[:-2] + ')'
        print(out)

MartMont(U, F, 1)
T = unify_system(T)
print_T(T)
