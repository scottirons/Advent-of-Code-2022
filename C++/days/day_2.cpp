//
// Created by Scott Irons on 12/2/22.
//

#include "day_2.h"
#include <fstream>
#include <iostream>
#include <sstream>
#include <numeric>
#include <unordered_map>

using namespace std;

void day_2() {
    ifstream my_file;
    string curr_line;
    unordered_map<int, int> map_a;
    unordered_map<int, int> map_b_1;
    unordered_map<int, int> map_b_2;
    int total_a = 0;
    int total_b = 0;
    int top_2[2] = {0, 0};

    // key/values for part a
    map_a[0] = 3;
    map_a[1] = 6;
    map_a[2] = 0;
    map_a[-1] = 0;
    map_a[-2] = 6;

    // values for part b losses
    map_b_1[1] = 3;
    map_b_1[2] = 1;
    map_b_1[3] = 2;

    // values for part b wins
    map_b_2[1] = 2;
    map_b_2[2] = 3;
    map_b_2[3] = 1;

    my_file.open("../../inputs/day_2.txt");

    if (my_file.is_open()) {
        while (getline(my_file, curr_line)) {

            top_2[0] = (int)curr_line[0] - 64;
            top_2[1] = (int)curr_line[2] - 87;

            total_a += map_a[top_2[1] - top_2[0]] + top_2[1];

            if (top_2[1] == 1) {
                total_b += map_b_1[top_2[0]];
            } else if (top_2[1] == 2) {
                total_b += (top_2[0] + 3);
            } else {
                total_b += (map_b_2[top_2[0]] + 6);
            }

        }
    }
    my_file.close();

    cout << "Total for part 1: " << total_a << '\n' << "Total for part 2: " << total_b << '\n';
}
