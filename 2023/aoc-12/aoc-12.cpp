#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

struct Input {
	vector<char> springs;
	vector<int> groups;
};

class AoC12 : public Case
{
private:
	vector<Input> input;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		auto parts = split(inputline, " ", 2);

		Input i;
		i.springs = vector(parts[0].begin(), parts[0].end());
		parse_csv(parts[1], i.groups);
		input.push_back(i);
	}

	virtual void solve()
	{
		long long total_combs = 0;
		for (const auto& i : input) {
			cout << i.springs << " " << " " << i.groups << endl;
			auto unknowns = count_unknowns(i.springs);
			long long try_combs = pow(2, unknowns);
			long long combs = 0;
			for (long long n = 1; n <= try_combs; ++n) {
				auto s = place_springs(i.springs, n);
				auto g = count_groups(s);
				if (g == i.groups) combs++;
			}
			cout << "Combinations: " << combs << endl;
			total_combs += combs;
		}
		cout << "Total combinations: " << total_combs << endl;

		cout << "E2 " << enumerate2(40, 80, vector<char>(), vector<int>());
	}

	long long enumerate(const Input& i)
	{
		auto unknowns = count_unknowns(i.springs);
		auto knowns = count_knowns(i.springs);
		auto vector = i.groups;
		auto sum_of_elems = std::accumulate(vector.begin(), vector.end(),
			decltype(vector)::value_type(0));
		return enumerate2(sum_of_elems - knowns, unknowns, i.springs, i.groups);
	}

	long long enumerate2(int bits, int places, const vector<char> &springs, const vector<int> & groups)
	{
		if (places < bits) {
			return 0;
		}
		if (places == bits) {
			return 1;
		}
		if (bits == 1) {
			/*
			long long c;
			for (int i = 0; i < places; ++i) {
				long long b = 1LL << i;
				if (is_match(springs, groups, b)) c++;
			}
			return c;
			*/
			return places;
		}
		else {
			for (int i = 0; i < places; ++i) {
				return enumerate2(bits - 1, places - 1, springs, groups);
			}
		}
	}

	bool is_match(const vector<char>& springs, const vector<int>& groups, vector<bool> n)
	{
		auto s = place_springs(springs, n);
		auto g = count_groups(s);
		return (g == groups);
	}

	virtual void solve(const Input& input)
	{
		auto& springs = input.springs;

		for (auto i = 0; i < springs.size(); ++i) {

		}
	}

	int count_unknowns(const vector<char> input)
	{
		int unk = 0;
		for (char c : input) {
			if (c == '?') unk++;
		}
		return unk;
	}

	int count_knowns(const vector<char> input)
	{
		int k = 0;
		for (char c : input) {
			if (c == '#') k++;
		}
		return k;
	}

	vector<char> place_springs(const vector<char> input, long long u)
	{
		vector<char> result;
		for (char c : input) {
			if (c == '?') {
				auto v = u & 1;
				result.push_back(v ? '#' : '.');
				u >>= 1;
			}
			else {
				result.push_back(c);
			}
		}
		return result;
	}

	vector<char> place_springs(const vector<char> input, vector<bool> n)
	{
		vector<char> result;
		auto it = n.begin();
		for (char c : input) {
			if (c == '?') {
				auto v = *(it++);
				result.push_back(v ? '#' : '.');
			}
			else {
				result.push_back(c);
			}
		}
		return result;
	}


	vector<int> count_groups(const vector<char> input) {
		vector<int> result;
		for (auto i = 0; i < input.size(); ++i) {
			if (input[i] == '#') {
				size_t d;
				for (d = i; d < input.size() && input[d] == '#'; ++d);
				result.push_back(d - i);
				i = d;
			}
		}
		return result;
	}

};

int main(int argc, char *argv[])
{
	return AoC12().run(argc, argv);
}
