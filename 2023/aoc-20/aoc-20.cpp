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


/*
ostream& operator<<(ostream& os, const Limit& l) {
	os << "[" << l.min_val << "->" << l.max_val << "]";
	return os;
}*/

const regex input_regex("([%&]?)([a-z]+) -> ([a-z, ]*)");

enum Pulse {
	Low,
	High
};

class Module;

struct Transmission {
	Module *source;
	Module *destination;
	Pulse pulse;

	Transmission(Module* source, Module *destination, const Pulse pulse) {
		this->source = source;
		this->destination = destination;
		this->pulse = pulse;
	}
};


class Module
{
protected:


	Module(const string&name) : name(name)
	{
	}

	vector<Transmission> emit(Pulse pulse)
	{
		vector<Transmission> out;
		emit(pulse, out);
		return out;
	}

	void emit(Pulse pulse, vector<Transmission> &out)
	{
		for (const auto& output : outputs)
		{
			out.emplace_back(this, output, pulse);
		}
	}

public:
	vector<Module*> inputs;
	vector<Module*> outputs;
	const string name;
	virtual void add_input(Module* input) { inputs.push_back(input); }
	virtual void add_output(Module* output) { outputs.push_back(output); }

	virtual void process(vector<Transmission> inputs, vector<Transmission> &outputs) = 0;
	virtual void reset() {}

};

class Broadcaster : public Module
{
public:
	Broadcaster(const string& name) : Module(name) {}


	virtual void process(vector<Transmission> inputs, vector<Transmission>& outputs) 
	{
		if (inputs.empty()) return;
		auto broadcast_val = inputs.front();

		return emit(broadcast_val.pulse, outputs);
	}
};

class Output : public Module
{
public:
	bool low_signalled = false;

	Output(const string&name) : Module(name) {}
	virtual void process(vector<Transmission> inputs, vector<Transmission>& outputs)
	{
		for (const auto& i : inputs) {
			if (i.pulse == Low) low_signalled = true;
		}
	}

	virtual void reset() {
		low_signalled = false;
	}

};

class FlipFlop : public Module
{
private:
	bool isOn = false;
public:
	FlipFlop(const string&name) : Module(name) {}

	virtual void process(vector<Transmission> inputs, vector<Transmission>& outputs)
	{
		for (const auto& val : inputs) {
			if (val.pulse != Low) {
				//cout << "%" << name << ": ignoring high from " << val.source->name << endl;
				continue;
			}

			//cout << "%" << name << ": state was " << (isOn ? " on, swithing off and emitting low" : "off, switching on and emitting high") << endl;
			Pulse pulse = isOn ? Low : High;
			isOn = !isOn;
			emit(pulse, outputs);
		}
	}

	virtual void reset() {
		isOn = false;
	}

};

class Conjunction : public Module
{
private:
	map<Module *, Pulse> state;
public:
	Conjunction(const string&name) : Module(name) {
	}

	virtual void add_input(Module* input) { 
		Module::add_input(input);
		state[input] = Low;
	}

	virtual void process(vector<Transmission> inputs, vector<Transmission>& outputs)
	{
		for (const auto& val : inputs) {
			state[val.source] = val.pulse;
			bool allHigh = true;
			if (!inputs.empty()) {
				//cout << "&" << name << " state ";
				for (const auto& s : state) {
					//cout << s.first->name << ": " << (s.second == High ? "high" : "low") << " ";
					if (s.second != High) {
						allHigh = false;
						break;
					}
				}
				//cout << endl;
				emit(allHigh ? Low : High, outputs);
			}
		}
	}

	virtual void reset() {
		for (auto &i : state) {
			i.second = Low;
		}
	}

};

class AoC20 : public Case
{
private:
	map<string, vector<string>> output_map;
	map<string, Module *> modules;

	virtual void process_line(const std::string& inputline)
	{
		if (inputline.empty()) return;

		smatch input_match;
		if (regex_match(inputline, input_match, input_regex)) {
			auto kind = input_match[1].str();
			auto name = input_match[2].str();
			auto targets = split(input_match[3].str(), ", ");

			cout << kind << " " << name << " -> " << targets << endl;
			if (name == "broadcaster") {
				modules.emplace(name, new Broadcaster(name));
			}
			else if (kind == "%") {
				modules.emplace(name, new FlipFlop(name));
			}
			else if (kind == "&") {
				modules.emplace(name, new Conjunction(name));
			}
			output_map[name] = targets;
		}
	}

	virtual void end_input() {
		for (const auto& mod : output_map) {
			const auto source = modules[mod.first];
			const auto& targets = mod.second;
			for (const auto& t : targets) {
				if (modules.find(t) != modules.end()) {
					const auto dest = modules[t];
					source->add_output(dest);
					dest->add_input(source);
				}
				else {
					auto dest = new Output(t);
					modules[t] = dest;
					source->add_output(dest);
				}
			}
		}
	}

	char type_sigil(Module* m)
	{
		if (dynamic_cast<FlipFlop*>(m)) return '%';
		if (dynamic_cast<Conjunction*>(m)) return '&';
		return '_';
	}

	virtual void solve()
	{
		cout << "digraph {" << endl;
		for (const auto& m : modules) {
			const auto mod = m.second;
			cout << mod->name << "[label=\"" << type_sigil(mod) << mod->name << "\"]" << endl;
			for (const auto& o : mod->outputs) {
				cout << mod->name << " -> " << o->name << endl;
			}

		}
		cout << "}" << endl;

		vector<Transmission> a;
		vector<Transmission> b;
		vector<Transmission>* inputs = &a;
		vector<Transmission>* outputs = &b;
		int high_pulses = 0;
		int low_pulses = 0;
		for (int i = 0; i < 1000; ++i) {
			loop(inputs, outputs, high_pulses, low_pulses, false);
		}
		cout << "High pulses: " << high_pulses << " Low pulses: " << low_pulses << " Result: " << high_pulses * low_pulses << endl;

		for (auto& mod : modules)
		{
			mod.second->reset();
		}

		if (modules.find("rx") != modules.end()) {
			Output *rx = (Output *)modules["rx"];
			auto presses = 0;
			while (!rx->low_signalled) {
				presses++;
				loop(inputs, outputs, high_pulses, low_pulses, false);
			}
			cout << "rx received low pulse after " << presses << " button presses" << endl;
		}

	}

	void loop(vector<Transmission>*& inputs, vector<Transmission>*& outputs, int& high_pulses, int& low_pulses, bool debug)
	{
		inputs->emplace_back(modules["broadcaster"], modules["broadcaster"], Low);
		if(debug) cout << "button -low-> broadcaster" << endl;
		low_pulses++;
		while (!inputs->empty()) {
			outputs->clear();
			for (const auto& m : modules) {
				auto mod = m.second;
				vector<Transmission> inp;
				for (const auto& i : *inputs) {
					if (i.destination == mod) inp.push_back(i);
				}
				mod->process(inp, *outputs);
			}
			vector<Transmission>* tmp = inputs;
			inputs = outputs;
			outputs = tmp;
			for (auto& t : *inputs) {
				if(debug) cout << (t.source->name) << " -" << (t.pulse == High ? "high" : "low") << "-> " << (t.destination->name) << endl;

				if (t.pulse == High)high_pulses++; else low_pulses++;
			}
		}
	}

};

int main(int argc, char *argv[])
{
	return AoC20().run(argc, argv);
}
