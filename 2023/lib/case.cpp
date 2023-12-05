#define _CRT_SECURE_NO_WARNINGS
#include "case.h"
#include "strings.h"

#include <iostream>
#include <string>

using namespace std;

bool Case::open_file(const char* filename, ifstream& fd) 
{
	fd.open(filename);
	if (!fd.good())
	{
		cout << "Unable to open file: " << strerror(errno) << endl;
		return false;
	}
	return true;
}

int Case::run(int argc, char* argv[])
{
	if (argc < 2) {
		cout << "Usage: " << argv[0] << " INPUTFILE" << endl;
		return 2;
	}

	ifstream fd;
	if (!open_file(argv[1], fd)) {
		return 2;
	}

	string inputline;
	while (getline(fd, inputline)) {
		trim(inputline);

		process_line(inputline);
	}

	solve();

	return 0;
}
