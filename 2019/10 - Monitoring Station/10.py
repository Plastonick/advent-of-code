import numpy as np


def yield_intermediate_points(s, t):
    v = (s[0] - t[0], s[1] - t[1])

    if s[0] == t[0] or s[1] == t[1]:
        hcf = max(abs(s[0] - t[0]), abs(s[1] - t[1]))
    else:
        hcf = int((abs(v[0]) * abs(v[1])) / np.lcm(abs(v[0]), abs(v[1])))

    v = (int(v[0] / hcf), int(v[1] / hcf))
    c = (s[0] - v[0], s[1] - v[1])

    while c != t:
        yield c

        c = (c[0] - v[0], c[1] - v[1])


def is_seen(s, t, asteroids) -> bool:
    for intermediate in yield_intermediate_points(s, t):
        if intermediate == s or intermediate == t:
            continue

        if intermediate in asteroids:
            return False

    return True


def number_seen(base, asteroids) -> int:
    n = 0

    for potential in asteroids:
        # don't count the base as something it can see!
        if potential == base:
            continue

        if is_seen(base, potential, asteroids):
            n += 1

    return n


with open('input') as file:
    _asteroids = {}
    i = 0
    for line in file:
        for j in range(len(line)):
            if line[j] == '#':
                _asteroids[(i, j)] = True
        i += 1

    size = len(line)

best = None
m = 0
for p in _asteroids:
    num = number_seen(p, _asteroids)
    if num > m:
        m = num
        best = p

print(m)
print(best)
