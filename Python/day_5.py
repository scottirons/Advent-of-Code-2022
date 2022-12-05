def solve():
    with open("../inputs/day_5.txt", "r") as f:
        lines = f.readlines()

    data_lines = [[] for i in range(10)]
    data_lines_2 = [[] for i in range(10)]
    instructions = []

    for i, line in enumerate(lines):
        # first lines
        if i < 8:
            for j, char in enumerate(line):
                if (j - 1) % 4 == 0 and char != ' ':
                    data_lines[(j - 1) // 4 + 1].append(char)
                    data_lines_2[(j - 1) // 4 + 1].append(char)
        elif i > 9:
            temp = line[:-1].split(" ")
            instructions.append(temp)

    for line in data_lines:
        line.reverse()
    for line in data_lines_2:
        line.reverse()

    # part A
    for move in instructions:
        source, destination = int(move[3]), int(move[5])
        n = int(move[1])

        for i in range(n):
            char = data_lines[source].pop()
            data_lines[destination].append(char)

    # for line in data_lines:
    #     if line:
    #         print(line[-1])

    # part B
    for move in instructions:
        source, destination = int(move[3]), int(move[5])
        n = int(move[1])
        temp = []
        for i in range(n):
            char = data_lines_2[source].pop()
            temp.append(char)
        for i in range(n):
            char = temp.pop()
            data_lines_2[destination].append(char)

    for line in data_lines_2:
        if line:
            print(line[-1])


if __name__ == "__main__":
    solve()
