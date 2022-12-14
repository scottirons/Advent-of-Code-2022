//
// Created by Scott Irons on 12/8/22.
//

#include "day_8.h"
#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

int count_visibility(const vector<vector<int>>& trees, vector<vector<int>>& visible) {

    // from left to right
    for (int row = 0; row < trees.size(); row++) {
        int curr_height = trees[row][0];
        visible[row][0] = 1;
        for (int col = 0; col < trees.size(); col++) {
            if (trees[row][col] > curr_height) {
                visible[row][col] = 1;
                curr_height = trees[row][col];
            }
        }
    }
    // right to left
    for (int row = trees.size() - 1; row >= 0; row--) {
        int curr_height = trees[row][trees.size() - 1];
        visible[row][trees.size() - 1] = 1;
        for (int col = trees.size() - 1; col >= 0; col--) {
            if (trees[row][col] > curr_height) {
                visible[row][col] = 1;
                curr_height = trees[row][col];
            }
        }
    }
    // up to down
    for (int col = 0; col < trees.size(); col++) {
        int curr_height = trees[0][col];
        visible[0][col] = 1;
        for (int row = 0; row < trees.size(); row++) {
            if (trees[row][col] > curr_height) {
                visible[row][col] = 1;
                curr_height = trees[row][col];
            }
        }
    }
    // down to up
    for (int col = trees.size() - 1; col >= 0; col--) {
        int curr_height = trees[trees.size() - 1][col];
        visible[trees.size() - 1][col] = 1;
        for (int row = trees.size() - 1; row >= 0; row--) {
            if (trees[row][col] > curr_height) {
                visible[row][col] = 1;
                curr_height = trees[row][col];
            }
        }
    }
    int visible_count = 0;
    for (const auto& row: visible) {
        for (auto& tree: row) {
            visible_count += tree;
        }
    }
    return visible_count;
}

int find_greatest_view_score(const vector<vector<int>>& trees) {

    int curr_top = 0;

    for (int row = 1; row < trees.size() - 1; row++) {
        for (int col = 1; col < trees.size() - 1; col++) {

            int start_val = trees[row][col];
            int curr_score = 1;

            // check up
            int i = row - 1;
            int up = 0;
            while (i >= 0) {
                if (trees[i][col] >= start_val) {
                    up++;
                    break;
                } else {
                    up++;
                }
                i--;
            }
            curr_score *= up;

            // check down
            i = row + 1;
            int down = 0;
            while (i < trees.size()) {
                if (trees[i][col] >= start_val) {
                    down++;
                    break;
                } else {
                    down++;
                }
                i++;
            }
            curr_score *= down;

            // check right
            i = col + 1;
            int right = 0;
            while (i < trees.size()) {
                if (trees[row][i] >= start_val) {
                    right++;
                    break;
                } else {
                    right++;
                }
                i++;
            }
            curr_score *= right;

            // check left
            i = col - 1;
            int left = 0;
            while (i >= 0) {
                if (trees[row][i] >= start_val) {
                    left++;
                    break;
                } else {
                    left++;
                }
                i--;
            }
            curr_score *= left;
            curr_top = max(curr_score, curr_top);
        }
    }
    return curr_top;
}

void day_8() {
    fstream file;
    string line;

    file.open("../../inputs/day_8.txt");

    vector<vector<int>> trees;

    while (getline(file, line)) {
        vector<int> curr_line;
        for (auto tree: line) {
            if (tree != '\n') {
                curr_line.push_back((int) tree);
            }
        }
        trees.push_back(curr_line);
    }

    int rows = trees.size();
    vector<vector<int>> visible(rows, vector<int>(rows, 0));

    cout << count_visibility(trees, visible) << endl;
    cout << find_greatest_view_score(trees) << endl;
}
