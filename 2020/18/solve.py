import sys
import re

token_rexp = re.compile(r"(?:[()*+])|(?:[0-9]+)")

expressions = [token_rexp.findall(l) for l in sys.stdin.readlines()]

def parse_expression(token_iter):
    expression = []
    while True:
        try:
            token = next(token_iter)
            if token == '(':
                expression.append(parse_expression(token_iter))
            elif token == ')':
                break
            elif token in '+*':
                expression.append(token)
            else:
                expression.append(int(token))
        except StopIteration:
            break
    return expression

def evaluate_expression(parse_tree):
    value = None
    op = lambda a, b: b
    for token in parse_tree:
        if isinstance(token, list):
            value = op(value, evaluate_expression(token))
        elif token == '+':
            op = lambda a, b: a + b
        elif token == '*':
            op = lambda a, b: a * b
        else:
            value = op(value, token)
    return value

sumvals = 0
for tokens in expressions:
    parse_tree = parse_expression(iter(tokens))
    value = evaluate_expression(parse_tree)        
    sumvals += value
    print(value)

print(sumvals)

