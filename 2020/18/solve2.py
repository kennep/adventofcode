import sys
import re

token_rexp = re.compile(r"(?:[()*+])|(?:[0-9]+)")

expressions = [token_rexp.findall(l) for l in sys.stdin.readlines()]

def convert_token(t):
    if t in '()*+': return t
    try:
        return int(t)
    except ValueError:
        return t

expressions = [[convert_token(t) for t in e] for e in expressions]


# Expr: Sum * Sum
#       Sum
# Sum: Sum + Term
#      Term
# Term: ( Expr )
#       Num

def parse_num(tokens, pos):
    print("parse_num", tokens[pos:])
    if pos >= len(tokens): return None, pos
    token = tokens[pos]
    if isinstance(token, int):
        print("ACCEPT: NUM", token)
        return token, pos + 1
    else:
        return None, pos

def parse_paren(tokens, pos):
    print("parse_paren", tokens[pos:])
    if pos >= len(tokens): return None, pos
    token = tokens[pos]
    if token == '(':
        expr, final_pos = parse_expr(tokens, pos + 1)
        if final_pos < len(tokens) and tokens[final_pos] == ')':
            print("ACCEPT: PAREN", expr)
            return expr, final_pos + 1
        else:
            return None, pos
    else:
        return None, pos

def parse_term(tokens, pos):
    print("parse_term", tokens[pos:])
    if pos >= len(tokens): return None, pos
    val, final_pos = parse_paren(tokens, pos)
    if val:
        print("ACCEPT: TERM (paren)", val, pos)
        return val, final_pos
    val, final_pos = parse_num(tokens, pos)
    if val:
        print("ACCEPT: TERM (num)", val, pos)
        return val, final_pos
    return None, pos

def parse_term_expr(tokens, pos):
    print("parse_term_expr", tokens[pos:])
    if pos >= len(tokens): return None, pos
    term, final_pos = parse_term(tokens, pos)
    if term:
        print("DEBUG: ", term, final_pos)
        if final_pos < len(tokens) and tokens[final_pos] == '+':
            expr, final_pos = parse_term(tokens, final_pos + 1)
            if expr:
                add_terms = []
                try_pos = final_pos
                while True:
                    if try_pos >= len(tokens) or tokens[try_pos] != '+':
                        break
                    add_term, try_pos = parse_term(tokens, try_pos + 1)
                    if not add_term: break
                    add_terms.append(add_term)
                    final_pos = try_pos

                print("ACCEPT: TERM_EXPR", term, expr, add_terms)
                return ['+', term, expr] + add_terms, final_pos
    return None, pos

def parse_sum(tokens, pos):
    print("parse_sum", tokens[pos:])
    if pos >= len(tokens): return None, pos
    val, final_pos = parse_term_expr(tokens, pos)
    if val:
        print("ACCEPT: SUM (expr)", val)
        return val, final_pos
    val, final_pos = parse_term(tokens, final_pos)
    if val:
        print("ACCEPT: SUM (single)", val)
        return val, final_pos
    return None, pos

def parse_sum_expr(tokens, pos):
    print("parse_sum_expr", tokens[pos:])
    if pos >= len(tokens): return None, pos
    summ, final_pos = parse_sum(tokens, pos)
    if summ:
        if final_pos < len(tokens) and tokens[final_pos] == '*':
            expr, final_pos = parse_sum(tokens, final_pos + 1)
            if expr:
                add_sums = []
                try_pos = final_pos
                while True:
                    if try_pos >= len(tokens) or tokens[try_pos] != '*':
                        break
                    add_sum, try_pos = parse_sum(tokens, try_pos + 1)
                    if not add_sum: break
                    add_sums.append(add_sum)
                    final_pos = try_pos
                print("ACCEPT: SUM_EXPR", summ, expr, add_sums)
                return ['*', summ, expr] + add_sums, final_pos
    return None, pos

def parse_expr(tokens, pos):
    print("parse_expr", tokens[pos:])
    if pos >= len(tokens): return None, pos
    val, final_pos = parse_sum_expr(tokens, pos)
    if val:
        print("ACCEPT: EXPR (sum * expr)", val)
        return val, final_pos
    val, final_pos = parse_sum(tokens, final_pos)
    if val:
        print("ACCEPT: EXPR (sum)", val)
        return val, final_pos
    return None, pos

def parse_expression(tokens):
    tree, pos = parse_expr(tokens, 0)
    return tree


def multiply(values):
    r = 1
    for v in values:
        r *= v
    return r

def evaluate_expression(parse_tree):
    print("EVALUATE", parse_tree)
    op = parse_tree[0]
    if op == '+':
        op = sum
    else:
        op = multiply
    arguments = [evaluate_expression(p) if isinstance(p, list) else p for p in parse_tree[1:]]
    return op(arguments) 


sumvals = 0
for tokens in expressions:
    parse_tree = parse_expression(tokens)
    print(parse_tree)
    value = evaluate_expression(parse_tree)        
    sumvals += value
    print(value)

print(sumvals)

