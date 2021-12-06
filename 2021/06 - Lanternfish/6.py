import numpy as np
from typing import Dict


def build_fish_ages(d) -> Dict[int, int]:
    ret = {}
    for age in d:
        if age not in ret:
            ret[age] = 0

        ret[age] += 1

    return ret


def iterate_ages(ages, days) -> Dict[int, int]:
    new_ages = {}
    for age in range(9):
        if age not in ages:
            continue

        count = ages[age]

        if age == 0:
            new_ages[8] = count
            new_ages[6] = count
        else:
            new_age = age - 1

            if new_age not in new_ages:
                new_ages[new_age] = 0

            new_ages[new_age] += count

    if days > 1:
        return iterate_ages(new_ages, days - 1)
    else:
        return new_ages


data = np.genfromtxt("./input", dtype=int, delimiter=",")
_ages = build_fish_ages(data)

_ages_after_80 = iterate_ages(_ages, 80)
print(sum(_ages_after_80.values()))

_ages_after_256 = iterate_ages(_ages, 256)
print(sum(_ages_after_256.values()))
