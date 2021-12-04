import sys

player1_deck = []
player2_deck = []

for player_deck in player1_deck, player2_deck:
    sys.stdin.readline()
    while True:
        l = sys.stdin.readline().strip()
        if not l: break
        player_deck.append(int(l))

def play(game, player1_deck, player2_deck):
    round = 1
    seen_configurations = set()
    while player1_deck and player2_deck:
        print(f"-- Round {round} in game {game} --")
        print(f"Player 1's deck: {', '.join(str(s) for s in player1_deck)}")
        print(f"Player 2's deck: {', '.join(str(s) for s in player2_deck)}")
        configuration = ",".join(str(s) for s in player1_deck) + "|" + ",".join(str(s) for s in player2_deck)
        if configuration in seen_configurations:
            print(f"-- This configuration has previously been seen - ending game in win for player 1!")
            return 'player1'
        seen_configurations.add(configuration)

        player1_card = player1_deck.pop(0)
        player2_card = player2_deck.pop(0)
        print(f"Player 1 plays: {player1_card}")
        print(f"Player 2 plays: {player2_card}")
        if len(player1_deck) >= player1_card and len(player2_deck) >= player2_card:
            print(f"-- Playing a new sub-game to determine winner of round")
            winner = play(
                game + 1,
                player1_deck[:player1_card].copy(),
                player2_deck[:player2_card].copy()
            )
        elif player1_card > player2_card:
            winner = 'player1'
        else:
            winner = 'player2'


        if winner == 'player1':
            print(f"Player 1 wins round {round} of game {game}!")
            player1_deck.append(player1_card)
            player1_deck.append(player2_card)
        else:
            print(f"Player 2 wins round {round} of game {game}!")
            player2_deck.append(player2_card)
            player2_deck.append(player1_card)
        round += 1
    if player1_deck:
        return 'player1'
    else:
        return 'player2'

winner = play(1, player1_deck, player2_deck)
print("== Post-game results ==")
print(f"Player 1's deck: {', '.join(str(s) for s in player1_deck)}")
print(f"Player 2's deck: {', '.join(str(s) for s in player2_deck)}")

if winner == 'player1':
    player_deck = player1_deck
else:
    player_deck = player2_deck

multiplier = 1
score = 0
while player_deck:
    score += player_deck.pop() * multiplier
    multiplier += 1
print(f"Winning player's score: {score}")