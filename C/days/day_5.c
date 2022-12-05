//
// Created by Scott Irons on 12/5/22.
//

#include "day_5.h"
#include <stdio.h>
#include <string.h>

int normalize(int n) {
    switch (n) {
        case 1:
            return 0;
        case 3:
            return 1;
        case 5:
            return 2;
        default:
            return n;
    }
}

void day_5() {

    FILE* infile;
    char buffer[40];
    char temp[60];          // temporary stack for part b transfering
    char temp_char;         // after pushing and popping
    char stacc_a[10][60];
    char stacc_b[10][60];
    int height_a[10] = {0};
    int height_b[10] = {0};
    int row_index = 0;
    int moves[3] = {0, 0, 0};
    int curr_i = 0;
    int dir_index;
    int curr_pile;
    int stack_index;

    infile = fopen("../../inputs/day_5.txt", "r");

    while (fgets(buffer, sizeof(buffer), infile) != NULL) {

        if (row_index < 9) {
            for (int i = 0; i < strlen(buffer); i++) {
                if ((i - 1) % 4 == 0) {
                    curr_pile = (i / 4 + 1);
                    stack_index = height_a[curr_pile];
                    // add value to the appropriate stack
                    stacc_a[curr_pile][stack_index] = buffer[i];
                    stacc_b[curr_pile][stack_index] = buffer[i];
                    height_a[curr_pile]++;
                    height_b[curr_pile]++;
                }
            }
        } else if (row_index > 9) {
            for (int i = 0; i < strlen(buffer); i++) {
                if (buffer[i] == ' ') {
                    curr_i++;
                } else if (buffer[i] == '\n') {
                    curr_i = 0;                         // reset the index
                    // printf("%d, %d, %d\n", moves[0], moves[1], moves[2]);

                    for (int j = 0; j < moves[0]; j++) {

                    }

                    // clear moves
                    moves[0] = 0;
                    moves[1] = 0;
                    moves[2] = 0;
                } else if (curr_i % 2 == 1) {           // at a number ;)
                    dir_index = normalize(curr_i);
                    moves[dir_index] *= 10;
                    moves[dir_index] += (buffer[i] - 48);
                }
            }
        }
        row_index++;
    }

//    for (int i = 1; i < 10; i++) {
//        for (int j = 0; j < height_a[i] - 1; j++) {
//            printf("%c", stacc_a[i][j]);
//        } printf("\n");
//    }
}