import sys

player1_deck = []
player2_deck = []

for player_deck in player1_deck, player2_deck:
    sys.stdin.readline()
    while True:
        l = sys.stdin.readline().strip()
        if not l: break
        player_deck.append(int(l))

round = 1
while player1_deck and player2_deck:
    print(f"-- Round {round} --")
    print(f"Player 1's deck: {', '.join(str(s) for s in player1_deck)}")
    print(f"Player 2's deck: {', '.join(str(s) for s in player2_deck)}")
    player1_card = player1_deck.pop(0)
    player2_card = player2_deck.pop(0)
    print(f"Player 1 plays: {player1_card}")
    print(f"Player 2 plays: {player2_card}")
    if player1_card > player2_card:
        print(f"Player 1 wins the round!")
        player1_deck.append(player1_card)
        player1_deck.append(player2_card)
    else:
        print(f"Player 2 wins the round!")
        player2_deck.append(player2_card)
        player2_deck.append(player1_card)
    round += 1

print("== Post-game results ==")
print(f"Player 1's deck: {', '.join(str(s) for s in player1_deck)}")
print(f"Player 2's deck: {', '.join(str(s) for s in player2_deck)}")

for player_deck in player1_deck, player2_deck:
    if not player_deck: continue
    multiplier = 1
    score = 0
    while player_deck:
        score += player_deck.pop() * multiplier
        multiplier += 1
    print(f"Winning player's score: {score}")