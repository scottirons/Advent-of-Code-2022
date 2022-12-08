//
// Created by Scott Irons on 12/7/22.
//

#include "day_7.h"
#include <stdio.h>
#include <limits.h>
#include <ctype.h>
#include <stdlib.h>

static int all_dirs[200] = {0}, A = 0, B = INT_MAX;

static int traverse(FILE* file) {

    static int newPos = 0;
    static char buffer[40];
    int pos = newPos++;

    while (fgets(buffer, sizeof(buffer), file)) {
        if (isdigit (buffer[0])) {
            all_dirs[pos] += atoi(buffer);
        } else if (buffer[0] == '$') {
            if (buffer[2] == 'c') {
                if (buffer[5] != '.') {
                    all_dirs[pos] += traverse(file);
                } else {
                    break;
                }
            }
        }
    }
    return all_dirs[pos];
}

void day_7() {

    FILE* infile;

    infile = fopen("../../inputs/day_7.txt", "r");

    int points_left = traverse(infile) - 40000000;
    for (int i = 0; i < 200; i++) {
        int curr = all_dirs[i];
        if (curr <= 100000) {
            A += curr;
        } if ((B >= curr) && (curr >= points_left)) {
            B = curr;
        }
    }
    printf("Part A total: %d\nPart B total: %d", A, B);
}