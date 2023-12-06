#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"
#include "ranges.h"

using namespace std;

class AoC06 : public Case
{
private:
	vector<int> times;
	vector<int> distances;

	long long int long_t;
	long long int long_d;

	virtual void process_line(const std::string& inputline)
	{
		auto split_line = split(inputline, ":", 2);
		auto label = split_line[0];
		auto numbers = split_line[1];

		if (label == "Time") {
			parse_list(numbers, times);

			replace(numbers, " ", "");
			long_t = atoll(numbers.c_str());
		}
		else if (label == "Distance") {
			parse_list(numbers, distances);

			replace(numbers, " ", "");
			long_d = atoll(numbers.c_str());
		}
	}

	virtual void solve()
	{
		cout << times << endl;
		cout << distances << endl;

		int solution_a = 1;
		for (int i = 0; i < times.size(); ++i) {
			auto time = times[i];
			auto distance = distances[i];
			auto win_count = 0;
			for (auto j = 0; j <= time; ++j) {
				auto d = compute_distance(time, j);
				if (d > distance) {
					win_count++;
				}
			}
			cout << "Win count for t = " << time << ", d = " << distance << ": " << win_count << endl;
			cout << "Win count for t = " << time << ", d = " << distance << ": " << compute_win_count(time, distance) << endl;
			solution_a *= win_count;
		}

		cout << "Solution A: " << solution_a << endl;

		cout << long_t << endl;
		cout << long_d << endl;

		cout << "Solution B: " << compute_win_count(long_t, long_d) << endl;

	}

	int compute_distance(int max_time, int hold_time)
	{
		auto velocity = hold_time;
		return (max_time - hold_time) * velocity;
	}

	long long compute_win_count(long long max_time, long long distance)
	{
		auto s1 = (int)(max_time - sqrt(max_time * max_time - 4 * distance)) / 2;
		auto s2 = (int)(max_time + sqrt(max_time * max_time - 4 * distance)) / 2;
		return s2 - s1;
	}
};


int main(int argc, char *argv[])
{
	return AoC06().run(argc, argv);
}
