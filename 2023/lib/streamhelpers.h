#pragma once
#include <ostream>
#include <iostream>

template <class Ch, class Tr, typename X, typename Y>
std::basic_ostream<Ch, Tr>& operator << (std::basic_ostream<Ch, Tr>& os, std::pair <X, Y> const& p) {
	return os << "(" << p.first << ", " << p.second << ")";
}

template <class Ch, class Tr, typename Container>
std::basic_ostream <Ch, Tr>& operator << (std::basic_ostream <Ch, Tr>& os, Container const& container)
{
	bool first = true;
	std::cout << "[";
	for (const auto& e : container)
	{
		if (first)
		{
			first = false;
		}
		else
		{
			std::cout << ", ";
		}
		std::cout << e;
	}
	std::cout << "]";
	return os;
}

