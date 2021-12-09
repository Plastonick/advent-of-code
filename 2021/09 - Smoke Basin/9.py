import numpy as np


def build_height_map(file):
    map = []
    for line in file:
        map.append([int(n) for n in line.strip()])

    return map


def is_valid_point(p, heights):
    if p[0] < 0 or p[0] >= len(heights):
        return False

    if p[1] < 0 or p[1] >= len(heights[p[0]]):
        return False

    return True


def point_is_lower(p, t, heights):
    if not is_valid_point(t, heights):
        return True

    return heights[p[0]][p[1]] < heights[t[0]][t[1]]


def get_adjacent_points(p):
    return [
        (p[0] + 1, p[1]),
        (p[0], p[1] + 1),
        (p[0] - 1, p[1]),
        (p[0], p[1] - 1),
    ]


def is_low_point(p, heights):
    for t in get_adjacent_points(p):
        if not point_is_lower(p, t, heights):
            return False

    return True


def find_low_points(heights):
    ret = []
    for i in range(len(heights)):
        for j in range(len(heights[i])):
            p = (i, j)
            if is_low_point(p, heights):
                ret.append(p)

    return ret


def get_next_basin_points(p, heights):
    ret = []

    for t in get_adjacent_points(p):
        if not is_valid_point(t, heights):
            continue

        if heights[t[0]][t[1]] == 9:
            continue

        if point_is_lower(p, t, heights):
            ret.append(t)

    return ret


def build_basin(s, heights):
    basin = [s]

    next_points = get_next_basin_points(s, heights)

    for p in next_points:
        basin += build_basin(p, heights)

    return basin


with open("input") as f:
    height_map = build_height_map(f)

    low_points = find_low_points(height_map)
    low_heights = np.asarray([height_map[p[0]][p[1]] for p in low_points])
    print("part1", low_heights.sum() + len(low_heights))

    basins = []
    for point in low_points:
        basins.append(set(build_basin(point, height_map)))

    basin_sizes = np.asarray([len(b) for b in basins])
    basin_sizes[::-1].sort()

    print("part2", basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
