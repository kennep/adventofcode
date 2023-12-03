#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <list>
#include <string>
#include <regex>
#include <ranges>
#include <string_view>
#include <numeric>

using namespace std;

bool open_file(const char * filename, ifstream &fd) {
	fd.open(filename);
	if (!fd.good())
	{
		cout << "Unable to open file: " << strerror(errno) << endl;
		return false;
	}
	return true;
}

enum Color { Red, Green, Blue};

struct Draw {
	Color color;
	int num_cubes;

	Draw(const Color& color, int num_cubes)
		: color(color), num_cubes(num_cubes)
	{
	}
};

typedef list<Draw> Round;

struct Game {
	int id;
	list<Round> rounds;

	Game() : id(0) {}

	Game(int id)
		: id(id) {
	}

	int power() const {
		map<Color, int> max_cubes;
		for (const auto& round : rounds) {
			for(const auto& draw: round) {
				auto pos = max_cubes.find(draw.color);
				if (pos == max_cubes.end() || (*pos).second < draw.num_cubes) {
					max_cubes[draw.color] = draw.num_cubes;
				}
			}
		}
		return accumulate(max_cubes.begin(), max_cubes.end(), 1, [](int prod, pair<Color, int> next) { return prod * next.second; });
	}
};


const std::regex game_regex("Game (\\d+): (.*)");
const std::regex draw_regex("\\s*(\\d+) ([a-z]+)");

Game parse_game(string& s)
{
	smatch match;

	if (!regex_match(s, match, game_regex)) {
		cout << "Invalid input: " << s << endl;
		return Game(0);
	}


	auto id = atoi(match[1].str().c_str());

	auto rounds = match[2].str();

	auto game = Game(id);

	auto split_strings = string_view(rounds) | ranges::views::split(';');
	for (const auto& round_rng : split_strings) {
		auto round_sv = string_view(round_rng);
		Round round;
		auto split_round = round_sv | ranges::views::split(',');
		for (const auto& draw_rng : split_round) {
			smatch draw_match;
			auto draw_str = string(string_view(draw_rng));
			if (!regex_match(draw_str, draw_match, draw_regex)) {
				cout << "Invalid input: " << draw_str << endl;
				continue;
			}
			auto count = atoi(draw_match[1].str().c_str());
			auto color_str = draw_match[2].str();
			Color color;
			if (color_str == "red") {
				color = Red;
			}
			else if (color_str == "green") {
				color = Green;
			}
			else if (color_str == "blue") {
				color = Blue;
			}
			else {
				cout << "Invalid color: " << color_str << endl;
			}
			round.push_back(Draw(color, count));
		}
		game.rounds.push_back(round);

	}

	return game;
}

ostream& operator<<(ostream& os, const Color& color)
{
	switch(color) {
	case Red: 
		os << "red";
		break;
	case Green:
		os << "green";
		break;
	case Blue:
		os << "blue";
		break;
	default:
		os << "Invalid color";
		break;
	}
	return os;
}


ostream& operator<<(ostream& os, const Draw& draw)
{
	os << draw.num_cubes << " " << draw.color;
	return os;
}

ostream& operator<<(ostream& os, const Round& round)
{
	bool first = true;
	for (const auto& draw : round) {
		if (first) {
			first = false;
		}
		else {
			os << ", ";
		}
		os << draw;
	}
	return os;
}

ostream& operator<<(ostream& os, const Game& game)
{
	os << "Game " << game.id << ": ";
	bool first = true;
	for (const auto& round : game.rounds) {
		if (first) {
			first = false;
		}
		else {
			os << "; ";
		}
		os << round;
	}
	os << " (Power: " << game.power() << ")";
	return os;
}

void trim(string& s)
{
	s.erase(s.find_last_not_of(" \n\r\t") + 1);
}

int main(int argc, char *argv[])
{
	if (argc < 2) {
		cout << "Need filename" << endl;
		return 2;
	}

	ifstream fd;
	if (!open_file(argv[1], fd)) {
		return 2;
	}

	string inputline;
	list<Game> games;
	while (getline(fd, inputline)) {
		trim(inputline);
		auto game = parse_game(inputline);
		cout << "Game: " << game << endl;
		games.push_back(game);
	}

	int sum_ids = 0;
	for (const auto &game : games) {
		bool valid = true;
		for (const auto& round : game.rounds) {
			for (const auto& draw : round) {
				switch (draw.color) {
				case Red:
					if (draw.num_cubes > 12) {
						valid = false;
					}
					break;
				case Green:
					if (draw.num_cubes > 13) {
						valid = false;
					}
					break;
				case Blue:
					if (draw.num_cubes > 14) {
						valid = false;
					}
					break;
				}
			}
		}
		if (valid) {
			sum_ids += game.id;
		} else {
			cout << "Invalid game: " << game << endl;
		}
	}

	cout << "Sum of valid game IDs: " << sum_ids << endl;

	auto sum_powers = accumulate(games.begin(), games.end(), 0, [](int sum, const Game &next) { return sum + next.power(); });

	cout << "Sum of powers: " << sum_powers << endl;

	return 0;
}
