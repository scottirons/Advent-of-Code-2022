//
// Created by Scott Irons on 12/3/22.
//

#include "day_3.h"
#include <fstream>
#include <iostream>
#include <sstream>
#include <numeric>
#include <unordered_set>

using namespace std;

void day_3() {
    // cout << "testing stuff here " << (2 >> 1) << '\n';
    ifstream my_file;
    string curr_line;
    int total_a = 0;
    int total_b = 0;
    int half;
    int full_len;
    int curr_val;
    unordered_set<int> part_a_set;
    int part_b[53] = {};
    int count = 0;

    my_file.open("../../inputs/day_3.txt");
    // my_file.open("../days/test.txt");

    if (my_file.is_open()) {
        while (getline(my_file, curr_line)) {
            count++;
            full_len = (int)curr_line.length();
            half = full_len / 2;
            for (int i = 0; i < full_len; i++) {
                curr_val = (int)(u_char)curr_line[i];
                if (curr_val - 91 > 0) {
                    curr_val -= 96;
                } else {
                    curr_val -= 38;
                }

                // part a
                if (i < half) {
                    part_a_set.insert(curr_val);
                } else if (part_a_set.contains(curr_val)) {
                    total_a += curr_val;
                    part_a_set.erase(curr_val);
                }

                // part b
                if (count % 3 == 1) {
                    part_b[curr_val] = 1;
                } else if (count % 3 == 2) {
                    part_b[curr_val] = ((1 + part_b[curr_val]) & 2);
                } else {
                    total_b += ((part_b[curr_val] >> 1) * curr_val);
                    part_b[curr_val] = 0;
                }
            }
            if (count % 3 == 0) {
                for (int & j : part_b) {
                    j = 0;
                }
            }
            part_a_set.clear();
        }
        cout << "Part A total: " << total_a << '\n' << "Part B total: " << total_b;
    }
}