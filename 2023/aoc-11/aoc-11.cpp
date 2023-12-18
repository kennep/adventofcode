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

int abs(int n)
{
	return n < 0 ? -n : n;
}

int min(int n1, int n2)
{
	return n1 < n2 ? n1 : n2;
}

int max(int n1, int n2)
{
	return n1 > n2 ? n1 : n2;
}

class AoC11 : public Case
{
private:
	vector<vector<char>> map;
	vector<size_t> x_gaps;
	vector<size_t> y_gaps;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		map.push_back(vector<char>(inputline.begin(), inputline.end()));
	}

	virtual void solve()		
	{
		expand();
		/*
		for (const auto& m : map) {
			cout << string(m.begin(), m.end()) << endl;
		}*/
		vector<Coord> galaxies;
		for (auto y = 0; y < map.size(); ++y) {
			for (auto x = 0; x < map[y].size(); ++x) {
				if (map[y][x] != '#') continue;
				galaxies.push_back(Coord(x, y));
			}
		}

		vector<pair<Coord, Coord>> pairs;
		for (auto const& c1 : galaxies) {
			for (auto const& c2 : galaxies) {
				if (c1 == c2) continue;
				if (c1.y == c2.y && c2.x < c1.x) continue;
				if (c2.y < c1.y) continue;
				pairs.push_back(pair(c1, c2));
			}
		}

		for (auto scale_factor : vector<int>{ 2, 10, 100, 1000000 }) {
			long long sum_distance = 0;
			for (auto const& pair : pairs) {
				sum_distance += distance(pair.first, pair.second, scale_factor);
			}

			cout << "Sum of distance (scale factor " << scale_factor << "): " << sum_distance << endl;
		}

	}

	void expand() {
		bool found_galaxy;

		for (auto y = 0; y < map.size(); ++y) {
			found_galaxy = false;
			for (auto x = 0; x < map[y].size(); ++x) {
				if (map[y][x] == '#') {
					found_galaxy = true;
					break;
				}
			}
			if (!found_galaxy) {
				y_gaps.push_back(y);
				//string dots(map[y].size(), '.');
				//map.insert(map.begin() + y, vector(dots.begin(), dots.end()));
				//y++;
			}
		}

		for (auto x = 0; x < map[0].size(); ++x) {
			found_galaxy = false;
			for (auto y = 0; y < map.size(); ++y) {
				if (map[y][x] == '#') {
					found_galaxy = true;
					break;
				}
			}
			if (!found_galaxy) {
				/*
				for (auto y = 0; y < map.size(); ++y) {
					map[y].insert(map[y].begin() + x, '.');
				}
				x++;
				*/
				x_gaps.push_back(x);
			}
		}
	}

	char get(Coord c) const
	{
		return map[c.y][c.x];
	}

	long long distance(Coord p1, Coord p2, int scale_factor) const
	{
		//auto base_d = abs((int)p2.y - (int)p1.y) + abs((int)p2.x - (int)p1.x);
		
		auto start_x = min(p1.x, p2.x);
		auto end_x = max(p1.x, p2.x);
		auto start_y = min(p1.y, p2.y);
		auto end_y = max(p1.y, p2.y);

		long long x_distance = end_x - start_x;
		for (const auto& x_gap : x_gaps)
		{
			if (x_gap >= start_x && x_gap <= end_x) x_distance += (scale_factor - 1);
		}


		long long y_distance = end_y - start_y;
		for (const auto& y_gap : y_gaps)
		{
			if (y_gap >= start_y && y_gap <= end_y) y_distance += (scale_factor - 1);
		}

		return x_distance + y_distance;
	}
};

int main(int argc, char *argv[])
{
	return AoC11().run(argc, argv);
}
