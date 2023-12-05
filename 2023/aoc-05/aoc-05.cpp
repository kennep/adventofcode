#include <string>
#include <iostream>
#include <algorithm>
#include <set>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"
#include "ranges.h"

using namespace std;

struct Mapping {
	long long dest_range_start;
	long long source_range_start;
	long long range_length;

	Mapping(const vector<long long> &mapping) {
		dest_range_start = mapping[0];
		source_range_start = mapping[1];
		range_length = mapping[2];
	}

	long long map(long long input) const
	{
		if (input >= source_range_start && input < source_range_start + range_length) {
			return (input - source_range_start) + dest_range_start;
		}
		return input;
	}

	bool operator<(const Mapping& other) const
	{
		return source_range_start < other.source_range_start;
	}

	Range<long long> source_range() const
	{
		return Range(source_range_start, range_length);
	}
	Range<long long> dest_range() const
	{
		return Range(dest_range_start, range_length);
	}

	Range<long long> map(const Range<long long> &input) const
	{
		auto sr = source_range();
		if (sr.intersects(input)) {
			cout << "ERROR: Intersecting range. This range: " << sr << " Other range: " << input << endl;
			return input;
		}

		if (sr.contains(input)) {
			return Range(input.start - source_range_start + dest_range_start, input.length);
		}
		return input;
	}
};


ostream& operator<<(ostream& os, const Mapping& mapping)
{
	os << mapping.dest_range_start << " " << mapping.source_range_start << " " << mapping.range_length;
	return os;
}

class AoC05 : public Case
{
private:
	vector<long long> seeds;
	vector<Range<long long>> seed_ranges;
	vector<vector<Mapping>> mappings;

	vector<Mapping> current_mapping;

	virtual void process_line(const std::string& inputline)
	{
		if (seeds.empty()) {
			auto sl = split(inputline, ":", 2);
			parse_list(sl[1], seeds);
			return;
		}

		if (inputline.empty()) return;

		if (inputline.find(':') != string::npos) {
			if (!current_mapping.empty()) {
				mappings.push_back(current_mapping);
			}
			current_mapping.clear();
			return;
		}

		vector<long long> line;
		parse_list(inputline, line);
		current_mapping.push_back(Mapping(line));
		sort(current_mapping.begin(), current_mapping.end());
	}
	virtual void solve()
	{
		if (!current_mapping.empty()) {
			mappings.push_back(current_mapping);
		}
		cout << seeds << endl;
		cout << mappings << endl;
		vector<long long> result;
		for (const auto& seed : seeds) {
			auto mapped_seed = map(seed);
			cout << seed << " maps to " << mapped_seed << endl;
			result.push_back(mapped_seed);
		}
		sort(result.begin(), result.end());
		cout << "Lowest location: " << result[0] << endl;

		for (auto i = 0; i < seeds.size(); i+=2) {
			seed_ranges.emplace_back(seeds[i], seeds[i + 1]);
		}

		cout << "Seed ranges: " << seed_ranges << endl;

		vector<Range<long long>> result_ranges;
		for (const auto& seed_range : seed_ranges) {
			vector<Range<long long>> r = { seed_range };
			for (const auto& mappingset : mappings) {
				r = map(mappingset, { r });
				//cout << "Step debug: " << r << endl;
			}
			for (const auto& rr : r) {
				result_ranges.push_back(rr);
			}
		}
		sort(result_ranges.begin(), result_ranges.end());
		cout << "Result ranges: " << result_ranges << endl;
		cout << "Lowest location range: " << result_ranges[0] << endl;
	}

	long long map(long long seed)
	{
		for (const auto& mappingset : mappings) {
			for (const auto& mapping : mappingset) {
				long long mapped_seed = mapping.map(seed);
				if (mapped_seed != seed) {
					seed = mapped_seed;
					break;
				}
			}
		}
		return seed;
	}

	vector<Range<long long>> map(const vector<Mapping> &mapping_set, const vector<Range<long long>> &seed_ranges)
	{
		vector<Range<long long>> split_ranges = seed_ranges;
		for (const auto& mapping : mapping_set) {
			auto sr = mapping.source_range();
			vector<Range<long long>> tmp;
			for (const auto& range : split_ranges) {
				auto split = split_range(range, sr);
				for (const auto& r : split) {
					tmp.push_back(r);
				}
			}
			split_ranges = tmp;
		}

		vector<Range<long long>> result;
		for (const auto& range : split_ranges) {
			bool mapped = false;
			for (const auto& mapping : mapping_set) {
				auto r = mapping.map(range);
				if (r.start != range.start) {
					mapped = true;
					result.push_back(r);
					break;
				}
			}
			if (!mapped) {
				result.push_back(range);
			}
		}
		return result;
	}

};


int main(int argc, char *argv[])
{
	return AoC05().run(argc, argv);
}
