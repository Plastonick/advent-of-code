# cups = list(range(1, num_cups))

# print(*range(1, 10))
# exit()

import time

arr = [1, 2, 3, 4, 5, 6]


class RingBuffer:
    def __init__(self, data):
        self.data = data
        self.size = len(data)

    def shift_three(self, from_idx, to_idx, direction=1):
        from_idx = from_idx % self.size
        to_idx = to_idx % self.size

        direction = 1

        idx = to_idx

        while True:
            target_idx = (idx + (3 * direction)) % self.size
            if self.data[idx] is not None:
                self.data[target_idx] = self.data[idx]

            if idx == from_idx:
                break

            idx = (idx - direction) % self.size

    def get(self, idx):
        return self.data[idx]

    def pop_range(self, from_idx, to_idx):
        idx = from_idx % self.size
        to_idx = to_idx % self.size
        ret = []
        while idx != to_idx:
            ret.append(self.data[idx])
            self.data[idx] = None

            idx = (idx + 1) % self.size

        return ret

    def index_of(self, val):
        return self.data.index(val)

    def set(self, idx, value):
        self.data[idx] = value


part_a = True

if part_a:
    debug = True
    cups = RingBuffer([2, 8, 4, 5, 7, 3, 9, 6, 1])
    goes = 100
else:
    debug = False
    cups = RingBuffer([2, 8, 4, 5, 7, 3, 9, 6, 1] + [*range(10, 1_000_001)])
    goes = 10_000_000
    goes = 1000

num_cups = cups.size

current_index = 0


def get_next_value(current, maximum, exclude):
    next_value = current - 1

    if next_value in exclude:
        return get_next_value(next_value, maximum, exclude)
    elif next_value > 0:
        return next_value
    else:
        return get_next_value(maximum + 1, maximum, exclude)


# s => starting index, d => destination index, m => maximum cup number
def iterator_step(s, d, m):
    if d > s:
        if d - s < m / 2:
            return 1
        else:
            return -1
    else:
        if s - d > m / 2:
            return 1
        else:
            return -1


picking_up_time = 0
finding_destination_time = 0
modifying_time = 0
modifying_time2 = 0
modifying_time3 = 0

for i in range(0, goes):
    if i % 100 == 0:
        print(i / 10000000)

    current_val = cups.get(current_index)
    if debug:
        print("current cup =>", current_val)
        print(cups.data)

    # select the picked up cups, and replace their current position with "None"
    start = time.perf_counter_ns()

    picked_up = cups.pop_range(current_index + 1, current_index + 4)

    picking_up_time += time.perf_counter_ns() - start

    if debug:
        print("pick up", picked_up)
        print(cups.data)

    start = time.perf_counter_ns()
    destination_cup_val = get_next_value(current_val, num_cups, picked_up)

    if debug:
        print("destination cup =>", destination_cup_val)

    destination_cup_index = cups.index_of(destination_cup_val)
    finding_destination_time += time.perf_counter_ns() - start

    start = time.perf_counter_ns()

    # we now need to move our three picked up cups _after_ the destination cup.
    # this is equivalent to either moving everything after the destination cup along three
    # OR moving the destination cup and everything before that back three spaces
    # in both cases, stopping at the three empty places the picked up cups used to inhabit
    cups.shift_three(destination_cup_index + 1, current_index, -1)

    idx = destination_cup_index
    for cup in picked_up:
        idx = (idx + 1) % num_cups
        cups.set(idx, cup)

    # first = cups[:destination_cup_index + 1]
    # second = cups[destination_cup_index + 1:]
    modifying_time += time.perf_counter_ns() - start

    start = time.perf_counter_ns()
    # if debug:
    #     print(first)
    #     print(second)
    #
    # cups = first + picked_up + second
    modifying_time2 += time.perf_counter_ns() - start
    start = time.perf_counter_ns()

    current_index = (cups.index_of(current_val) + 1) % num_cups

    modifying_time3 += time.perf_counter_ns() - start

    if debug:
        print()

print("picking_up_time", round(picking_up_time / 1_000_000), "milliseconds")
print("finding_destination_time", round(finding_destination_time / 1_000_000), "milliseconds")
print("modifying_time", round(modifying_time / 1_000_000), "milliseconds")
print("modifying_time2", round(modifying_time2 / 1_000_000), "milliseconds")
print("modifying_time3", round(modifying_time3 / 1_000_000), "milliseconds")

if part_a:
    print(cups.data)
else:
    cup_1_pos = cups.index_of(1)

    print(cups.get((cup_1_pos + 1) % cups.size))
    print(cups.get((cup_1_pos + 2) % cups.size))
