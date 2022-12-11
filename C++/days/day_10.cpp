//
// Created by Scott Irons on 12/10/22.
//

#include "day_10.h"
#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

void day_10() {
    //fstream file("../days/test.txt");
    fstream file("../../inputs/day_10.txt");
    //cout << file.is_open();
    string line;
    int total = 0;
    int reg_val = 1;
    int cycle = 0;
    vector<char> output_b;

    while (getline(file, line)) {
        int number = 0;
        int moves = 1;
        if (line.length() > 4) {
            number = stoi(line.substr(5));
            moves = 2;
        }
        // at 19
        if ((cycle - 19) % 40 == 0) {
            total += (reg_val * (cycle + 1));
        } else if ((moves == 2) && ((cycle - 18) % 40 == 0)) {
            total += (reg_val * (cycle + 2));
        }
        while (moves--) {
            if (abs((cycle % 40) - reg_val) <= 1) {
                output_b.push_back('#');
            } else {
                output_b.push_back(' ');
            }
            cycle++;
        }
        reg_val += number;
    }
    cout << total << endl;
    for (int i = 0; i < output_b.size(); ++i) {
        cout << output_b[i];
        if ((i + 1) % 40 == 0) {
            cout << endl;
        }
    }
}
