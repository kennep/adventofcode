import sys
import re

class Terminal:
    def __init__(self, symbol):
        self.symbol = symbol

    def __str__(self):
        return self.symbol

class Sequence:
    def __init__(self, rules):
        self.rules = rules

    def __str__(self):
        return ''.join([str(r) for r in self.rules])


class Alternates:
    def __init__(self, branches):
        self.branches = branches

    def __str__(self):
        return '(' + '|'.join(
            '(' + str(branch) + ')' for branch in self.branches
        ) + ')'

class UnprocessedRule:
    def __init__(self, rules, rulenum, definition):
        self.rules = rules
        self.rulenum = rulenum
        self.definition = definition

    def process(self):
        print("Process ", self.rulenum)
        if len(self.definition) == 1:
            term = self.definition[0]
            if term.startswith('"'):
                return Terminal(term[1])
        branches = []
        curdef = []
        for term in self.definition:
            if term == '|':
                assert curdef
                branches.append(Sequence(curdef))
                curdef = []
            else:
                referenced_rule = rules[term]
                if isinstance(referenced_rule, UnprocessedRule):
                    referenced_rule = referenced_rule.process()
                curdef.append(referenced_rule)
        if curdef:
            branches.append(Sequence(curdef))
        if len(branches) == 1:
            return branches[0]
        else:
            return Alternates(branches)


import sys

rules = {}

while True:
    line = sys.stdin.readline().strip()
    if not line: break
    rulenum, definition = line.split(": ", 2)
    definition = definition.split(' ')
    rules[rulenum] = UnprocessedRule(rules, rulenum, definition)

#8: 42 | 42 8
#11: 42 31 | 42 11 31
rule8 = []
rule11 = []
for i in range(1, 11):
    rule8 += ["42"]  * i
    rule8.append("|")
    rule11 += ["42"] * i
    rule11 += ["31"] * i
    rule11.append("|")

rules["8"] = UnprocessedRule(rules, "8", rule8[:-1])
rules["11"] = UnprocessedRule(rules, "11", rule11[:-1])

messages = [l.strip() for l in sys.stdin.readlines()]

rule_zero = rules["0"].process()
rexp = re.compile(str(rule_zero))

print(rule_zero)
for m in messages:
    matches = rexp.fullmatch(m)
    if matches:
        print(f"{m} matches {rexp}")
    else:
        print(f"{m} does not match {rexp}")
print(len([m for m in messages if rexp.fullmatch(m)]))
