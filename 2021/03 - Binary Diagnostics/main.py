from typing import List


def bin_array_to_int(a: List[int]) -> int:
    a_int = 0
    for i in range(len(a)):
        pow = 12 - i - 1
        a_int += a[i] * (2 ** (pow))

    return a_int


def mul_bitlist(a: List[int], b: List[int]) -> int:
    return bin_array_to_int(a) * bin_array_to_int(b)


def add_to_oxygen(bit_freq: List[int], line_vals: List[int]) -> bool:
    for i in range(12):
        if line_vals[i] == 1 and bit_freq[i] < 0:
            return False
        if line_vals[i] == 0 and bit_freq[i] > 0:
            return False

    return True


def line_to_bits(linestr: str) -> List[int]:
    bit_list = []
    for char in linestr:
        if char == "1":
            bit_list.append(1)
        elif char == "0":
            bit_list.append(0)

    return bit_list


def get_bit_frequency(lines: List[List[int]]) -> List[int]:
    freq = [0 for _ in range(12)]

    for line in lines:
        i = 0
        for bit in line:
            if bit == "1":
                freq[i] += 1
            elif bit == "0":
                freq[i] -= 1

            i += 1

    return freq


with open("./input", "r") as file:
    lines = [line for line in file]
    bit_frequency = get_bit_frequency(lines)

    epsilon = [1 if n > 0 else 0 for n in bit_frequency]
    gamma = [0 if n > 0 else 1 for n in bit_frequency]

    print("part 1", mul_bitlist(epsilon, gamma))

    oxygen_lines = lines.copy()
    co2_lines = lines.copy()

    examine_bit = 0
    while len(oxygen_lines) > 1:
        oxygen_freq = get_bit_frequency(oxygen_lines)
        most_common = 1 if oxygen_freq[examine_bit] >= 0 else 0
        oxygen_lines = list(filter(lambda l: int(l[examine_bit]) == most_common, oxygen_lines))

        examine_bit += 1

    examine_bit = 0
    while len(co2_lines) > 1:
        co2_freq = get_bit_frequency(co2_lines)
        least_common = 0 if co2_freq[examine_bit] >= 0 else 1
        co2_lines = list(filter(lambda l: int(l[examine_bit]) == least_common, co2_lines))

        examine_bit += 1

    print("part 2", mul_bitlist(line_to_bits(oxygen_lines[0]), line_to_bits(co2_lines[0])))


