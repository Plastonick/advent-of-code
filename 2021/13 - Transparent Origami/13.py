def fold(marks: set[tuple[int, int]], x=None, y=None) -> set[tuple[int, int]]:
    to_remove = []
    to_add = []
    if x is not None:
        for point in marks:
            if point[0] < x:
                continue
            else:
                to_remove.append(point)
                to_add.append(((x * 2) - point[0], point[1]))
    elif y is not None:
        for point in marks:
            if point[1] < y:
                continue
            else:
                to_remove.append(point)
                to_add.append((point[0], (y * 2) - point[1]))

    for p in to_remove:
        marks.remove(p)
    for p in to_add:
        marks.add(p)

    return marks


def print_marks(marks: set[tuple[int, int]]):
    max_x = 0
    max_y = 0

    for point in marks:
        max_x = max(max_x, point[0])
        max_y = max(max_y, point[1])

    paper = [
        [" " for _ in range(max_x + 1)]
        for _ in range(max_y + 1)
    ]

    for point in marks:
        paper[point[1]][point[0]] = "#"

    print("\n".join(["".join(line) for line in paper]))


m = set()

with open("input") as f:
    for line in f.read().strip().split("\n"):
        if line == "":
            break

        x, y = line.split(",")
        m.add((int(x), int(y)))

print(len(m))
m = fold(marks=m, x=655)
print("part1", len(m))

m = fold(marks=m, y=447)
m = fold(marks=m, x=327)
m = fold(marks=m, y=223)
m = fold(marks=m, x=163)
m = fold(marks=m, y=111)
m = fold(marks=m, x=81)
m = fold(marks=m, y=55)
m = fold(marks=m, x=40)
m = fold(marks=m, y=27)
m = fold(marks=m, y=13)
m = fold(marks=m, y=6)

print_marks(m)
