# // Differences = 0 => val[1] + 3 (3 combos)
# //               1 => val[1] + 6 (2 combos)
# //               2 => val[1]     (1 combo)
# //              -1 => val[1]     (2 combos)
# //              -2 => val[1] + 6 (1 combo)

def day_2():

    total_a = 0
    total_b = 0
    pattern = [1, 2, 3]
    pairs = {0: 3, 1: 6, 2: 0, -1: 0, -2: 6}

    with open("../inputs/day_2.txt", "r") as f:
        lines = f.readlines()

    # with open("testing.txt", "r") as f:
    #     lines = f.readlines()

    for line in lines:
        a, b = ord(line[0]) - 64, ord(line[2]) - 87

        total_a += (b + pairs[b - a])
        total_b += ((pattern[(a - 2) % 3]) if b == 1
                    else (pattern[a % 3] + 6) if b == 3
                    else (a + 3))

    print(f"Total A: {total_a}\nTotal B: {total_b}")


if __name__ == "__main__":
    day_2()
