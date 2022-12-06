//
// Created by Scott Irons on 12/5/22.
//

#include "day_5.h"
#include <stdio.h>
#include <string.h>


// when parsing the move command inputs, keep track of the numbers in the correct order (move amount, source, destination)
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

    //nobody knows what these variables mean ;')
    //input file stuff
    FILE* infile;
    char buffer[40];
    int row_index = 0;      // which row in the text input. I know the first 8 are the piles, 9 is numbers, below 9 is
                            // move commandz

    // temp stack for transferring values in part b. 60 is max height it could be
    char temp[60];
    int temp_height = 0;

    // temp characters for the transfer step
    char temp_char_a;
    char temp_char_b;

    // stacks and their respective heights. each one has 10 indices so I can keep things indexed as they are in the prob
    // (i.e. 1 is the first pile of stuff)
    char stacc_a[10][60];
    char stacc_b[10][60];
    int height_a[10] = {0};
    int height_b[10] = {0};

    // at each level of the move commands. moves[] will keep track of the values as I go (might need to do fancy
    // multiplication stuff in the case of double-digit move commands).
    int moves[3] = {0};
    int move_amt;           // how many blocks to move at a particular level
    int source;             // source pile when moving blocks
    int destination;        // destination pile

    // these deal with the "index" of the characters in the move command input lines
    // curr_i keeps track of how many spaces we've come across.
    // For example, in this line: "move 5 from 2 to 2"
    // curr_i = 1 after the space after move. This tells me the next chars I come across (before the space) are numbers
    // to be parsed.
    // dir_index is the normalized input to add things to the moves array above (normalized with the handy-dandy
    // normalize function)
    int curr_i = 0;
    int dir_index;

    // curr_pile is used when parsing the first input lines to add characters to the correct pile
    // stack_index is used a few times to push and pop to/from the correct stacks
    int curr_pile;
    int stack_index;

    infile = fopen("../../inputs/day_5.txt", "r");

    while (fgets(buffer, sizeof(buffer), infile) != NULL) {

        if (row_index < 8) {
            for (int i = 0; i < strlen(buffer); i++) {
                if (((i - 1) % 4 == 0) && (buffer[i] != ' ')) {
                    curr_pile = (i / 4 + 1);
                    stack_index = height_a[curr_pile];

                    // add value to the appropriate stack
                    stacc_a[curr_pile][stack_index] = buffer[i];
                    stacc_b[curr_pile][stack_index] = buffer[i];
                    height_a[curr_pile]++;
                    height_b[curr_pile]++;
                }
            }
            // flip order of each pile ;]
        } else if (row_index == 9) {
            for (int j = 1; j < 10; j++) {
                int low = 0;
                int high = height_a[j] - 1;
                while (high > low) {
                    temp_char_a = stacc_a[j][high];
                    stacc_a[j][high] = stacc_a[j][low];
                    stacc_b[j][high] = stacc_b[j][low];
                    stacc_a[j][low] = temp_char_a;
                    stacc_b[j][low] = temp_char_a;
                    high--; low++;
                }
            }
        } else if (row_index > 9) {
            for (int i = 0; i < strlen(buffer); i++) {
                if (buffer[i] == ' ') {
                    curr_i++;
                } else if (buffer[i] == '\n') {
                    curr_i = 0;                         // reset the index
                    move_amt = moves[0];
                    source = moves[1];
                    destination = moves[2];

                    for (int j = 0; j < move_amt; j++) {
                        // get char from A to move over
                        stack_index = height_a[source];
                        temp_char_a = stacc_a[source][stack_index - 1];

                        // move char from B to temp
                        stack_index = height_b[source];
                        temp_char_b = stacc_b[source][stack_index - 1];
                        temp[temp_height] = temp_char_b;
                        temp_height++;

                        // decrement size of a and b
                        height_a[source]--;
                        height_b[source]--;

                        // move character over and increment size of destination
                        stacc_a[destination][height_a[destination]] = temp_char_a;
                        height_a[destination]++;

                    }
                    // after adding all values to the temp, move them to destination_b
                    while (temp_height) {
                        stacc_b[destination][height_b[destination]] = temp[temp_height - 1];
                        temp_height--;
                        height_b[destination]++;
                    }

                    // clear moves
                    moves[0] = 0;
                    moves[1] = 0;
                    moves[2] = 0;
                } else if (curr_i % 2 == 1) {               // at a number ;)
                    dir_index = normalize(curr_i);
                    moves[dir_index] *= 10;                 // in case we have multiple digits
                    moves[dir_index] += (buffer[i] - 48);
                }
            }
        }
        row_index++;
    }

    printf("\nPart A: ");
    for (int i = 1; i < 10; i++) {
        printf("%c", stacc_a[i][height_a[i] - 1]);
    } printf("\n");
    printf("Part B: ");
    for (int i = 1; i < 10; i++) {
        printf("%c", stacc_b[i][height_b[i] - 1]);
    } printf("\n");
}