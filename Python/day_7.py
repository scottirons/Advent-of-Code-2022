class Dir:
    def __init__(self, name=None, prev=None):
        self.total = 0
        self.prev = prev
        self.sub_dirs = {}
        self.name = name

    def get_total(self):
        return self.total

    def add_to_total(self, val: int):
        self.total += val

    def add_sub_dir(self, sub_dir_name: str):
        if sub_dir_name not in self.sub_dirs:
            self.sub_dirs[sub_dir_name] = Dir(sub_dir_name, self)

    def get_sub_dirs(self):
        return self.sub_dirs

    def get_prev_dir(self):
        return self.prev

    def get_name(self):
        return self.name


class Structure:
    def __init__(self):
        self.dirs: [Dir] = []

    def add_dir(self, dir_obj):
        self.dirs.append(dir_obj)

    def show_dirs(self):
        return self.dirs


def recursive_add(directory: Dir):
    for _, sub_dir in directory.get_sub_dirs().items():
        add_val = recursive_add(sub_dir)
        directory.add_to_total(add_val)

    return directory.get_total()


def solve(file):
    with open(file, "r") as f:
        lines = f.readlines()

    structure = Structure()

    curr_dir = Dir("/")
    root = curr_dir

    structure.add_dir(curr_dir)

    for line in lines[1:]:
        if line[0] == "$" and line[2:4] == "cd":
            next_dir = line[5:-1]
            if next_dir == "..":
                curr_dir = curr_dir.get_prev_dir()
            else:
                curr_dir = curr_dir.get_sub_dirs()[next_dir]
                structure.add_dir(curr_dir)
        elif line[0].isdigit():
            number = int(line.split(" ")[0])
            curr_dir.add_to_total(number)
        elif line[0:3] == "dir":
            curr_dir.add_sub_dir(line[4:-1])

    # try to recursively go through starting at root??
    recursive_add(root)

    total_a, total_b = 0, 0
    min_del = root.get_total() - 40000000
    curr_min = root.get_total()

    for value in structure.show_dirs():
        if value.get_total() <= 100000:
            total_a += value.get_total()
        if min_del <= value.get_total() <= curr_min:
            curr_min = value.get_total()

    total_b = curr_min

    return total_a, total_b


if __name__ == "__main__":
    # test_ans = solve("testing.txt")
    real_ans = solve("../inputs/day_7.txt")
    print(f"Part A ans: {real_ans}")
