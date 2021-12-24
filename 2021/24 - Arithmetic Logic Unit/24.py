import fileinput
import math
from functools import cache

# by inspection, there are 14 modulo operations, and none of them does anything! The result of the modulo operations
# are not considered before the value is then compared (eq) to both w and 0. Comparing a value to anything and then 0
# necessarily


def yield_next_number() -> int:
    start = 99999999999999
    while start > 0:
        yield start

        start -= 1
        while "0" in str(start):
            start -= 1


vars = {
    "w": 0,
    "x": 1,
    "y": 2,
    "z": 3
}
_operations = []
_grouped_ops = []
_group = None
for line in fileinput.input():
    terms = line.strip().split(" ")
    if len(terms) == 3:
        _op, _left, _right = terms
        _group.append((_op, _left, _right))
        _operations.append((_op, _left, _right))
    else:
        if _group is not None:
            _grouped_ops.append(_group)
        _op, _left = terms
        _group = []
        _operations.append((_op, _left, None))

_grouped_ops.append(_group)


@cache
def process_group(w, x, y, z, group_index):
    memory = [w, x, y, z]
    operations = _grouped_ops[group_index]

    for op, left, right in operations:
        left_address = vars[left]

        if right in 'wxyz':
            right_val = memory[vars[right]]
        else:
            right_val = int(right)

        if op == 'add':
            memory[left_address] = memory[left_address] + right_val
        elif op == 'mul':
            memory[left_address] = memory[left_address] * right_val
        elif op == 'div':
            if right_val == 0:
                return None, None, None

            memory[left_address] = int(memory[left_address] / right_val)
        elif op == 'mod':
            if memory[left_address] < 0 or right_val <= 0:
                return None, None, None

            memory[left_address] = memory[left_address] % right_val
        elif op == 'eql':
            memory[left_address] = 1 if memory[left_address] == right_val else 0
        else:
            print("Invalid operation!")
            exit(1)

    # we don't really care what w is, since we necessarily wipe it
    return memory[1], memory[2], memory[3]


def process_all(number):
    x, y, z = 0, 0, 0
    power = int(math.log10(number))
    for i in range(len(_grouped_ops)):
        input_digit = int(number / (10 ** power))
        number -= input_digit * (10 ** power)
        power -= 1

        print(input_digit)

        x, y, z = process_group(input_digit, x, y, z, i)

    return x, y, z


def iterate_numbers():
    i = 0
    for num in yield_next_number():
        x, y, z = process_all(num)
        i += 1
        if i % 100_000 == 0:
            print(i)

        if z == 0:
            print(num, (x, y, z))
            return


print(process_all(13579246899999))

# iterate_numbers()

# valid_range = {14: {1: [0], 2: [0], 3: [0], 4: [0], 5: [0], 6: [0], 7: [0], 8: [0], 9: [0]}}
# # valid_range = {13: {1: [14], 2: [15], 3: [16], 4: [17], 5: [18], 6: [19], 7: [20], 8: [21], 9: [22]}}
#
# print(list(range(13, -1, -1)))
#
# for ind in range(13, -1, -1):
#     for w in valid_range[ind + 1].keys():
#         for z in range(-10000, 10000):
#             x, y, z_out = process_group(w, 0, 0, z, ind)
#             if x is None:
#                 continue
#
#             if z_out in valid_range[ind + 1][w]:
#                 if ind not in valid_range:
#                     valid_range[ind] = {}
#                 if w not in valid_range[ind]:
#                     valid_range[ind][w] = []
#
#                 valid_range[ind][w].append(z)
#
#     print(ind, valid_range[ind])
#     # break
#
# print(valid_range)

# need to find the range of (z,w) that must be given to the last group to => z = 0
# then find the next range of values, and the next, etc.

# def inverse_group(w, x, y, z, ind):
#     group = _grouped_ops[ind]
#
#
#
#
# x, y, z = 0, 0, 0
# for ind in range(14):
#     for w in range(1, 10):
#         x, y, z = process_group(w, x, y, z, ind)
#         if z not in range(-10000, 10000):
#             continue
#         else:
#             print(ind, w)
