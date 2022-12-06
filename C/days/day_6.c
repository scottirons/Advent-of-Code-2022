//
// Created by Scott Irons on 12/6/22.
//

#include <stdio.h>
#include "day_6.h"
#include <string.h>

void day_6() {
    FILE* infile;
    char buffer[10000];

    infile = fopen("../../inputs/day_6.txt", "r");

    int count = 0;
    int curr;
    int nums[26];
    for (int i = 0; i < sizeof(nums); i++){
        nums[i] = -1;
    }

    while (fgets(buffer, sizeof(buffer), infile) != NULL) {
        for (int i = 0; i < strlen(buffer); i++) {
            curr = (int)buffer[i] - 97;
            if ((nums[curr] == -1) | (nums[curr] < (i - count))) {
                count++;
                nums[curr] = i;
                if (count == 14) {
                    printf("Solution found: %d", i + 1);
                    break;
                }
            } else {
                count = i - nums[curr];
                nums[curr] = i;
            }

        }
    }

}