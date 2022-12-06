//
// Created by Scott Irons on 12/5/22.
//

#include "day_5.h"
#include <iostream>
#include <fstream>
using namespace std;

void day_5(){
    ifstream my_file;
    string curr_line;

    my_file.open("../../inputs/day_5.txt");
    // get first characters
    string stacc[10];
    string stacc_b[10];
    while (getline(my_file, curr_line)) {
        if (curr_line[1] == '1') {
            break;
        }
        for (int i = 1, pos = 1; pos < curr_line.length(); i++, pos += 4) {
            if (isalpha(curr_line[pos])) {
                stacc[i] += curr_line[pos];
                stacc_b[i] += curr_line[pos];
            }
        }
    }
    for (auto &s: stacc) {
        reverse(s.begin(), s.end());
    }
    for (auto &s: stacc_b) {
        reverse(s.begin(), s.end());
    }

    // skip the empty one
    getline(my_file, curr_line);

    while (getline(my_file, curr_line)) {

        int moves, start, end;
        sscanf(curr_line.c_str(), "move %d from %d to %d", &moves, &start, &end);

        std::string temp;
        while (moves) {
            stacc[end] += stacc[start].back();
            stacc[start].pop_back();

            temp += stacc_b[start].back();
            stacc_b[start].pop_back();

            moves--;
        }
        reverse(temp.begin(), temp.end());
        stacc_b[end] += temp;
    }

    std::string result_a;
    std::string result_b;

    for (int i = 1; i < 10; i++) {
        result_a += stacc[i].back();
        result_b += stacc_b[i].back();
    }

    std::cout << "Part A result: " << result_a << '\n' << "Part B result: " << result_b;

}
