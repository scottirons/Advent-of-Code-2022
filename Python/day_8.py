def solve():

    with open("../inputs/day_8.txt", "r") as f:
        lines = f.readlines()

    trees = [[] for i in range(len(lines[-1]))]
    visible = [[False for i in range(len(lines[-1]))] for j in range(len(lines[-1]))]

    for i, line in enumerate(lines):
        for j, char in enumerate(line):
            if j < len(lines[-1]):
                trees[i].append(int(char))

    # visible from the left
    for row in range(len(trees)):
        start_height = trees[row][0]
        visible[row][0] = True
        for col in range(1, len(trees[0])):
            if trees[row][col] > start_height:
                visible[row][col] = True
                start_height = trees[row][col]

    # visible from the right
    for row in range(len(trees) - 1, -1, -1):
        start_height = trees[row][-1]
        visible[row][-1] = True
        for col in range(len(trees[0]) - 2, -1, -1):
            if trees[row][col] > start_height:
                visible[row][col] = True
                start_height = trees[row][col]

    # visible from the top
    for col in range(len(trees[0])):
        start_height = trees[0][col]
        visible[0][col] = True
        for row in range(1, len(trees)):
            if trees[row][col] > start_height:
                visible[row][col] = True
                start_height = trees[row][col]

    # visible from the bottom
    for col in range(len(trees[0]) - 1, -1, -1):
        start_height = trees[-1][col]
        visible[-1][col] = True
        for row in range(len(trees) - 2, -1, -1):
            if trees[row][col] > start_height:
                visible[row][col] = True
                start_height = trees[row][col]

    visible_count = 0
    for row in visible:
        for char in row:
            visible_count += 1 if char is True else 0

    max_view_score = 1
    for row in range(1, len(trees) - 1):
        for col in range(1, len(trees[0]) - 1):
            curr_score = 1
            curr_val = trees[row][col]

            # up
            i = row - 1
            score_up = 0
            while i >= 0:
                if trees[i][col] < curr_val:
                    score_up += 1
                else:
                    score_up += 1
                    break
                i -= 1
            curr_score *= score_up

            # down
            i = row + 1
            score_down = 0
            while i < len(trees):
                if trees[i][col] < curr_val:
                    score_down += 1
                else:
                    score_down += 1
                    break
                i += 1
            curr_score *= score_down

            # right
            i = col + 1
            score_right = 0
            while i < len(trees):
                if trees[row][i] < curr_val:
                    score_right += 1
                else:
                    score_right += 1
                    break
                i += 1
            curr_score *= score_right

            # left
            i = col - 1
            score_left = 0
            while i >= 0:
                if trees[row][i] < curr_val:
                    score_left += 1
                else:
                    score_left += 1
                    break
                i -= 1
            curr_score *= score_left

            max_view_score = max(curr_score, max_view_score)

    print(f"Part A: {visible_count}")
    print(f"Part B: {max_view_score}")


if __name__ == "__main__":
    solve()
