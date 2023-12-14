#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <list>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

typedef vector<char> row;
typedef vector<row> platform;

ostream& operator<<(ostream& os, const row &r) {
	cout << string(r.begin(), r.end());
	return os;
}

ostream& operator<<(ostream& os, const platform &p) {
	for (const auto& r : p) {
		cout << r << endl;
	}
	return os;
}

class AoC14 : public Case
{
private:
	platform map;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		map.push_back(row(inputline.begin(), inputline.end()));
	}

	virtual void solve()		
	{
		const int INITIAL_ROUNDS = 100; // experimentally determined
		const long long TOTAL_CYCLES = 1000000000;

		tilt(0, -1);

		cout << map << endl;

		cout << "Load: " << calc_load() << endl;

		for (int i = 0; i < INITIAL_ROUNDS; ++i) spin();

		auto mark = map;
		long long cycles = 0;
		do {
			cycles++;
			spin();
		} while (mark != map);
		cout << "Cycle found after " << cycles << " spin cycles" << endl;
		long long remaining_rounds = (TOTAL_CYCLES - INITIAL_ROUNDS) % cycles;
		for (auto i = 0; i < remaining_rounds - 1; ++i) spin();

		cout << "Load: " << calc_load() << endl;
	}

	void tilt(int dx, int dy)
	{
		int stones_moved = 1;
		while (stones_moved > 0) {
			stones_moved = 0;
			for (auto y = 0; y < map.size(); ++y) {
				if (y + dy < 0) continue;
				if (y + dy >= map.size()) continue;
				for (auto x = 0; x < map[y].size(); ++x) {
					if (x + dx < 0) continue;
					if (x + dx >= map[y].size()) continue;
					if (map[y][x] != 'O') continue;

					if (map[y + dy][x + dx] == '.') {
						map[y + dy][x + dx] = 'O';
						map[y][x] = '.';
						stones_moved++;
					}
				}
			}
		}
	}

	void spin()
	{
		tilt(0, -1);
		tilt(-1, 0);
		tilt(0, 1);
		tilt(1, 0);
	}

	size_t calc_load()
	{
		size_t load = 0;
		for (auto y = 0; y < map.size(); ++y) {
			for (auto x = 0; x < map.size(); ++x) {
				if (map[y][x] == 'O') load += map.size() - y;
			}
		}
		return load;
	}


};

int main(int argc, char *argv[])
{
	return AoC14().run(argc, argv);
}
