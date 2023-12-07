#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"
#include "ranges.h"

using namespace std;

enum Type {
	HighCard, Pair, TwoPairs, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind
};

const string Ranks =  "23456789TJQKA";
const string Ranks2 = "J23456789TQKA";

struct Game {
	string hand;
	int bid;

	vector<string> split_to_groups() const {
		auto s = hand;
		sort(s.begin(), s.end());
		vector<string> result;
		size_t lastidx = string::npos;
		for (char c : s) {
			string l;
			if (lastidx == -1 || (l = result[lastidx], l[l.length() - 1] != c)) {
				result.push_back(string(1, c));
				lastidx = result.size() - 1;
			}
			else {
				result[lastidx] = l + c;
			}
		}
		return result;
	}

	bool has_g_len(const vector<string>& groups, size_t len) const {
		return find_if(groups.begin(), groups.end(), [&len](const string& g) { return g.size() == len; }) != groups.end();
	}

	virtual Type type() const {
		auto g = split_to_groups();
		if (has_g_len(g, 5)) return FiveOfAKind;
		if (has_g_len(g, 4)) return FourOfAKind;
		if (has_g_len(g, 3) && has_g_len(g, 2)) return FullHouse;
		if (has_g_len(g, 3)) return ThreeOfAKind;
		int pairs = 0;
		for (const auto& a : g) {
			if (a.size() == 2) pairs++;
		}
		if (pairs == 2) return TwoPairs;
		if (pairs == 1) return Pair;
		return HighCard;
	}

	bool operator<(const Game& other) const {
		auto t1 = type();
		auto t2 = other.type();
		if (t1 != t2) return t1 < t2;

		for (auto i = 0; i < 5; ++i) {
			auto r1 = Ranks.find(hand[i]);
			auto r2 = Ranks.find(other.hand[i]);
			if (r1 != r2) return r1 < r2;
		}

		return false;
	}
};

struct Game2 : Game {
	bool has_g_len(const vector<string>& groups, size_t len, bool use_joker=true) const {
		auto j = use_joker ? j_len(groups) : 0;
		return find_if(groups.begin(), groups.end(), [&len, &j](const string& g) { return g[0] != 'J' && g.size() + j == len; }) != groups.end();
	}

	size_t j_len(const vector<string>& groups) const {
		auto result = find_if(groups.begin(), groups.end(), [](const string& g) { return g[0] == 'J'; });
		return result == groups.end() ? 0 : (*result).size();
	}

	virtual Type type() const {
		auto g = split_to_groups();
		auto j = j_len(g);
		if (j == 5) return FiveOfAKind;
		if (has_g_len(g, 5)) return FiveOfAKind;
		if (has_g_len(g, 4)) return FourOfAKind;
		if (has_g_len(g, 3, false) && has_g_len(g, 2, false)) return FullHouse;
		if (has_g_len(g, 3, false) && j >= 1) return FullHouse;
		if (has_g_len(g, 2, false) && j >= 2) return FullHouse;
		int pairs = 0;
		for (const auto& a : g) {
			if (a.size() == 2) pairs++;
		}
		if (pairs == 2) return j==1 ? FullHouse : TwoPairs;
		if (has_g_len(g, 3)) return ThreeOfAKind;
		if (pairs == 1) return Pair;
		if (j == 1) return Pair; // Joker plus another card
		return HighCard;
	}

	bool operator<(const Game& other) const {
		auto t1 = type();
		auto t2 = other.type();
		if (t1 != t2) return t1 < t2;

		for (auto i = 0; i < 5; ++i) {
			auto r1 = Ranks2.find(hand[i]);
			auto r2 = Ranks2.find(other.hand[i]);
			if (r1 != r2) return r1 < r2;
		}

		return false;
	}
};


ostream& operator<<(ostream& os, Type type)
{
	switch (type) {
	case FiveOfAKind: os << "Five of a kind"; break;
	case FourOfAKind: os << "Four of a kind"; break;
	case FullHouse: os << "Full house"; break;
	case ThreeOfAKind: os << "Three of a kind"; break;
	case TwoPairs: os << "Two pairs"; break;
	case Pair: os << "Pair"; break;
	case HighCard: os << "High card"; break;
	default: os << "UNKNOWN"; break;
	}
	return os;
}

ostream& operator<<(ostream& os, const Game& game)
{
	os << game.hand << " " << game.bid << " " << game.split_to_groups() << " " << game.type();
	return os;
}

ostream& operator<<(ostream& os, const Game2& game)
{
	os << game.hand << " " << game.bid << " " << game.split_to_groups() << " " << game.type();
	return os;
}


class AoC07 : public Case
{
private:
	vector<Game> games;
	vector<Game2> games2;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		auto split_line = split(inputline, " ", 2);
		
		Game g;
		g.hand = split_line[0];
		g.bid = stoi(split_line[1]);
		games.push_back(g);

		Game2 g2;
		g2.hand = split_line[0];
		g2.bid = stoi(split_line[1]);
		games2.push_back(g2);
	}

	virtual void solve()
	{
		sort(games.begin(), games.end());
		int rank = 1;
		int winnings = 0;
		for (const auto& g : games) {
			cout << g << endl;
			winnings += g.bid * rank++;
		}

		cout << "Winnings (A): " << winnings << endl;

		sort(games2.begin(), games2.end());
		int rank2 = 1;
		int winnings2 = 0;
		for (const auto& g : games2) {
			cout << g << endl;
			winnings2 += g.bid * rank2++;
		}

		cout << "Winnings (B): " << winnings2 << endl;
	}
};

int main(int argc, char *argv[])
{
	return AoC07().run(argc, argv);
}
