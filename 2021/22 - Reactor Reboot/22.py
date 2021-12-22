def range_intersection(a: tuple[int, int], b: tuple[int, int]) -> tuple[int, int]:
    start = max(a[0], b[0])
    end = min(a[1], b[1])

    if start > end:
        return None
    else:
        return start, end


def cuboid_intersection(a: list[tuple[int, int]], b: list[tuple[int, int]]) -> list[tuple[int, int]]:
    x_intersection = range_intersection(a[0], b[0])
    if x_intersection is None:
        return None

    y_intersection = range_intersection(a[1], b[1])
    if y_intersection is None:
        return None

    z_intersection = range_intersection(a[2], b[2])
    if z_intersection is None:
        return None

    return [x_intersection, y_intersection, z_intersection]


class Cuboid:
    def __init__(self, stack: int, xdim: tuple[int, int], ydim: tuple[int, int], zdim: tuple[int, int]):
        self.stack = stack
        self.xdim = xdim
        self.ydim = ydim
        self.zdim = zdim

    def total(self):
        return self.stack * (self.xdim[1] - self.xdim[0] + 1) * (self.ydim[1] - self.ydim[0] + 1) * (
                self.zdim[1] - self.zdim[0] + 1)


def add_cuboid(cuboid: Cuboid, turn_on, field: list[Cuboid]):
    for i in range(len(field)):
        a = field[i]

        # if there is an intersection between a and cuboid, add the intersect and note it's double (or more!) counted
        inter = cuboid_intersection([cuboid.xdim, cuboid.ydim, cuboid.zdim], [a.xdim, a.ydim, a.zdim])

        if inter is None:
            continue

        intersection_cuboid = Cuboid(-a.stack, inter[0], inter[1], inter[2])
        field.append(intersection_cuboid)

    if turn_on:
        field.append(cuboid)

    return field


def sum_cuboids(field: list[Cuboid]) -> int:
    total = 0
    for cuboid in field:
        total += cuboid.total()

    return total


def count_on_in_ranges(ranges) -> int:
    _, x_r, y_r, z_r = ranges[0]
    field = [Cuboid(1, x_r, y_r, z_r)]

    for turn_on, x_r, y_r, z_r in ranges[1:]:
        add = Cuboid(1, x_r, y_r, z_r)
        field = add_cuboid(add, turn_on, field)

    return sum_cuboids(field)


_ranges = []
with open('input') as f:
    for line in f.read().strip().splitlines():
        toggle, _range = line.strip().split(" ")
        x, y, z = _range.split(",")
        x1, x2 = [int(n) for n in x[2:].split('..')]
        y1, y2 = [int(n) for n in y[2:].split('..')]
        z1, z2 = [int(n) for n in z[2:].split('..')]

        _ranges.append(((True if toggle == 'on' else False), (x1, x2), (y1, y2), (z1, z2)))

print("part1", count_on_in_ranges(ranges=_ranges[:20]))
print("part2", count_on_in_ranges(ranges=_ranges))
