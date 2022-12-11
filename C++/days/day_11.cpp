//
// Created by Scott Irons on 12/11/22.
//

#include "day_11.h"
#include <fstream>
#include <iostream>
#include <vector>
#include <deque>

const unsigned long long modulo = 2 * 7 * 13 * 5 * 3 * 19 * 11 * 17;

using namespace std;

struct RetVal {
    unsigned long long val;
    int next_monkey;
};

class Monkey {
public:
    Monkey(int _mod_amt, char _op_a, int _op_b, int _throw_a, int _throw_b) {
        mod_amt = _mod_amt;
        throw_a = _throw_a;
        throw_b = _throw_b;
        op_a = _op_a;
        op_b = _op_b;
    }

    bool has_items() {
        return !items.empty();
    }

    unsigned long long num_items() {
        return items.size();
    }

    void add_item(unsigned long long item) {
        items.push_back(item);
    }

    int get_inspect_count() const {
        return inspect_amt;
    }

    RetVal process_item() {
        inspect_amt++;
        unsigned long item = items.front();
        items.pop_front();
        unsigned long new_val = operation(item) % modulo;
        return RetVal{new_val, (new_val % mod_amt == 0 ? throw_a: throw_b)};
    }
private:
    unsigned long long operation(unsigned long long val) const {
        bool plus = false;
        unsigned long long amt = val;
        if (op_a == '+') {
            plus = true;
        }
        if (op_b != 0) {
            amt = op_b;
        }
        if (plus) {
            return val + amt;
        }
        return (val % modulo) * amt;
    }

    deque<unsigned long long> items;
    int mod_amt;
    char op_a;
    int op_b;
    int throw_a;
    int throw_b;
    int inspect_amt = 0;

};

vector<Monkey> test() {
        vector<Monkey> monkeys;
        // test
        Monkey new_monkey = Monkey(23, '*', 19, 2, 3);
        monkeys.push_back(new_monkey);
        monkeys[0].add_item(79);
        monkeys[0].add_item(98);

        new_monkey = Monkey(19, '+', 6, 2, 0);
        monkeys.push_back(new_monkey);
        monkeys[1].add_item(54);
        monkeys[1].add_item(65);
        monkeys[1].add_item(75);
        monkeys[1].add_item(74);

        new_monkey = Monkey(13, '*', 0, 1, 3);
        monkeys.push_back(new_monkey);
        monkeys[2].add_item(79);
        monkeys[2].add_item(60);
        monkeys[2].add_item(97);

        new_monkey = Monkey(17, '+', 3, 0, 1);
        monkeys.push_back(new_monkey);
        monkeys[3].add_item(74);

        return monkeys;
};

vector<Monkey> real_deal() {

    vector<Monkey> monkeys;

    Monkey new_monkey = Monkey(2, '*', 17, 2, 6);
    monkeys.push_back(new_monkey);
    monkeys[0].add_item(85);
    monkeys[0].add_item(79);
    monkeys[0].add_item(63);
    monkeys[0].add_item(72);

    new_monkey = Monkey(7, '*', 0, 0, 2);
    monkeys.push_back(new_monkey);
    vector<unsigned long long> vals {53, 94, 65, 81, 93, 73, 57, 92};
    for (auto val: vals) {
        monkeys[1].add_item(val);
    }

    new_monkey = Monkey(13, '+', 7, 7, 6);
    monkeys.push_back(new_monkey);
    vals = {62, 63};
    for (auto val: vals) {
        monkeys[2].add_item(val);
    }

    new_monkey = Monkey(5, '+', 4, 4, 5);
    monkeys.push_back(new_monkey);
    vals = {57, 92, 56};
    for (auto val: vals) {
        monkeys[3].add_item(val);
    }

    new_monkey = Monkey(3, '+', 5, 1, 5);
    monkeys.push_back(new_monkey);
    vals = {67};
    for (auto val: vals) {
        monkeys[4].add_item(val);
    }

    new_monkey = Monkey(19, '+', 6, 1, 0);
    monkeys.push_back(new_monkey);
    vals = {85, 56, 66, 72, 57, 99};
    for (auto val: vals) {
        monkeys[5].add_item(val);
    }

    new_monkey = Monkey(11, '*', 13, 3, 7);
    monkeys.push_back(new_monkey);
    vals = {86, 65, 98, 97, 69};
    for (auto val: vals) {
        monkeys[6].add_item(val);
    }

    new_monkey = Monkey(17, '+', 2, 4, 3);
    monkeys.push_back(new_monkey);
    vals = {87, 68, 92, 66, 91, 50, 68};
    for (auto val: vals) {
        monkeys[7].add_item(val);
    }

    return monkeys;
}

void day_11() {

    //vector<Monkey> monkeys = test();
    vector<Monkey> monkeys = real_deal();

    for (int i = 0; i < 10000; i++) {
        for (auto& monkey: monkeys) {
            while (monkey.has_items()) {
                RetVal result = monkey.process_item();
                //cout << result.val << " " << monkey.num_items() << endl;
                monkeys[result.next_monkey].add_item(result.val);
            }
        }
    }
    unsigned long long a = 0;
    unsigned long long b = 0;
    for (auto& monkey: monkeys) {
        cout << monkey.get_inspect_count() << endl;
        if (monkey.get_inspect_count() > b) {
            a = b;
            b = monkey.get_inspect_count();
        } else if (monkey.get_inspect_count() > a) {
            a = monkey.get_inspect_count();
        }
    } cout << a * b;
}