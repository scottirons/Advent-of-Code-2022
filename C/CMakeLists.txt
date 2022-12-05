cmake_minimum_required(VERSION 3.24)
project(C C)

set(CMAKE_C_STANDARD 99)

add_executable(C main.c all_days.h days/day_1.c days/day_1.h days/day_2.c days/day_2.h days/day_3.c days/day_3.h days/day_4.c days/day_4.h days/day_5.c days/day_5.h)
