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
#include <vector>

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

void trim(string& s)
{
	s.erase(s.find_last_not_of(" \n\r\t") + 1);
}

int sum_part_numbers(const vector<vector<char>>& input)
{
	auto h = input.size();
	auto w = input[0].size();
	auto sum_part_numbers = 0;

	for (int y = 0; y < h; ++y) {
		for (int x = 0; x < w; ++x) {
			auto c = input[y][x];
			if (c >= '0' && c <= '9') {
				auto xe = x + 1;
				while (xe < w && input[y][xe] >= '0' && input[y][xe] <= '9') xe++;

				auto found_symbol = false;
				for (int pny = y - 1; pny <= y + 1; ++pny) {
					for (int pnx = x - 1; pnx < xe + 1; ++pnx) {
						if (pny >= 0 && pny < h && pnx >= 0 && pnx < w) {
							auto sc = input[pny][pnx];
							if (sc != '.' && (sc < '0' || sc > '9')) {
								found_symbol = true;
							}
						}
					}
				}

				if (found_symbol) {
					int pn = 0;
					auto xc = x;
					while (xc < xe) {
						pn *= 10;
						pn += input[y][xc] - '0';
						xc++;
					}
					sum_part_numbers += pn;
				}
				x = xe;
			}
		}
	}
	return sum_part_numbers;
}

struct PartNumber {
	int part_number;
	vector<pair<int, int>> gears;
};

vector<PartNumber> scan_input(const vector<vector<char>>& input)
{
	auto h = input.size();
	auto w = input[0].size();

	vector<PartNumber> parts;

	for (int y = 0; y < h; ++y) {
		for (int x = 0; x < w; ++x) {
			auto c = input[y][x];
			if (c >= '0' && c <= '9') {
				auto xe = x + 1;
				while (xe < w && input[y][xe] >= '0' && input[y][xe] <= '9') xe++;

				int pn = 0;
				auto xc = x;
				while (xc < xe) {
					pn *= 10;
					pn += input[y][xc] - '0';
					xc++;
				}
				PartNumber part;
				part.part_number = pn;

				auto gx = -1;
				auto gy = -1;
				for (int pny = y - 1; pny <= y + 1; ++pny) {
					for (int pnx = x - 1; pnx < xe + 1; ++pnx) {
						if (pny >= 0 && pny < h && pnx >= 0 && pnx < w) {
							auto sc = input[pny][pnx];
							if (sc == '*') {
								part.gears.push_back(pair(pnx, pny));
							}
						}
					}
				}

				parts.push_back(part);
				x = xe;
			}
		}
	}
	return parts;
}

int sum_gear_ratios(const vector<vector<char>>& input)
{
	auto parts = scan_input(input);

	map<pair<int, int>, vector<PartNumber>> gear_map;
	for (const auto& part : parts) {
		for (const auto& gear_coord : part.gears) {
			auto it = gear_map.find(gear_coord);
			if (it == gear_map.end()) {
				auto pn_vec = vector<PartNumber>();
				pn_vec.push_back(part);
				gear_map[gear_coord] = pn_vec;
			}
			else {
				(*it).second.push_back(part);
			}
		}
	}

	int gear_ratio_sum = 0;
	for (const auto& gearpos : gear_map) {
		if (gearpos.second.size() == 2) {
			gear_ratio_sum += gearpos.second[0].part_number * gearpos.second[1].part_number;
		}
	}
	return gear_ratio_sum;
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

	vector<vector<char>> input;
	string inputline;
	while (getline(fd, inputline)) {
		trim(inputline);
		input.push_back(vector<char>(inputline.begin(), inputline.end()));
	}

	for (const auto& row : input)
	{
		for (const auto& col : row) {
			cout << col;
		}
		cout << endl;
	}

	auto sum_pn = sum_part_numbers(input);
	cout << "Sum of part numbers: " << sum_pn << endl;

	auto sum_gr = sum_gear_ratios(input);
	cout << "Sum of gear ratios: " << sum_gr << endl;


	return 0;
}
