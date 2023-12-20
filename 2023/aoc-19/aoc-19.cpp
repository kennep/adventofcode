#include <string>
#include <iostream>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <list>
#include <regex>
#include <map>

#include "case.h"
#include "strings.h"
#include "streamhelpers.h"

using namespace std;

enum Op {
	Goto = 'G',
	Lt = '<',
	Gt = '>'
};

struct Rule {
	char attribute;
	Op op;
	int argument;
	string target;
};

struct Workflow {
	vector<Rule> rules;
};

struct Part {
	map<char, int> ratings;
};

ostream& operator<<(ostream& os, const Op& e) {
	os << (char)e;
	return os;
}

ostream& operator<<(ostream& os, const Rule& r) {
	if (r.op == Goto) {
		os << r.target;
	}
	else {
		os << r.attribute << r.op << r.argument << ":" << r.target;
	}
	return os;
}

ostream& operator<<(ostream& os, const Workflow& w) {
	os << w.rules;
	return os;
}

ostream& operator<<(ostream& os, const Part& p) {
	os << "{";
	bool first = true;
	for (auto c : string("xmas")) {
		auto it = p.ratings.find(c);
		if (it != p.ratings.end()) {
			if (first) first = false;
			else os << ", ";
			os << (*it).first << "=" << (*it).second;
		}
	}
	os << "}";
	return os;
}

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

struct Limit
{
	int min_val;
	int max_val;

	Limit(int min, int max) { min_val = min; max_val = max; }
	Limit() : Limit(0, 0) {}

	void apply_min(int new_min) {
		min_val = max(new_min, min_val);
	}

	void apply_max(int new_max) {
		max_val = min(new_max, max_val);
	}

};

ostream& operator<<(ostream& os, const Limit& l) {
	os << "[" << l.min_val << "->" << l.max_val << "]";
	return os;
}


struct Constraint {
	map<char, Limit> limits;

	Constraint() {
		for (auto c : string("xmas")) {
			limits[c] = Limit(1, 4000);
		}
	}

	unsigned long long combinations() const {
		unsigned long long c = 1;
		for (const auto& l : limits) {
			if (l.second.max_val > l.second.min_val) {
				c *= ((l.second.max_val + 1) - l.second.min_val);
			}
			else {
				c = 0;
			}
		}
		return c;
	}

	void apply(Rule rule) {
		if (rule.op != Lt && rule.op != Gt) {
			cout << "Cannot apply rule: " << rule << endl;
			return;
		}
		auto& cur = *limits.find(rule.attribute);
		if (rule.op == Lt) {
			// value must be at most one less than the rule argument
			cur.second.apply_max(rule.argument - 1);
		}
		else {
			// value must be at least one more than the rule argument
			cur.second.apply_min(rule.argument + 1);
		}
	}

	void apply_neg(Rule rule) {
		if (rule.op != Lt && rule.op != Gt) {
			cout << "Cannot apply negation of rule: " << rule << endl;
			return;
		}
		auto& cur = *limits.find(rule.attribute);
		if (rule.op == Lt) {
			// value must be at least equal to the rule argument
			cur.second.apply_min(rule.argument);
		}
		else {
			// value must be at most equal to the rule argument
			cur.second.apply_max(rule.argument);
		}
	}

};


// px{a<2006:qkq,m>2090:A,rfg}
const regex workflow_regex("([a-z]+)\\{(.*)\\}");

const regex conditional_regex("(x|m|a|s)([><])([0-9]+):(A|R|[a-z]+)");
const regex goto_regex("(A|R|[a-z]+)");

// {x=787,m=2655,a=1222,s=2876}
const regex part_regex("\\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\\}");

class AoC19 : public Case
{
private:
	map<string, Workflow> workflows;
	vector<Part> parts;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		smatch input_match;
		if (regex_match(inputline, input_match, workflow_regex)) {
			auto workflow_name = input_match[1].str();
			auto workflow_rules = split(input_match[2].str(), ",");
			Workflow workflow;
			for (const auto& workflow_rule : workflow_rules) {
				Rule rule;
				if (regex_match(workflow_rule, input_match, conditional_regex)) {
					rule.attribute = input_match[1].str()[0];
					rule.op = (Op)input_match[2].str()[0];
					rule.argument = stoi(input_match[3].str());
					rule.target = input_match[4].str();
				}
				else if (regex_match(workflow_rule, input_match, goto_regex)) {
					rule.op = Goto;
					rule.target = input_match[1].str();
				}
				else {
					cout << "Invalid rule in workflow " << workflow_name << ": " << workflow_rule << endl;
					continue;
				}
				workflow.rules.push_back(rule);
			}
			workflows[workflow_name] = workflow;
		}
		else if (regex_match(inputline, input_match, part_regex)) {
			Part part;
			part.ratings['x'] = stoi(input_match[1].str());
			part.ratings['m'] = stoi(input_match[2].str());
			part.ratings['a'] = stoi(input_match[3].str());
			part.ratings['s'] = stoi(input_match[4].str());
			parts.push_back(part);
		}
		else {
			cout << "Invalid input: " << inputline << endl;
			return;
		}
	}

	virtual void solve()
	{
		int sum_ratings = 0;
		for (const auto& part : parts) {
			if (evaluate_workflow("in", part) == "A") {
				sum_ratings += accumulate(part.ratings.begin(), part.ratings.end(), 0, [](int a, pair<char, int> b) { return a + b.second; });
			}
		}

		cout << "Sum of ratings: " << sum_ratings << endl;

		auto total_combinations = evaluate_combinations("in", Constraint());
		cout << "Total combinations: " << total_combinations << endl;
	}

	string evaluate_workflow(const string &workflow_name, const Part& part)
	{
		if (workflow_name == "A" || workflow_name == "R") return workflow_name;
		const auto &workflow = workflows[workflow_name];

		for (const auto& rule : workflow.rules) {
			if (rule.op == Goto) {
				return evaluate_workflow(rule.target, part);
			}
			auto attr = part.ratings.at(rule.attribute);
			if (rule.op == Gt ? attr > rule.argument : attr < rule.argument)
				return evaluate_workflow(rule.target, part);
		}
		return "!";
	}

	unsigned long long evaluate_combinations(const string& workflow_name, Constraint constraint)
	{
		if (workflow_name == "A") {
			return constraint.combinations();
		}
		if (workflow_name == "R") {
			return 0LL;
		}

		unsigned long long combinations = 0;
		const auto& workflow = workflows[workflow_name];
		for (const auto& rule : workflow.rules) {
			if (rule.op == Goto) {
				combinations += evaluate_combinations(rule.target, constraint);
				continue;
			}
			auto constr_if_true = constraint;
			constr_if_true.apply(rule);
			combinations += evaluate_combinations(rule.target, constr_if_true);

			constraint.apply_neg(rule);
		}
		return combinations;
	}

};

int main(int argc, char *argv[])
{
	return AoC19().run(argc, argv);
}
