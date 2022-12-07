//
// Created by Scott Irons on 12/7/22.
//

#include "day_7.h"
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

void day_7() {
    ifstream infile;
    string line;

    infile.open("../../inputs/day_7.txt");
    //cout << infile.is_open() << endl;

    // data structures
    vector<string> curr_path;
    unordered_map<string, int> dirs;
    string curr_path_str;

    while ((getline(infile, line))) {

        if (line[0] == '$') {
            if (line.contains("cd")) {
                if (line.contains("..")) {
                    string old_val = curr_path.back();
                    curr_path.pop_back();
                    for (auto _: old_val) {
                        curr_path_str.pop_back();
                    }
                } else {
                    string key = line.substr(5, line.length() - 5);
                    curr_path.push_back(key);
                    curr_path_str += key;
                    dirs[curr_path_str] = 0;
                }
            }
        } else if (line[0] != 'd') {
            int number = stoi(line.substr(0, line.find(' ')));
            string sub_path;
            int path_index = 0;
            while (sub_path.length() < curr_path_str.length()) {
                sub_path += curr_path[path_index];
                dirs[sub_path] += number;
                path_index++;
            }
        }
    }
    int total_sub_100 = 0;
    int amt_left = dirs["/"] - 40000000;
    int curr_min = INT_MAX;
    for (auto const& [key, value]: dirs) {
        if (value <= 100000) {
            total_sub_100 += value;
        }
        if ((value >= amt_left) && (value <= curr_min)) {
            curr_min = value;
        }
    }
    cout << "Part A total: " << total_sub_100 << endl;
    cout << "Part B total: " << curr_min << endl;



}
