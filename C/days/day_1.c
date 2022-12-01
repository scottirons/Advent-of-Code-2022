//
// Created by Scott Irons on 12/1/22.
//

#include "day_1.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct {
    long A;
    long B;
    long C;
} tuple;

void day_1() {

    FILE* infile;
    char buffer[10];
    long number;
    char *ptr;
    long running_sum = 0;
    tuple values = {0, 0, 0};

    infile = fopen("../../inputs/day_1.txt", "r");

    while (fgets(buffer, sizeof(buffer), infile) != NULL) {

        // reached a gap
        if (buffer[0] == '\n') {
            if (running_sum > values.C) {
                values.A = values.B;
                values.B = values.C;
                values.C = running_sum;
            } else if (running_sum > values.B && running_sum < values.C) {
                values.A = values.B;
                values.B = running_sum;
            } else if (running_sum > values.A) {
                values.A = running_sum;
            }
            running_sum = 0;
        } else {
            // add to running total
            number = strtol(buffer, &ptr, 10);
            running_sum += number;
        }

    }
    printf("Third: %ld\nSecond: %ld\nFirst: %ld\nSum: %ld", values.A, values.B, values.C, (values.C + values.B + values.A));

}



