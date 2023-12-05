#pragma once
#include <string>
#include <vector>
#include <sstream>
#include <iostream>

void rtrim(std::string& input);
void ltrim(std::string& input);
void trim(std::string& input);

std::vector<std::string> split(const std::string& input, const std::string& delimiter, size_t max_elems = (size_t)-1);

// Parse space-separated data from string
template<typename T>
bool parse_list(const std::string& input, std::vector<T>& dest)
{
	auto ss = std::stringstream(input);
	while (!ss.eof()) {
		T value;
		if (ss >> value) {
			dest.push_back(value);
		}
		else { 
 			std::cerr << "Error: element " << (dest.size() + 1) << " in list not an integer " << input << std::endl;
			return false;
		}
	}
	return true;
}