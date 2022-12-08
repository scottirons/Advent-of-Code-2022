//
// Created by Scott Irons on 12/8/22.
//

#include "day_8.h"
#include <stdio.h>
#include <string.h>

void day_8() {
    FILE* file;
    char buffer[101];
    char grid[99][99];
    int tall[99][99] = {0};
    int view[99][99] = {0};

    file = fopen("../../inputs/day_8.txt", "r");
    int i = 0;

    while (fgets(buffer, sizeof(buffer), file)) {
        for (int j = 0; j < strlen(buffer); j++) {
            grid[i][j] = buffer[j];
        }
        i++;
    }

    int count = (98 * 4);

    // left to right
    for (int row = 0; row < 99; row++) {
        int curr = (int)grid[row][0];
        tall[row][0] = 1;
        for (int col = 1; col < 99; col++) {
            if ((int)grid[row][col] > curr) {
                count++;
                tall[row][col] = 1;
                curr = (int)grid[row][col];
            }
        }
    }
    // right to left
    for (int row = 0; row < 99; row++) {
        int curr = (int)grid[row][98];
        tall[row][98] = 1;
        for (int col = 97; col >- 0; col--) {
            if ((int)grid[row][col] > curr) {
                count++;
                tall[row][col] = 1;
                curr = (int)grid[row][col];
            }
        }
    }
    // up to down
    for (int col = 0; col < 99; col++) {
        int curr = (int)grid[0][col];
        tall[0][col] = 1;
        for (int row = 1; row < 99; row++) {
            if ((int)grid[row][col] > curr) {
                count++;
                tall[row][col] = 1;
                curr = (int)grid[row][col];
            }
        }
    }
    // down to up
    for (int col = 0; col < 99; col++) {
        int curr = (int)grid[98][col];
        tall[98][col] = 1;
        for (int row = 97; row >= 0; row--) {
            if ((int)grid[row][col] > curr) {
                count++;
                tall[row][col] = 1;
                curr = (int)grid[row][col];
            }
        }
    }
    int tall_count = 0;
    for (int row = 0; row < 99; row++) {
        for (int col = 0; col < 99; col++) {
            //printf("%d", tall[row][col]);
            if (tall[row][col] == 1) {
                tall_count++;
            }
        }
        //printf("\n");
    }

    // part B
    int max_score = 0;
    int score;
    for (int row = 1; row < 98; row++) {
        for (int col = 1; col < 98; col++) {
            int curr_h = (int)grid[row][col];
            score = 1;
            // up
            i = row - 1;
            int count_up = 0;
            while (i >= 0) {
                if (grid[i][col] < curr_h) {
                    count_up++;
                } else {
                    count_up++;
                    break;
                }
                i--;
            } score *= count_up;

            // down
            i = row + 1;
            int count_down = 0;
            while (i < 99) {
                if (grid[i][col] < curr_h) {
                    count_down++;
                } else {
                    count_down++;
                    break;
                }
                i++;
            } score *= count_down;

            // left
            i = col - 1;
            int count_left = 0;
            while (i >= 0) {
                if (grid[row][i] < curr_h) {
                    count_left++;
                } else {
                    count_left++;
                    break;
                }
                i--;
            } score *= count_left;

            // right
            i = col + 1;
            int count_right = 0;
            while (i < 99) {
                if (grid[row][i] < curr_h) {
                    count_right++;
                } else {
                    count_right++;
                    break;
                }
                i++;
            } score *= count_right;
            view[row][col] = score;

            if (score > max_score) {
                max_score = score;
            }
        }
    }

    printf("Part A: %d\n", tall_count);
    printf("Part B: %d", max_score);
}
