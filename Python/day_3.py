def solve():
    with open("../inputs/day_3.txt", "r") as f:
        lines = f.readlines()

    # with open("testing.txt", "r") as f:
    #     lines = f.readlines()

    total_a = 0
    total_b = 0
    # part 1
    for line in lines:
        half_set = set()
        for i, char in enumerate(line):
            if i < len(line)//2:
                half_set.add(char)
            elif char in half_set:
                total_a += ord(char) - 96 if ord(char) > 91 else ord(char) - 38
                half_set.remove(char)

    # part 2
    fancy_count_array = [0] * 53

    for i, line in enumerate(lines):
        for char in line.strip():
            num = ord(char) - 96 if ord(char) > 91 else ord(char) - 38
            if (i + 1) % 3 == 1:
                fancy_count_array[num] = 1
            elif (i + 1) % 3 == 2 and fancy_count_array[num] == 1:
                fancy_count_array[num] = 2
            elif fancy_count_array[num] == 2 and (i + 1) % 3 == 0:
                total_b += num
                fancy_count_array[num] = 0

        if (i + 1) % 3 == 0:
            for j in range(len(fancy_count_array)):
                fancy_count_array[j] = 0

    print(f"total A: {total_a}\nTotalB: {total_b}")


if __name__ == "__main__":
    solve()

