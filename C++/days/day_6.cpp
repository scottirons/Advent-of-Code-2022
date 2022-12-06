//
// Created by Scott Irons on 12/6/22.
//

#include "day_6.h"
#include <fstream>
#include <iostream>

using namespace std;

void day_6() {

    ifstream file;
    string line;

    file.open("../../inputs/day_6.txt");

    cout << file.is_open() << endl;

    int nums[26] = {-1};
    int curr_i;
    int count = 0;
    int part_a;
    static int PART_A_LEN = 4;
    static int PART_B_LEN = 14;
    while (getline(file, line)) {
        for (int i = 0; i < line.length(); i++) {
            curr_i = (int) line[i] - 97;
            if (nums[curr_i] == -1 || nums[curr_i] < i - count) {
                nums[curr_i] = i; count++;
                if (count == PART_B_LEN) {
                    part_a = i; break;
                }
            }
            count = i - nums[curr_i];
            nums[curr_i] = i;

        }
    } cout << "Part A answer: " << part_a+1 << endl;
}
