import sys, time, array

cups = [int(l) for l in sys.stdin.readline().strip()]


class Game:
    def __init__(self, cups):     
        self.t = time.time()
        self.max = cups[0]
        self.cups = array.array('L')
        self.cups.extend([0] * (len(cups) + 1))
        self.head = cups[0]
        prev = cups[0]
        for cup in cups[1:]:
            if cup > self.max:
                self.max = cup
            self.cups[prev] = cup
        self.cups[cup] = self.head
        self.current_cup = self.head

    def pop_cups(self, num):
        popped_cups = []        
        cur = self.cups[self.current_cup]
        while len(popped_cups) < num:
            popped_cups.append(cur)
            cur = self.cups[cur]
            self.cups[cur] = 0
        self.cups[self.current_cup] = cur
        return popped_cups

    def find_destination_cup(self, label):
        while True:
            if label < 1:
                label = self.max
            if self.cups[label] != 0:
                return label
            label -= 1

    def get_cups(self):
        cups = []
        cur = self.head
        while self.cups[cur] != self.head:
            cups.append(cur)
            cur = self.cups[cur]
        return cups

    def fmt_c(self, node):
        if self.current_cup == node:
            return f"({node})"
        else:
            return str(node)

    def move(self, round):
        if time.time() - self.t > 5:
            print(f"-- move {round} --")        
            self.t = time.time()
        print(f"cups: {' '.join(self.fmt_c(n) for n in self.get_cups())}")
        popped_cups = self.pop_cups(3)
        print(f"pick up: {', '.join(str(c) for c in popped_cups)}")
        dest_cup = self.find_destination_cup(self.current_cup - 1)
        print(f"destination: {dest_cup}")
        oldnext = self.cups[dest_cup]
        cur = dest_cup
        while popped_cups:
            cup = popped_cups.pop()
            self.cups[cur] = cup
            cur = cup
        self.cups[cur] = oldnext

        self.current_cup = self.cups[self.current_cup]

    def print_final(self):
        print(f"-- final --")        
        #print(f"cups: {' '.join(self.fmt_c(n) for n in self.get_cups())}")

    def answer(self):
        cur = self.cups[1]
        cups_answer = []
        while cur != 1:
            cups_answer.append(cur)
            cur = self.cups[cur]
        return ''.join(str(c) for c in cups_answer)

    def answer2(self):
        first = self.cups[1]
        second = self.cups[first]
        print(first)
        print(second)
        return first * second

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
