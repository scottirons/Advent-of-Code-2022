//
// Created by Scott Irons on 12/2/22.
//

#include "day_2.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

typedef struct {
    int Opp;
    int Me;
} scores;

void day_2() {

    char buffer[5];
    int total_a = 0;
    int total_b = 0;
    int dif;
    scores curr_score = {0, 0};
    FILE* infile;

    infile = fopen("../../inputs/day_2.txt", "r");

    // read input and convert to nicely usable ints in tuple form
    while (fgets(buffer, sizeof(buffer), infile) != NULL) {
        curr_score.Opp = (int)buffer[0] - 64;
        curr_score.Me = (int)buffer[2] - 87;

        // compute total for part a
        // Differences = 0 => val[1] + 3 (3 combos)
        //               1 => val[1] + 6 (2 combos)
        //               2 => val[1]     (1 combo)
        //              -1 => val[1]     (2 combos)
        //              -2 => val[1] + 6 (1 combo)
        dif = curr_score.Me - curr_score.Opp;
        switch (dif) {
            case 0:
                total_a += (curr_score.Me + 3);
                break;
            case 2:
            case -1:
                total_a += curr_score.Me;
                break;
            default:
                total_a += (curr_score.Me + 6);
                break;
        }
        switch (curr_score.Me) {
            case 1:
                switch (curr_score.Opp) {
                    case 1:
                        total_b += 3;
                        break;
                    case 2:
                        total_b += 1;
                        break;
                    case 3:
                        total_b += 2;
                        break;
                } break;
            case 2:
                total_b += (curr_score.Opp + 3);
                break;
            case 3:
                switch (curr_score.Opp) {
                    case 1:
                        total_b += 8;
                        break;
                    case 2:
                        total_b += 9;
                        break;
                    case 3:
                        total_b += 7;
                        break;
                }
            default:
                total_b = total_b;
                break;
        }
    }
    printf("Total A: %d\nTotal B: %d\n", total_a, total_b);

}
