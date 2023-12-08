#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <map>
#include <regex>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"
#include "ranges.h"

using namespace std;

// LCM algorithm from https://www.geeksforgeeks.org/lcm-of-given-array-elements/
typedef long long int ll;

// Utility function to find
// GCD of 'a' and 'b'
int gcd(int a, int b)
{
	if (b == 0)
		return a;
	return gcd(b, a % b);
}

// Returns LCM of array elements
ll findlcm(const vector<int> &arr)
{
	auto n = arr.size();
	// Initialize result
	ll ans = arr[0];

	// ans contains LCM of arr[0], ..arr[i]
	// after i'th iteration,
	for (int i = 1; i < n; i++)
		ans = (((arr[i] * ans)) /
			(gcd(arr[i], ans)));

	return ans;
}

// AAA = (BBB, CCC)
const std::regex input_regex("([A-Z0-9]{3}) = \\(([A-Z0-9]{3}), ([A-Z0-9]{3})\\)");

class AoC08 : public Case
{
private:
	string instructions;
	map<string, pair<string, string>> map;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		if (instructions.empty()) {
			instructions = inputline;
			return;
		}

		smatch input_match;
		if (!regex_match(inputline, input_match, input_regex)) {
			cout << "Invalid input: " << inputline << endl;
			return;
		}

		map[input_match[1].str()] = pair(input_match[2].str(), input_match[3].str());
	}

	virtual void solve()		
	{
		cout << instructions << endl;
		cout << map << endl;

		if (map.find("ZZZ") != map.end()) {
			int steps = 0;
			string current_node = "AAA";
			size_t ip = 0;
			while (current_node != "ZZZ")
			{
				auto instr = instructions[ip++];
				if (ip >= instructions.length()) ip = 0;

				auto n = map[current_node];
				current_node = instr == 'L' ? n.first : n.second;
				steps++;
			}

			cout << "Total steps: " << steps << endl;
		}

		vector<string> current_nodes;
		vector<int> cycles;
		for (const auto& n : map) {
			if (n.first[n.first.size() - 1] == 'A') {
				current_nodes.push_back(n.first);
			}
		}

		cout << "Starting nodes: " << current_nodes << endl;
		for (auto& current_node : current_nodes) {
			int steps = 0;
			size_t ip = 0;
			while (current_node[current_node.size()-1] != 'Z')
			{
				auto instr = instructions[ip++];
				if (ip >= instructions.length()) ip = 0;

				auto n = map[current_node];
				current_node = instr == 'L' ? n.first : n.second;
				steps++;
			}
			cout << "Cycle: " << steps << endl;
			cycles.push_back(steps);
		}

		cout << "Total number of steps: " << findlcm(cycles) << endl;
	}
};

int main(int argc, char *argv[])
{
	return AoC08().run(argc, argv);
}
