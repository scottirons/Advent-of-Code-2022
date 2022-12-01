def solve():
    with open("../inputs/day_1.txt", "r") as infile:
        lines = infile.readlines()

    third, second, first = 0, 0, 0
    running_sum = 0
    for line in lines:
        if line == "\n":
            if running_sum >= first:
                third = second
                second = first
                first = running_sum
            elif running_sum >= second:
                third = second
                second = running_sum
            elif running_sum >= third:
                third = running_sum
            running_sum = 0
        else:
            running_sum += int(line[:-1])

    print(f'Greatest: {first} \nSum of three greatest: {first + second + third}\n')


if __name__ == "__main__":
    solve()
