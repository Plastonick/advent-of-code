# cups = list(range(1, num_cups))

# print(*range(1, 10))
# exit()

import time

part_a = False

if part_a:
    debug = True
    cups = [2, 8, 4, 5, 7, 3, 9, 6, 1]
    goes = 100
else:
    debug = False
    cups = [2, 8, 4, 5, 7, 3, 9, 6, 1] + [*range(10, 1_000_001)]
    goes = 10_000_000
    goes = 1000

num_cups = len(cups)

current_index = 0


def get_next_value(current, maximum, exclude):
    next_value = current - 1

    if next_value in exclude:
        return get_next_value(next_value, maximum, exclude)
    elif next_value > 0:
        return next_value
    else:
        return get_next_value(maximum + 1, maximum, exclude)


picking_up_time = 0
finding_destination_time = 0
modifying_time = 0
modifying_time2 = 0
modifying_time3 = 0

for i in range(0, goes):
    if i % 100 == 0:
        print(i / 10000000)

    current_val = cups[current_index]
    if debug:
        print("current cup =>", current_val)
        print(cups)

    start = time.perf_counter_ns()
    if current_index + 4 >= num_cups:
        picked_up = cups[current_index + 1:current_index + 4] + cups[0:current_index + 4 - num_cups]
        del cups[slice(current_index + 1, current_index + 4)]
        del cups[slice(0, current_index + 4 - num_cups)]
    else:
        picked_up = cups[current_index + 1:current_index + 4]
        del cups[slice(current_index + 1, current_index + 4)]

    picking_up_time += time.perf_counter_ns() - start

    if debug:
        print("pick up", picked_up)
        print(cups)

    start = time.perf_counter_ns()
    destination_cup_val = get_next_value(current_val, num_cups, picked_up)

    if debug:
        print("destination cup =>", destination_cup_val)

    destination_cup_index = cups.index(destination_cup_val)
    finding_destination_time += time.perf_counter_ns() - start

    start = time.perf_counter_ns()
    first = cups[:destination_cup_index + 1]
    second = cups[destination_cup_index + 1:]
    modifying_time += time.perf_counter_ns() - start

    start = time.perf_counter_ns()
    if debug:
        print(first)
        print(second)

    cups = first + picked_up + second
    modifying_time2 += time.perf_counter_ns() - start
    start = time.perf_counter_ns()

    current_index = (cups.index(current_val) + 1) % num_cups

    modifying_time3 += time.perf_counter_ns() - start

    if debug:
        print()

print("picking_up_time", round(picking_up_time / 1_000_000), "milliseconds")
print("finding_destination_time", round(finding_destination_time / 1_000_000), "milliseconds")
print("modifying_time", round(modifying_time / 1_000_000), "milliseconds")
print("modifying_time2", round(modifying_time2 / 1_000_000), "milliseconds")
print("modifying_time3", round(modifying_time3 / 1_000_000), "milliseconds")

if part_a:
    print(cups)
