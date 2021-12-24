import fileinput
from functools import cache

# by inspection, there are 14 modulo operations, and none of them does anything! The result of the modulo operations
# are not considered before the value is then compared (eq) to both w and 0. Comparing a value to anything and then 0
# necessarily


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


# the commented out below method tries a look-back solution to find a valid range for each given input
# the only real missing link is, "what range do I need to consider for any given expected output"?

# valid_range = {14: {1: [0], 2: [0], 3: [0], 4: [0], 5: [0], 6: [0], 7: [0], 8: [0], 9: [0]}}
# # valid_range = {13: {1: [14], 2: [15], 3: [16], 4: [17], 5: [18], 6: [19], 7: [20], 8: [21], 9: [22]}}
#
# print(list(range(13, -1, -1)))
#
# for ind in range(13, -1, -1):
#     a = 1
#     for w in valid_range[ind + 1]:
#         valid_z = valid_range[ind + 1][w]
#         z_range = (max(valid_z) + 1) * 50
#         for z in range(-z_range, z_range):
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
#     print(ind)
#     # break
#
# print(valid_range)

# need to find the range of (z,w) that must be given to the last group to => z = 0
# then find the next range of values, and the next, etc.

# store the list of states (z values, that's all we care about) and the possible input values that can reach there.
states = {0: [0]}

# now, iterate through each group and update the state. In our case, we only care for the min and max value, so only
# store those at each iteration. We have no real information for what's a good "z" value at any stage except the last
# one, so we can't throw anything away. There was an attempted solution above that tried to find valid ranges of z at
# each step, but this eventually failed.

# this heavily abuses the fact that w, x, and y values are thrown away after each input, so we don't need to track their
# state, we just need to consider all possible w (1-9) at each successive iteration.
for i in range(len(_grouped_ops)):
    new_states = {}
    for z in states:
        for j in range(1, 10):
            numbers = states[z]
            max_n = max(numbers)
            min_n = min(numbers)
            _, _, new_z = process_group(j, 0, 0, z, i)

            if new_z not in new_states:
                new_states[new_z] = []

            new_states[new_z].append((max_n * 10) + j)
            new_states[new_z].append((min_n * 10) + j)

    states = new_states
    print(i, len(new_states))

print("part1", max(states[0]))
print("part2", min(states[0]))
