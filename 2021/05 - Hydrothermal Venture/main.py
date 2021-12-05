import re


def get_horz_vert_lines(file):
    ret = []
    for line in file:
        points = [int(n) for n in re.findall(r"(\d+)", line)]

        # only horizontal and vertical for part 1
        if points[0] != points[2] and points[1] != points[3]:
            continue

        ret.append(points)

    return ret


def yield_range(start: (int, int), end: (int, int)):
    magnitude = max(abs(end[0] - start[0]), abs(end[1] - start[1]))
    vector = (int((end[0] - start[0]) / magnitude), int((end[1] - start[1]) / magnitude))
    curr = start
    while curr != end:
        yield curr

        curr = (curr[0] + vector[0], curr[1] + vector[1])

    # we also want the end point
    yield curr


def build_vent_position_freq(l):
    ret = {}
    for points in l:
        yield_range((points[0], points[1]), (points[2], points[3]))

        start = (points[0], points[1])
        end = (points[2], points[3])

        for point in yield_range(start, end):
            if point not in ret:
                ret[point] = 0

            ret[point] += 1

    return ret


def count_dupes(freq):
    total = 0
    for key in freq:
        if freq[key] > 1:
            total += 1

    return total


with open("./input") as f:
    lines = get_horz_vert_lines(f)
    freq = build_vent_position_freq(lines)

    print(count_dupes(freq))
