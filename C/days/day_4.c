//
// Created by Scott Irons on 12/5/22.
//

#include "day_4.h"
#include <string.h>
#include <stdio.h>

void day_4() {
    FILE* infile;
    char buffer[15];
    int count_a = 0;
    int count_b = 0;
    int temp = 0;
    int nums[4];
    int num_index = 0;

    infile = fopen("../../inputs/day_4.txt", "r");

    while (fgets(buffer, sizeof(buffer), infile) != NULL) {
        int line_len = (int)strlen(buffer);
        for (int i = 0; i < line_len; i++) {
            if (buffer[i] == '-' | buffer[i] == ',') {
                nums[num_index] = temp;
                num_index += 1;
                temp = 0;
            } else if (buffer[i] == '\n') {
                nums[num_index] = temp;
                num_index = 0;
                temp = 0;
                if ((nums[0] <= nums[2]) & (nums[1] >= nums[3]) | ((nums[2] <= nums[0]) & (nums[3] >= nums[1]))) {
                    count_a++;
                }
                if (((nums[3] >= nums[1]) & (nums[1] >= nums[2])) | ((nums[3] >= nums[0]) & (nums[0] >= nums[2])) |
                        ((nums[1] >= nums[2]) & (nums[2] >= nums[0])) | ((nums[1] >= nums[3]) & (nums[3] >= nums[0]))) {
                    count_b++;
                }
            } else {
                temp = temp * 10;
                temp = temp + (int)buffer[i];
            }
        }
    }
    printf("Part A Total: %d\nPart B Total: %d", count_a, count_b);

}