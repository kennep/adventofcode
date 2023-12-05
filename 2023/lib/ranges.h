#pragma once
#include <vector>
#include <ostream>

template<typename T>
struct Range {
	T start; // Start coordinate, inclusive
	T length;

	Range(T start, T length)
	{
		this->start = start;
		this->length = length;
	}

	// End coordinate, exclusive
	T end() const {
		return start + length;
	}

	// This range wholly contains other range
	bool contains(const Range<T>& other) {
		return other.start >= start && other.end() <= end();
	}

	// This range partly overlaps with the other range
	bool intersects(const Range<T>& other) {
		return 
			(other.start < end() && other.end() > end()) ||
			(other.start < start && other.end() > start);
	}

	bool operator<(const Range<T>& other) {
		return start < other.start;
	}

};

template <class Ch, class Tr, typename T>
std::basic_ostream<Ch, Tr>& operator << (std::basic_ostream<Ch, Tr>& os, Range<T> const& r) {
	return os << "(" << r.start << "->" << r.end() << " {" << r.length << "})";
}


template<typename T>
std::vector<Range<T>> split_range(const Range<T>& range, const T& split_at)
{
	if (split_at <= range.start || split_at >= range.end()) {
		return { range };
	}

	return {
		Range(range.start, split_at - range.start),
		Range(split_at, range.length - (split_at - range.start))
	};
}

// Split a range so that it is either entirely within or entirely outside the target range
template<typename T>
std::vector<Range<T>> split_range(const Range<T>& range, const Range<T>& target)
{
	if (range.start > target.end()) {
		return { range };
	}
	if (range.end() < target.start) {
		return { range };
	} 
	if (range.start >= target.start && range.end() <= target.end()) {
		return { range };
	}

	if (range.start < target.start && range.end() <= target.end()) {
		return split_range(range, target.start);
	}

	if (range.start >= target.start && range.end() > target.end()) {
		return split_range(range, target.end());
	}

	if (range.start < target.start && range.end() > target.end()) {
		auto s1 = split_range(range, target.start);
		auto s2 = split_range(s1[1], target.end());
		return { s1[0], s2[0], s2[1] };
	}

	return { range };
}

