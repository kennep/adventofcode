#include <string>
#include <iostream>
#include <algorithm>
#include <set>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

struct Card {
	vector<int> winning_numbers;
	vector<int> my_numbers;
	int num_wins;
	int num_cards;

	Card() {
		num_wins = 0;
		num_cards = 1;
	}
};

class AoC04 : public Case
{
private:
	vector<Card> cards;

	void split_and_parse(const std::string& input, vector<int>& dest)
	{
		parse_list(input, dest);
		sort(dest.begin(), dest.end());
	}

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
		auto card_s = split(inputline, ": ", 2);
		auto parts = split(card_s[1], " | ", 2);
		
		Card card;
		split_and_parse(parts[0], card.winning_numbers);
		split_and_parse(parts[1], card.my_numbers);

		cards.push_back(card);
	}
	virtual void solve()
	{
		// Part A
		auto points = 0;
		for(auto& card : cards) {
			vector<int> intersection;
			set_intersection(
				card.winning_numbers.begin(), card.winning_numbers.end(),
				card.my_numbers.begin(), card.my_numbers.end(),
				inserter(intersection, intersection.begin())
				);
			cout << "Winning numbers: " << intersection << endl;
			card.num_wins = intersection.size();
			points += intersection.empty() ? 0 : 1 << (intersection.size() - 1);
		}
		cout << "Total points: " << points << endl;

		// Part B
		for (auto i = 0; i < cards.size(); ++i) {
			auto card = cards[i];
			for (auto j = i + 1; j < cards.size() && j < i + card.num_wins + 1; ++j) {
				cards[j].num_cards += card.num_cards;
			}
		}

		auto num_cards = 0;
		for (const auto& card : cards) {
			num_cards += card.num_cards;
		}
		
		cout << "Total number of cards: " << num_cards << endl;
	}
};


int main(int argc, char *argv[])
{
	return AoC04().run(argc, argv);
}
