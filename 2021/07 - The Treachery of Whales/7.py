import numpy as np


def cost_to_move_linearly(crabs, position) -> int:
    return abs(crabs - position).sum()


def cost_to_move_non_linearly(crabs, position) -> int:
    diff = abs(crabs - position)

    # triangular number
    return diff * (diff + 1) / 2


_crabs = np.genfromtxt('input', delimiter=",", dtype=int)

i = min(_crabs)
best = cost_to_move_non_linearly(_crabs, i)

while True:
    new_best = cost_to_move_non_linearly(_crabs, i)

    if new_best > best:
        break

    best = new_best
    i += 1

print(best)
