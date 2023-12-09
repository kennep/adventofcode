#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

typedef long long ll;

class AoC09 : public Case
{
private:
	vector<vector<ll>> input;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		vector<ll> readings;
		parse_list(inputline, readings);
		input.push_back(readings);
	}

	virtual void solve()		
	{
		ll sum_ext = 0;
		for (const auto& readings : input) {
			auto n = next(readings);
			cout << n << endl;
			sum_ext += n;
		}
		cout << "Sum of extrapolated values: " << sum_ext << endl;

		sum_ext = 0;
		for (auto readings : input) {
			reverse(readings.begin(), readings.end());
			auto n = next(readings);
			cout << n << endl;
			sum_ext += n;
		}
		cout << "Sum of extrapolated values: " << sum_ext << endl;
	}

	ll next(const vector<ll>& readings)
	{
		vector<vector<ll>> work;
		work.push_back(readings);
		auto cur = readings;
		while (true) {
			auto diff_r = diff(cur);
			work.push_back(diff_r);
			if (all_of(diff_r.begin(), diff_r.end(), [](ll e) { return e == 0; })) {
				break;
			}
			cur = diff_r;
		}

		for (auto i = work.size() - 1; i >= 1; --i) {
			auto n = work[i - 1][work[i - 1].size() - 1] + work[i][work[i].size() - 1];
			work[i - 1].push_back(n);
		}
		return work[0][work[0].size() - 1];
	}

	vector<ll> diff(const vector<ll> input) {
		vector<ll> result;
		for (auto i = 0; i < input.size() - 1; ++i) {
			result.push_back(input[i + 1] - input[i]);
		}
		return result;
	}
};

int main(int argc, char *argv[])
{
	return AoC09().run(argc, argv);
}
