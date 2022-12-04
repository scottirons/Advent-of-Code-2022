//
// Created by Scott Irons on 12/3/22.
//

#include "day_3.h"
#include "day_2.h"
#include <string.h>
#include <stdio.h>

void day_3() {

    FILE* infile;
    char buffer[60];
    int count_a = 0;
    int count_b = 0;
    int half;
    int full_len;
    int val;
    int curr_line = 1;
    static int count_arr_a[53];
    static int count_arr_b[53];

    infile = fopen("../../inputs/day_3.txt", "r");
    //infile = fopen("/Users/scottirons/Desktop/Coding Stuff/Advent_Of_Code_2022/C/days/test.txt", "r");
    // not == 10, which is newline ;>

    while (fgets(buffer, sizeof(buffer), infile) != NULL) {
        full_len = (int)(strlen(buffer) - 1);
        half = full_len / 2;
        for (int i = 0; i < full_len; i++) {

            // convert to correct score per instructions
            val = (int)buffer[i];
            if (val - 91 > 0) {
                val -= 96;
            } else {
                val -= 38;
            }
            // part A stuff
            if (i < half) {
                count_arr_a[val] += 1;
            } else {
                if (count_arr_a[val] > 0) {
                    count_a += val;
                    count_arr_a[val] = 0;
                }
            }
            // part B stuff
            if (curr_line % 3 == 0) {
                count_b += (count_arr_b[val] >> 1) * val;
                count_arr_b[val] = 0;
            } else if (curr_line % 3 == 1) {
                count_arr_b[val] = 1;
            } else {
                count_arr_b[val] = (count_arr_b[val] + 1) & 2;
            }
        }
        if (curr_line % 3 == 0) {
            for (int j = 0; j < (sizeof(count_arr_b) / sizeof(count_arr_b[0])); j++) {
                count_arr_b[j] = 0;
            }
        }
        curr_line++;
        // end of 3 lines, clear arr_b
        for (int i = 0; i < (sizeof(count_arr_a) / sizeof(count_arr_a[0])); i++) {
            count_arr_a[i] = 0;
        }
    }
    fclose(infile);
    printf("Part A value: %d\nPart B value: %d", count_a, count_b);

}