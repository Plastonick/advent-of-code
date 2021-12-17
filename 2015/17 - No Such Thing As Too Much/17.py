import numpy as np


def get_ways(target: int, containers: list[int], current: list[int]):
    combinations = []

    for i in range(len(containers)):
        container = containers[i]
        if container > target:
            # if the containers too big, we can't use it! This is sorted, so all after are bigger, break!
            break

        combination = current.copy()
        combination.append(container)
        if container == target:
            combinations.append(combination)
        else:
            from_container = containers[i + 1:]
            combinations_from_container = get_ways(target - container, from_container, combination)

            combinations = combinations + combinations_from_container

    return combinations


_containers = [
    1,
    6,
    13,
    13,
    14,
    16,
    18,
    18,
    20,
    24,
    30,
    33,
    35,
    35,
    41,
    42,
    44,
    45,
    48,
    50,
]

_combinations = get_ways(150, _containers, [])
_lengths = [len(c) for c in _combinations]

min_length = min(_lengths)
counts = np.bincount(_lengths)

print("part1", len(_combinations))
print("part2", counts[min_length])
