#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

struct Coord {
	size_t x;
	size_t y;

	Coord(size_t x, size_t y) {
		this->x = x;
		this->y = y;
	}

	Coord(const Coord& other) {
		this->x = other.x;
		this->y = other.y;
	}

	bool operator==(const Coord& other) const
	{
		return x == other.x && y == other.y;
	}

	Coord up() const {
		return Coord(x, y - 1);
	}

	Coord down() const {
		return Coord(x, y + 1);
	}

	Coord left() const {
		return Coord(x - 1, y);
	}

	Coord right() const {
		return Coord(x + 1, y);
	}

};

ostream& operator<<(ostream& os, const Coord &c) {
	os << "(" << c.x << ", " << c.y << ")";
	return os;
}

class AoC10 : public Case
{
private:
	vector<vector<char>> map;
	vector<vector<char>> clean_map;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		map.push_back(vector<char>(inputline.begin(), inputline.end()));

		string dots(inputline.length(), '.');
		clean_map.push_back(vector<char>(dots.begin(), dots.end()));
	}

	virtual void solve()		
	{
		auto s = find_s();
		cout << "Starting position: " << s << endl;
		size_t steps = 1;
		auto s_exits = find_exits(s);
		auto orig_l = s;
		auto orig_r = s;
		auto pos_l = s_exits.first;
		auto pos_r = s_exits.second;
		mark(s, get_shape(s));
		mark(pos_l);
		mark(pos_r);

		while (pos_l != pos_r) {
			auto l_exits = find_exits(pos_l);
			auto r_exits = find_exits(pos_r);

			auto new_pos_l = l_exits.first != orig_l ? l_exits.first : l_exits.second;
			auto new_pos_r = r_exits.first != orig_r ? r_exits.first : r_exits.second;

			orig_l = pos_l;
			orig_r = pos_r;
			pos_l = new_pos_l;
			pos_r = new_pos_r;

			mark(pos_l);
			mark(pos_r);

			steps++;
		}

		size_t inside_tiles = 0;
		for (auto y = 0; y < clean_map.size(); ++y) {
			bool on_right_side;
			char last_c;
			size_t span_start;
			char pipe_start_c;
			
			on_right_side = false;
			last_c = '.';
			pipe_start_c = '.';
			span_start = 0;

			// F----JIIIIIIF----J
            // L----JIIIIIIL----J
			// L----7IIIIIIL----7

			for (auto x = 0; x < clean_map[y].size(); ++x) {
				auto c = clean_map[y][x];
				switch (c) {
				case '|':
					on_right_side = !on_right_side;
					break;
				case 'J':
					if(pipe_start_c == 'F') on_right_side = !on_right_side;
					break;
				case '7':
					if (pipe_start_c == 'L') on_right_side = !on_right_side;
					break;
				case 'F':
				case 'L':
					pipe_start_c = c;
					break;
				case '.':
					if (on_right_side) {
						mark(Coord(x, y), 'I');
						inside_tiles++;
					}
				}
				

				/*
				if (c == '.' && last_c == 'X' && !on_right_side) {
					span_start = x;
					on_right_side = true;
				}
				if (c == 'X' && last_c == '.' && on_right_side) {
					on_right_side = false;
					for (auto mx = span_start; mx < x; ++mx) {
						mark(Coord(mx, y), 'I');
						inside_tiles++;
					}
				}*/
				last_c = c;
			}
		}

		cout << "Farthest coordinate at " << pos_l << " after " << steps << " steps" << endl;
		for (const auto& m : clean_map) {
			cout << string(m.begin(), m.end()) << endl;
		}
		cout << "Inside tiles: " << inside_tiles << endl;
	}

	char get(Coord c) const
	{
		return map[c.y][c.x];
	}

	void mark(Coord c, char x) {
		clean_map[c.y][c.x] = x;
	}

	void mark(Coord c) {
		clean_map[c.y][c.x] = get(c);
	}

	Coord find_s() const
	{
		for (auto y = 0; y < map.size(); ++y) {
			for (auto x = 0; x < map[y].size(); ++x) {
				if (map[y][x] == 'S') {
					return Coord(x, y);
				}
			}
		}
		return Coord((size_t)-1, (size_t)-1);
	}

	pair<Coord, Coord> find_exits(Coord pos) const {
		auto tile = get(pos);
		switch (tile) {
		case '|':
			return pair(Coord(pos.x, pos.y - 1), Coord(pos.x, pos.y + 1));
		case '-':
			return pair(Coord(pos.x - 1, pos.y), Coord(pos.x + 1, pos.y));
		case 'F':
			return pair(Coord(pos.x, pos.y + 1), Coord(pos.x + 1, pos.y));
		case 'L':
			return pair(Coord(pos.x, pos.y - 1), Coord(pos.x + 1, pos.y));
		case '7':
			return pair(Coord(pos.x - 1, pos.y), Coord(pos.x, pos.y + 1));
		case 'J':
			return pair(Coord(pos.x, pos.y - 1), Coord(pos.x - 1, pos.y));
		case 'S':
			return find_s_exits(pos);
		default:
			cout << "Uknown tile!" << tile << " at " << pos << endl;
		}
	}

	pair<Coord, Coord> find_s_exits(Coord pos) const {
		vector<Coord> found;
		if (pos.y > 0) {
			auto t = get(pos.up());
			if (t == '|' || t == 'F' || t == '7') {
				found.push_back(pos.up());
			}
		}
		if (pos.y < map.size() - 1) {
			auto t = get(pos.down());
			if (t == '|' || t == 'L' || t == 'J') {
				found.push_back(pos.down());
			}
		}
		if (pos.x > 0) {
			auto t = get(pos.left());
			if (t == '-' || t == 'F' || t == 'L') {
				found.push_back(pos.left());
			}
		}
		if (pos.x < map[0].size() - 1) {
			auto t = get(pos.right());
			if (t == '-' || t == '7' || t == 'J') {
				found.push_back(pos.right());
			}
		}

		if (found.size() != 2) {
			cout << "S didn't have exactly two exits! " << found << endl;
		}

		return pair(found[0], found[1]);
	}

	char get_shape(Coord pos) const {
		auto exits = find_s_exits(pos);
		
		if (has_exit(pos.up(), exits) && has_exit(pos.down(), exits))    return '|';
		if (has_exit(pos.left(), exits) && has_exit(pos.right(), exits)) return '-';
		if (has_exit(pos.left(), exits) && has_exit(pos.up(), exits)) return 'J';
		if (has_exit(pos.right(), exits) && has_exit(pos.up(), exits)) return 'L';
		if (has_exit(pos.left(), exits) && has_exit(pos.down(), exits)) return '7';
		if (has_exit(pos.right(), exits) && has_exit(pos.down(), exits)) return 'F';

		return '!';
	}

	bool has_exit(Coord pos, const pair<Coord, Coord>& exits) const {
		return (exits.first == pos || exits.second == pos);
	}

};

int main(int argc, char *argv[])
{
	return AoC10().run(argc, argv);
}
