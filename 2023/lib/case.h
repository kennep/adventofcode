#pragma once
#include <fstream>

class Case {
private:
	bool open_file(const char* filename, std::ifstream& fd);

	virtual void process_line(const std::string& inputline) = 0;
	virtual void solve() = 0;

public:
	int run(int argc, char* argv[]);
};
