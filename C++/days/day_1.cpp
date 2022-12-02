//
// Created by Scott Irons on 12/1/22.
//

#include "day_1.h"
#include <fstream>
#include <iostream>
#include <sstream>
#include <numeric>

using namespace std;

void day_1() {
    ifstream my_file;
    string curr_line;
    int curr;
    int top_3[3] = {0, 0, 0};
    int running_total = 0;

    my_file.open("../../inputs/day_1.txt");

    if (my_file.is_open()) {
        while (getline(my_file, curr_line)) {
            if (curr_line.empty())
            {
                if (running_total >= top_3[2])
                {
                    top_3[0] = top_3[1];
                    top_3[1] = top_3[2];
                    top_3[2] = running_total;
                }
                else if (running_total >= top_3[1])
                {
                    top_3[0] = top_3[1];
                    top_3[1] = running_total;
                }
                else if (running_total >= top_3[0])
                {
                    top_3[0] = running_total;
                }
                running_total = 0;
            }
            else
            {
                istringstream tmp(curr_line);
                tmp >> curr;
                running_total += curr;
            }

        }
        if (running_total >= top_3[2])
        {
            top_3[0] = top_3[1];
            top_3[1] = top_3[2];
            top_3[2] = running_total;
        }
        else if (running_total >= top_3[1])
        {
            top_3[0] = top_3[1];
            top_3[1] = running_total;
        }
        else if (running_total >= top_3[0])
        {
            top_3[0] = running_total;
        }
    }
    int total = accumulate(begin(top_3), end(top_3), 0, plus<>());
    cout << "The biggest number is: " << top_3[2] << '\n' << "The sum of the largest 3 is: " << total;

    my_file.close();
}

