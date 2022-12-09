//
// Created by Scott Irons on 12/9/22.
//

#include "day_9.h"
#include <fstream>
#include <iostream>
#include <vector>
#include <unordered_map>
#include <unordered_set>

using namespace std;

void move_da_rope(vector<vector<int>>& rope,
                  tuple<char, int> instruction,
                  unordered_map<int, unordered_set<int>>& visited) {

    int move_num = get<1>(instruction);
    char direction = get<0>(instruction);
    int x;
    int y;
    switch (direction) {
        case 'R':
            x = 1;
            y = 0;
            break;
        case 'L':
            x = -1;
            y = 0;
            break;
        case 'U':
            x = 0;
            y = 1;
            break;
        case 'D':
            x = 0;
            y = -1;
            break;
        default:
            break;
    }

    while (move_num) {
        rope[0][0] += x;
        rope[0][1] += y;

        for (int i = 1; i < rope.size(); i++) {
            if ((abs(rope[i - 1][1] - rope[i][1]) >= 2) &&
                    (abs(rope[i - 1][0] - rope[i][0]) >= 2)) {
                rope[i][1] = (rope[i][1] + rope[i - 1][1]) / 2;
                rope[i][0] = (rope[i][0] + rope[i - 1][0]) / 2;
            } else if (abs(rope[i - 1][1] - rope[i][1]) >= 2) {
                rope[i][1] = (rope[i][1] + rope[i - 1][1]) / 2;
                rope[i][0] = rope[i - 1][0];
            } else if (abs(rope[i - 1][0] - rope[i][0]) >= 2) {
                rope[i][0] = (rope[i][0] + rope[i - 1][0]) / 2;
                rope[i][1] = rope[i - 1][1];
            }
        }
        visited[rope[9][0]].insert(rope[9][1]);

        move_num--;
    }
//        cout << rope[0][0] << ", " << rope[0][1] << endl;
//        cout << rope[9][0] << ", " << rope[9][1] << endl;
//        cout << endl;

}

void day_9() {
    fstream file;
    string buffer;

    unordered_map<int, unordered_set<int>> visited;
    vector<vector<int>> rope;

    //second half: 11 total

    file.open("../../inputs/day_9.txt");
    //file.open("../days/test.txt");
    for (int i = 0; i < 10; i++) {
        vector<int> new_vec = {0, 0};
        rope.push_back(new_vec);
    }

    while (getline(file, buffer)) {
        tuple<char, int> instruction(buffer[0], stoi(buffer.substr(2)));
        move_da_rope(rope, instruction, visited);
    }
    int total_visited = 0;
    for (auto entry: visited) {
        total_visited += (int)get<1>(entry).size();
    }
    cout << total_visited;

}

