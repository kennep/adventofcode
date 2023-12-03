// aoc-01.cpp : Defines the entry point for the application.
//

#include "aoc-01.h"

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

int get_calibration_value(string & input) {
	auto b1 = input.find_first_of("0123456789");
	auto b2 = input.find_last_of("0123456789");
	return (b1 == string::npos ? 0 : input[b1] - '0') * 10 + (b2 == string::npos ? 0 : input[b2] - '0');
}

const map<string, int> digit_mapping = {
	{"1", 1},
	{"2", 2},
	{"3", 3},
	{"4", 4},
	{"5", 5},
	{"6", 6},
	{"7", 7},
	{"8", 8},
	{"9", 9},
	{"0", 0},
	{"one", 1},
	{"two", 2},
	{"three", 3},
	{"four", 4},
	{"five", 5},
	{"six", 6},
	{"seven", 7},
	{"eight", 8},
	{"nine", 9},
};

int get_digit_val(string& input, size_t pos)
{
	for (auto const& [key, val] : digit_mapping) {
		if (input.substr(pos, key.length()) == key) {
			return val;
		}
	}
	return -1;
}

int find_first_digit(string& input)
{
	for (size_t i = 0; i < input.length(); ++i) {
		auto d = get_digit_val(input, i);
		if (d != -1)
		{
			return d;
		}
	}
	return 0;
}

int find_last_digit(string& input)
{
	for (int i = input.length() - 1; i >= 0; --i) {
		auto d = get_digit_val(input, i);
		if (d != -1)
		{
			return d;
		}
	}
	return 0;
}


int get_calibration_value_2(string& input) {
	auto d1 = find_first_digit(input);
	auto d2 = find_last_digit(input);
	return d1 * 10 + d2;
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
	int sum_1 = 0;
	int sum_2 = 0;
	while (getline(fd, inputline)) {
		auto calibration_value_1 = get_calibration_value(inputline);
		sum_1 += calibration_value_1;
		auto calibration_value_2 = get_calibration_value_2(inputline);
		sum_2 += calibration_value_2;
	}

	cout << "Sum of calibration values (1): " << sum_1 << endl;
	cout << "Sum of calibration values (2): " << sum_2 << endl;

	return 0;
}
