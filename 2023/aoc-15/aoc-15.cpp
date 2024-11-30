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

struct Step {
	string label;
	char op;
	int focal_length;
};

struct Lens {
	string label;
	int focal_length;
};

ostream& operator<<(ostream& os, const Lens& l) {
	cout << "[" << l.label << " " << l.focal_length << "]";
	return os;
}

typedef vector<Lens> Box;

class AoC15 : public Case
{
private:
	vector<string> input;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		input = split(inputline, ",");
	}

	virtual void solve()		
	{
		int sumhash = 0;
		for (const auto& s : input) {
			auto h = hash(s);
			cout << s << ": " << h << endl;
			sumhash += h;
		}
		cout << "Sum of hashes: " << sumhash << endl;

		vector<Box> boxes(256);

		bool found;
		for (const auto& s : input) {
			auto step = parse_op(s);
			auto lblhash = hash(step.label);
			auto& box = boxes[lblhash];
			found = false;
			for (auto i = 0; i < box.size(); ++i) {
				auto& l = box[i];
				if (l.label == step.label) {
					switch (step.op) {
					case '=':
						l.focal_length = step.focal_length;
						break;
					case '-':
						box.erase(box.begin() + i);
						break;
					}
					found = true;
				}
			}
			if (!found && step.op == '=') {
				Lens l;
				l.label = step.label;
				l.focal_length = step.focal_length;
				box.push_back(l);
			}
		}

		int focusing_power = 0;
		for (auto i = 0; i < boxes.size(); ++i) {
			auto& b = boxes[i];
			if (b.size() == 0) continue;
			cout << i << ": " << b << endl;
			for (auto j = 0; j < b.size(); ++j) {
				auto& l = b[j];
				focusing_power += (i + 1) * (j + 1) * l.focal_length;
			}
		}

		cout << "Focusing power: " << focusing_power << endl;

	}

	Step parse_op(const string& input)
	{
		Step s;
		auto f = input.find('=');
		if (f != string::npos) {
			s.label = input.substr(0, f);
			s.op = '=';
			s.focal_length = atoi(input.substr(f + 1).c_str());
			return s;
		}
		s.label = input.substr(0, input.size() - 1);
		s.op = '-';
		s.focal_length = 0;
		return s;
	}

	int hash(const string &input)
	{
		int val = 0;
		for (char c : input) {
			val += c;
			val *= 17;
			val %= 256;
		}
		return val;
	}


};

int main(int argc, char *argv[])
{
	return AoC15().run(argc, argv);
}
