//
// Created by Scott Irons on 12/5/22.
//

#include "day_4.h"
#include <fstream>
#include <iostream>

using namespace std;

void day_4() {
    ifstream my_file;
    string curr_line;
    int total_a = 0;
    int total_b = 0;
    int full_len;
    int counter = 0;
    int nums[4] = {0, 0, 0, 0};

    my_file.open("../../inputs/day_4.txt");

    if (my_file.is_open()) {
        while (getline(my_file, curr_line)) {
            full_len = (int)curr_line.length();

            for (int i = 0; i <= full_len; i++) {
                if (curr_line[i] == '-' | curr_line[i] == ',') {
                    counter++;
                } else if (i == full_len) {
                    counter = 0;
                    if ((nums[0] <= nums[2]) & (nums[1] >= nums[3]) | ((nums[2] <= nums[0]) & (nums[3] >= nums[1]))) {
                        total_a++;
                    }
                    if (((nums[3] >= nums[1]) & (nums[1] >= nums[2])) | ((nums[3] >= nums[0]) & (nums[0] >= nums[2])) |
                        ((nums[1] >= nums[2]) & (nums[2] >= nums[0])) | ((nums[1] >= nums[3]) & (nums[3] >= nums[0]))) {
                        total_b++;
                    }
                    for (int &j: nums) {
                        j = 0;
                    }
                } else {
                    nums[counter] *= 10;
                    nums[counter] += (int)curr_line[i];
                }
            }

        }
    }
    my_file.close();
    cout << "Part A total: " << total_a << '\n' << "Part B total: " << total_b;
}
