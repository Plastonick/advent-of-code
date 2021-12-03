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


with open("./input", "r") as file:
    bit_frequency = [0 for i in range(12)]

    for line in file:
        i = 0
        for bit in line:
            if bit == "1":
                bit_frequency[i] += 1
            elif bit == "0":
                bit_frequency[i] -= 1

            i += 1

    epsilon = [1 if n > 0 else 0 for n in bit_frequency]
    gamma = [0 if n > 0 else 1 for n in bit_frequency]

    print("part 1", mul_bitlist(epsilon, gamma))
