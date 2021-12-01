# import time

part_a = False

if part_a:
    debug = True
    cups = [2, 8, 4, 5, 7, 3, 9, 6, 1]
    goes = 100
else:
    debug = False
    cups = [2, 8, 4, 5, 7, 3, 9, 6, 1] + [n for n in range(10, 1_000_001)]
    goes = 10_000_000
    # goes = 100


def get_next_value(current, maximum, exclude):
    next_value = current - 1

    if next_value in exclude:
        return get_next_value(next_value, maximum, exclude)
    elif next_value > 0:
        return next_value
    else:
        return get_next_value(maximum + 1, maximum, exclude)


successors = {}
max_cup = max(cups)
last_cup = max_cup
for idx in range(len(cups)):
    this_cup = cups[idx]
    prev_cup = cups[idx - 1]

    successors[prev_cup] = this_cup

current_cup = cups[0]

for _ in range(goes):
    cup1 = successors[current_cup]
    cup2 = successors[cup1]
    cup3 = successors[cup2]

    picked_up = [cup1, cup2, cup3]

    destination_value = get_next_value(current_cup, max_cup, picked_up)

    currently_after_destination = successors[destination_value]
    currently_after_cup3 = successors[cup3]

    # tell the successors list that cup1 comes after the destination value
    # and what was previously after the destination value, now comes after cup 3
    # cup 2 implicitly remains between cups 1 and 3
    successors[current_cup] = currently_after_cup3
    successors[destination_value] = cup1
    successors[cup3] = currently_after_destination

    current_cup = currently_after_cup3

after_1 = successors[1]
after_after_1 = successors[after_1]

print(after_1 * after_after_1)
