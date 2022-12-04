def solve():
    with open("testing.txt", "r") as f:
        lines = f.readlines()

    total_a = 0
    total_b = 0

    for line in lines:
        line = line.split(",")
        line[0] = line[0].split('-')
        line[1] = line[1][:-1].split('-')
        a, b = int(line[0][0]), int(line[0][1])
        c, d = int(line[1][0]), int(line[1][1])

        if ((b >= d) and (a <= c)) or ((d >= b) and (c <= a)):
            total_a += 1
        if (d >= b >= c) or (c <= a <= d) or (a <= c <= b) or (a <= d <= b):
            total_b += 1

    print(total_a, total_b)


if __name__ == "__main__":
    solve()