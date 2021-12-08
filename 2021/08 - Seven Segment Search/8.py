import re
import numpy as np


def str_to_arr(str):
    ret = np.array([0 for _ in range(7)])
    for char in str:
        ret[ord(char) - 97] = 1

    return ret


def build_map(line):
    all_vals = re.findall(r"([a-z]+)", line)
    arrays = {}

    str_to_number = {}
    number_to_str = {}
    possible_values = {}

    for val in all_vals:
        arr = str_to_arr(val)
        key = "".join([str(i) for i in arr])
        arrays[key] = arr

        if len(val) == 2:
            number_to_str[1] = arr
            str_to_number[key] = 1
        elif len(val) == 3:
            number_to_str[7] = arr
            str_to_number[key] = 7
        elif len(val) == 4:
            number_to_str[4] = arr
            str_to_number[key] = 4
        elif len(val) == 7:
            number_to_str[8] = arr
            str_to_number[key] = 8
        else:
            possible_values[key] = [2, 3, 5, 6, 9]

    for key in possible_values:
        line_arr = arrays[key]
        # find 2
        if (line_arr * number_to_str[4]).sum() == 2:
            str_to_number[key] = 2
        # either 0, 6 or 9
        elif (line_arr * number_to_str[8]).sum() == 6:
            # then 6
            if (line_arr * number_to_str[7]).sum() == 2:
                str_to_number[key] = 6
            # then 0
            elif (line_arr * number_to_str[4]).sum() == 3:
                str_to_number[key] = 0
            # else 9
            else:
                str_to_number[key] = 9
        # then 3
        elif (line_arr * number_to_str[1]).sum() == 2:
            str_to_number[key] = 3
        # finally 5
        else:
            str_to_number[key] = 5

    return str_to_number

total = 0
with open('input') as f:
    for l in f:
        pipe = l.find("|")
        _all_vals = re.findall(r"([a-z]+)", l)

        map = build_map(l)

        l = l[pipe + 2:].strip()
        _output_vals = re.findall(r"([a-z]+)", l)

        pow = len(_output_vals) - 1
        line_num = 0
        for val in _output_vals:
            arr = str_to_arr(val)
            key = "".join([str(i) for i in arr])
            line_num += map[key] * (10 ** pow)
            pow -= 1

        total += line_num
print(total)
