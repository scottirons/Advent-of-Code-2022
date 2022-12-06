import string

def solve():

    with open("../inputs/day_6.txt", "r") as f:
        line = f.readline().strip()

    prev = {char: -1 for char in string.ascii_lowercase}
    count = 0
    for i, char in enumerate(line):
        if prev[char] == -1 or prev[char] < i - count:
            prev[char] = i
            count += 1
            if count == 14:
                print(f"Your result is: {i + 1}")
                break
        else:
            count = i - prev[char]
            prev[char] = i


if __name__ == "__main__":
    solve()
