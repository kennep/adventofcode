#include "strings.h"

#include <iostream>
#include <sstream>

using namespace std;

void rtrim(string& input)
{
	input.erase(input.find_last_not_of(" \n\r\t") + 1);
}

void ltrim(string& input)
{
	input.erase(0, input.find_first_not_of(" \n\r\t"));
}

void trim(string& input)
{
	ltrim(input);
	rtrim(input);
}

vector<string> split(const string& input, const string &delimiter, size_t max_elems)
{
	size_t pos = 0;

	vector<string> result;

	while (true) 
	{
		auto delim = input.find(delimiter, pos);
		if (delim == string::npos) break;
		result.push_back(input.substr(pos, delim - pos));
		pos = delim + delimiter.length();
		if (result.size() >= max_elems - (size_t)1) break;
	}

	result.push_back(input.substr(pos, string::npos));

	return result;
}

void replace(
	std::string& input,
	const std::string& sub_string,
	const std::string& replacement_string)
{
	size_t start_pos = 0;
	size_t found_pos;
	while ((found_pos = input.find(sub_string, start_pos)) != std::string::npos)
	{
		input.replace(found_pos, sub_string.length(), replacement_string);
		start_pos = found_pos + replacement_string.length();
	}
}