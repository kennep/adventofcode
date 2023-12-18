#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <list>
#include <set>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

struct Beam {
	int x;
	int y;
	int dx;
	int dy;

	bool operator==(const Beam& other) const
	{
		return x == other.x && y == other.y && dx == other.dx && dy == other.dy;
	}

	int h() const
	{
		return 
			y * 100000 + x * 10 + ((dy + 1) * 4) + (dx + 1);
	}
};

typedef vector<char> Row;

ostream& operator<<(ostream& os, const Beam& b) {
	cout << "Beam at " << b.x << ", " << b.y << ", direction " << b.dx << ", " << b.dy;
	return os;
}

ostream& operator<<(ostream& os, const Row& r) {
	cout << string(r.begin(), r.end());
	return os;
}

ostream& operator<<(ostream& os, const vector<Row>& m) {
	for (const auto& r : m) {
		cout << r << endl;
	}
	return os;
}

struct Energized {
	bool energized : 1;
	bool lr : 1;
	bool rl : 1;
	bool ud : 1;
	bool du : 1;

	bool horiz() const { return lr || rl; }
	bool vert() const { return ud || du; }
	bool energize(int dx, int dy)
	{
		if (dx > 0) lr = true;
		if (dx < 0) rl = true;
		if (dy < 0) du = true;
		if (dy > 0) ud = true;
		if (energized) {
			return false;
		}
		energized = true;
		return true;
	}

	Energized() : energized(false), lr(false), rl(false), ud(false), du(false) {}
};

ostream& operator<<(ostream& os, const Energized& e) {
	os << "E[" << e.energized << " >" << e.lr << " <" << e.rl << " /\\" << e.du << " \\/" << e.ud << "]";
	return os;
}


class AoC16 : public Case
{
private:
	vector<Row> map;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		map.push_back(Row(inputline.begin(), inputline.end()));
	}

	virtual void solve()
	{
		Beam b;
		b.x = -1;
		b.y = 0;
		b.dx = 1;
		b.dy = 0;
		int energized_tiles = solve(b);
		cout << "Total energized tiles: " << energized_tiles << endl;

		vector<int> solutions;
		cout << map[0].size() << "x" << map.size() << endl;
		for (int x = 0; x < map[0].size(); ++x)
		{
			b.x = x;
			b.y = -1;
			b.dx = 0;
			b.dy = 1;
			solutions.push_back(solve(b));
			b.x = x;
			b.y = map.size();
			b.dx = 0;
			b.dy = -1;
			solutions.push_back(solve(b));
		}
		for (int y = 0; y < map.size(); ++y)
		{
			b.x = -1;
			b.y = y;
			b.dx = 1;
			b.dy = 0;
			solutions.push_back(solve(b));
			b.x = map[0].size();
			b.y = y;
			b.dx = -1;
			b.dy = 0;
			solutions.push_back(solve(b));
		}
		sort(solutions.begin(), solutions.end());

		cout << "Max energized tiles: " << solutions.back() << endl;
	}

	int solve(const Beam& b)		
	{
		vector<vector<Energized>> energized;
		for (const auto& r : map)
		{
			energized.push_back(vector<Energized>(r.size()));
		}

		vector<Beam> beams;
		beams.push_back(b);

		int energized_tiles = 0;
		int rounds_since_last_change = 0;
		list<Beam> added_beams;
		while (beams.size() > 0 && rounds_since_last_change < 10)
		{
			rounds_since_last_change++;
			for (auto it = beams.begin(); it != beams.end();)
			{
				Beam& cur =(*it);
				Beam orig = cur;
				//cout << cur << endl;
				cur.x += cur.dx;
				cur.y += cur.dy;
				if (cur.x < 0 || cur.y < 0 || cur.x >= map[0].size() || cur.y >= map.size()) {
					it = beams.erase(it);
					continue;
				}
				auto tile = map[cur.y][cur.x];
				auto& et = energized[cur.y][cur.x];
				switch (tile)
				{
				case '.':
					break;
				case '|':
				{
					if (et.horiz()) {
						it = beams.erase(it);
						continue;
					}
					if (cur.dx != 0) {
						cur.dx = 0;
						cur.dy = -1;
						Beam new_beam;
						new_beam.x = cur.x;
						new_beam.y = cur.y;
						new_beam.dx = 0;
						new_beam.dy = 1;
						added_beams.push_back(new_beam);
					}
					break;
				}
				case '-':
				{
					if (et.vert()) {
						it = beams.erase(it);
						continue;
					}
					if (cur.dy != 0) {
						cur.dy = 0;
						cur.dx = -1;
						Beam new_beam;
						new_beam.x = cur.x;
						new_beam.y = cur.y;
						new_beam.dx = 1;
						new_beam.dy = 0;
						added_beams.push_back(new_beam);
					}
					break;
				}
				case '/':
				{
					auto dy = cur.dy;
					auto dx = cur.dx;
					cur.dy = -dx;
					cur.dx = -dy;
					break;
				}
				case '\\':
				{
					auto dy = cur.dy;
					auto dx = cur.dx;
					cur.dy = dx;
					cur.dx = dy;
					break;
				}

				}
				if (!et.energized) {
					et.energize(orig.dx, orig.dy);
					energized_tiles++;
					rounds_since_last_change = 0;
				}
				else {
					if ((orig.dx == 1 && et.lr) ||
						(orig.dx == -1 && et.rl) ||
						(orig.dy == 1 && et.ud) ||
						(orig.dy == -1 && et.du)) {
						it = beams.erase(it);
						continue;
					}
				}

				it++;
			}
			for (const auto& added_beam : added_beams) {
				beams.push_back(added_beam);
			}
			//cout << energized_tiles << " R " << rounds_since_last_change << " B " << beams.size() << endl;
		}
		cout << energized_tiles << endl;
		return energized_tiles;
	}


};

int main(int argc, char *argv[])
{
	return AoC16().run(argc, argv);
}
