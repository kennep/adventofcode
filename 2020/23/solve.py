import sys

cups = [int(l) for l in sys.stdin.readline().strip()]

class Game:
    def __init__(self, cups):
        self.cups = cups
        self.current_cup_idx = 0

    def pop_cups(self, num):
        popped_cups = []
        idx = self.current_cup_idx + 1
        while len(popped_cups) < num and self.cups:
            idx = idx % len(self.cups)
            popped_cups.append(self.cups.pop(idx))
            if idx < self.current_cup_idx:
                self.current_cup_idx -= 1
            if self.current_cup_idx < 0:
                self.current_cup_idx = len(self.cups) - 1
        return popped_cups

    def find_destination_cup_idx(self, label):
        while label > 0:
            try:
                idx = self.cups.index(label)
                return idx
            except ValueError:
                label -= 1
        max_label = max(self.cups)
        return self.cups.index(max_label)

    def move(self, round):
        print(f"-- move {round} --")        
        def fmt_c(i):
            if i == self.current_cup_idx:
                return f"({self.cups[i]})"
            else:
                return str(self.cups[i])
        print(f"cups: {' '.join(fmt_c(i) for i in range(0, len(self.cups)))}")
        popped_cups = self.pop_cups(3)
        print(f"pick up: {', '.join(str(c) for c in popped_cups)}")
        current_cup_label = self.cups[self.current_cup_idx]
        dest_cup_idx = self.find_destination_cup_idx(current_cup_label - 1)
        print(f"destination: {self.cups[dest_cup_idx]}")
        for c in reversed(popped_cups):
            self.cups.insert(dest_cup_idx + 1, c)
            if self.current_cup_idx > dest_cup_idx:
                self.current_cup_idx += 1
        self.current_cup_idx = (self.current_cup_idx + 1) % len(self.cups)

    def print_final(self):
        print(f"-- final --")        
        def fmt_c(i):
            if i == self.current_cup_idx:
                return f"({self.cups[i]})"
            else:
                return str(self.cups[i])
        print(f"cups: {' '.join(fmt_c(i) for i in range(0, len(self.cups)))}")

    def answer(self):
        idx = self.cups.index(1)
        cups_answer = []
        while len(cups_answer) < len(self.cups) - 1:
            idx = (idx + 1) % len(self.cups)
            cups_answer.append(self.cups[idx])
        return ''.join(str(c) for c in cups_answer)

g = Game(cups)
for i in range(1, 101):
    g.move(i)

g.print_final()
print(g.answer())
