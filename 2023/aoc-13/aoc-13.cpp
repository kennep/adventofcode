#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

typedef vector<char> row;
typedef vector<row> pattern;

ostream& operator<<(ostream& os, const row &r) {
	cout << string(r.begin(), r.end());
	return os;
}

ostream& operator<<(ostream& os, const pattern &p) {
	for (const auto& r : p) {
		cout << r << endl;
	}
	return os;
}

class AoC13 : public Case
{
private:
	vector<pattern> patterns;
	pattern current_pattern;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty())
		{
			push_pattern();
			return;
		}

		current_pattern.push_back(row(inputline.begin(), inputline.end()));
	}

	virtual void end_input()
	{
		push_pattern();
	}

	void push_pattern()
	{
		if (!current_pattern.empty()) {
			patterns.push_back(current_pattern);
			current_pattern.clear();
		}
	}

	virtual void solve()		
	{
		cout << "Part A" << endl;
		size_t sum = 0;
		vector<pair<size_t, size_t>> solutions;
		for (const auto& p : patterns) {
			auto c = find_vertical_mirror(p, 0);
			auto r = find_horizontal_mirror(p, 0);
			cout << "Vertical " << c << " Horizontal " << r << endl;
			sum += c + 100 * r;
			if (c == 0 && r == 0) {
				cout << "WARNING Did not find pattern: " << endl;
				cout << p << endl;

			}
			solutions.emplace_back(c, r);
		}

		cout << "Sum " << sum << endl;


		cout << "Part B" << endl;
		bool found;
		sum = 0;
		size_t i = 0;
		size_t x;
		size_t y;
		for (auto& p : patterns) {
			auto sol = solutions[i];
			found = false;
			for (y = 0; y < p.size(); ++y) {
				for (x = 0; x < p[y].size(); ++x) {
					p[y][x] = p[y][x] == '.' ? '#' : '.';
					auto c = find_vertical_mirror(p, sol.first);
					auto r = find_horizontal_mirror(p, sol.second);
					p[y][x] = p[y][x] == '.' ? '#' : '.';
					if (c == 0 && r == 0) continue; 

					cout << "Smudge at " << x << ", " << y << ": Vertical " << c << " Horizontal " << r << endl;
					sum += c + 100 * r;
					found = true;
					break;
				}
				if (found) break;
			}
			if (!found) {
				cout << "No different reflection lines found for pattern" << endl;
				cout << "Prev solution Vertical " << sol.first << " Horizontal " << sol.second << endl;
				cout << p << endl;
			}
			i++;
		}

		cout << "Sum " << sum << endl;

	}

	vector<char> get_col(const pattern& p, size_t col)
	{
		vector<char> result(p.size());
		for (const auto& r : p) {
			result.push_back(r[col]);
		}
		return result;
	}

	vector<char> get_row(const pattern& p, size_t row) {
		return p[row];
	}

	size_t find_vertical_mirror(const pattern &p, size_t avoid)
	{
		for (auto x = 1; x < p[0].size(); ++x) {
			if (x == avoid) continue;
			bool mirrored = true;
			for (auto dx = 0; x - dx - 1>= 0 && x + dx < p[0].size(); ++dx) {
				auto c1 = get_col(p, x - dx - 1);
				auto c2 = get_col(p, x + dx);
				if (c1 != c2) {
					mirrored = false;
				}
			}
			if (mirrored) {
				return x;
			}
		}
		return 0;
	}

	size_t find_horizontal_mirror(const pattern& p, size_t avoid)
	{
		for (auto y = 1; y < p.size(); ++y) {
			if (y == avoid) continue;
			bool mirrored = true;
			for (auto dy = 0; y - dy - 1 >= 0 && y + dy < p.size(); ++dy) {
				auto r1 = get_row(p, y - dy - 1);
				auto r2 = get_row(p, y + dy);
				if (r1 != r2) {
					mirrored = false;
				}
			}
			if (mirrored) {
				return y;
			}
		}
		return 0;
	}

};

int main(int argc, char *argv[])
{
	return AoC13().run(argc, argv);
}
