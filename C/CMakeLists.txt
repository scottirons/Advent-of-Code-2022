cmake_minimum_required(VERSION 3.24)
project(C C)

set(CMAKE_C_STANDARD 99)

add_executable(C main.c all_days.h days/day_1.c days/day_1.h)
