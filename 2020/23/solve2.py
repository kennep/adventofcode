import sys, time

cups = [int(l) for l in sys.stdin.readline().strip()]

class Node:
    def __init__(self, label, next):
        self.label = label
        self.next = next
        self.in_game = True

class Game:
    def __init__(self, cups):     
        self.t = time.time()
        self.max = cups[0]
        self.label_to_node = {}   
        head = Node(cups[0], None)
        self.label_to_node[cups[0]] = head
        prev = head
        for cup in cups[1:]:
            node = Node(cup, None)
            self.label_to_node[cup] = node
            if cup > self.max:
                self.max = cup
            if prev:
                prev.next = node
            prev = node
        node.next = head # Circular list

        self.head = head
        self.current_cup = head

    def pop_cups(self, num):
        popped_cups = []        
        cur = self.current_cup.next
        while len(popped_cups) < num:
            cur.in_game = False
            popped_cups.append(cur)
            cur = cur.next
        self.current_cup.next = cur
        return popped_cups

    def find_destination_cup(self, label):
        while True:
            if label < 1:
                label = self.max
            node = self.label_to_node[label]
            if node.in_game:
                return node
            label -= 1

    def get_cups(self):
        cups = []
        cur = self.head
        while cur.next != self.head:
            cups.append(cur)
            cur = cur.next
        return cups

    def fmt_c(self, node):
        if self.current_cup == node:
            return f"({node.label})"
        else:
            return str(node.label)

    def move(self, round):
        if time.time() - self.t > 5:
            print(f"-- move {round} --")        
            self.t = time.time()
        #print(f"cups: {' '.join(self.fmt_c(n) for n in self.get_cups())}")
        popped_cups = self.pop_cups(3)
        #print(f"pick up: {', '.join(str(c.label) for c in popped_cups)}")
        dest_cup = self.find_destination_cup(self.current_cup.label - 1)
        #print(f"destination: {dest_cup.label}")
        for c in popped_cups:
            c.in_game = True
        oldnext = dest_cup.next
        dest_cup.next = popped_cups[0]
        popped_cups[-1].next = oldnext
        self.current_cup = self.current_cup.next

    def print_final(self):
        print(f"-- final --")        
        #print(f"cups: {' '.join(self.fmt_c(n) for n in self.get_cups())}")

    def answer(self):
        one = self.label_to_node[1]
        cur = one.next
        cups_answer = []
        while cur != one:
            cups_answer.append(cur.label)
            cur = cur.next
        return ''.join(str(c) for c in cups_answer)

    def answer2(self):
        one = self.label_to_node[1]
        print(one.next.label)
        print(one.next.next.label)
        return one.next.label * one.next.next.label

cupsinput = cups + list(range(len(cups) + 1, 1000001))
#print(cupsinput[:100], cupsinput[-10:])
#sys.exit(1)
g = Game(cupsinput)
#g = Game(cups)
for i in range(1, 10000001):
#for i in range(1, 101):
    g.move(i)

g.print_final()
print(g.answer2())
