import fileinput
import math


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

for line in fileinput.input():
    terms = line.strip().split(" ")
    if len(terms) == 3:
        _op, _left, _right = terms
        _operations.append((_op, _left, _right))
    else:
        _op, _left = terms
        _operations.append((_op, _left, None))


def process(number, operations):
    memory = [number, 0, 0, 0]
    power = int(math.log10(number))
    for op, left, right in operations:
        left_address = vars[left]

        if op == 'inp':
            input = int(number / (10 ** power))
            number -= input * (10 ** power)
            power -= 1

            memory[left_address] = input
            continue

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
                return None

            memory[left_address] = int(memory[left_address] / right_val)
        elif op == 'mod':
            if memory[left_address] < 0 or right_val <= 0:
                return None

            memory[left_address] = memory[left_address] % right_val
        elif op == 'eql':
            memory[left_address] = 1 if memory[left_address] == right_val else 0
        else:
            print("Invalid operation")
            exit(1)

    return memory


for num in yield_next_number():
    output = process(num, _operations)
    if output is not None and output[3] == 0:
        print(num, output)
        break
