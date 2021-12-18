import math
import re
from typing import Optional


def combine_numbers(a: str, b: str) -> str:
    return f"[{a},{b}]"


def find_deep_nested_bracket(expression: str) -> Optional[tuple[int, int, int]]:
    depth = 0
    last_opening_bracket = 0
    last_comma = 0
    for i in range(len(expression)):
        if expression[i] == "[":
            last_opening_bracket = i
            depth += 1
        elif expression[i] == "]":
            if depth > 4:
                return last_opening_bracket, last_comma, i
            depth -= 1
        elif expression[i] == ",":
            last_comma = i

    return None


def try_explode(expression: str) -> str:
    position = find_deep_nested_bracket(expression)
    if position is None:
        return expression

    left, comma, right = position

    # grab the values and then remove the nested expression
    lv = int(expression[left + 1:comma])
    rv = int(expression[comma + 1:right])

    expression = expression[:left] + "0" + expression[right + 1:]

    matches = re.search(r"(\d+)[^\d]*$", expression[:left])
    if matches is not None:
        left_num = int(matches[1]) + lv

        expression = expression[:matches.start()] + str(left_num) + expression[matches.start() + len(matches[1]):]
        right = left + len(str(left_num))
    else:
        right = left + 1

    matches = re.search(r"(\d+)", expression[right:])
    if matches is not None:
        right_num = int(matches[1]) + rv

        expression = expression[:matches.start() + right] + str(right_num) + expression[matches.end() + right:]

    return expression


def try_split(expression: str) -> str:
    matches = re.search(r"(\d{2})", expression)

    if matches is None:
        return expression
    else:
        val = int(matches[1])
        l, r = math.floor(val / 2), math.ceil(val / 2)
        replace = f"[{str(l)},{str(r)}]"

        expression = expression[:matches.start()] + replace + expression[matches.end():]

    return expression


def sum_snail_numbers(a: str, b: str) -> str:
    expr = combine_numbers(a, b)

    while True:
        before = expr
        expr = try_explode(expr)
        if before != expr:
            # allow trying another explosion
            continue

        expr = try_split(expr)
        if before == expr:
            # we've not exploded or split, we've reached a stable point
            break

    return expr


def calculate_magnitude(expression: str) -> int:
    while True:
        match = re.search(r"(\[\d+,\d+\])", expression)
        if match is None:
            break

        digit_matches = [int(d) for d in re.findall(r"(\d+)", match[1])]
        replace = (3 * digit_matches[0]) + (2 * digit_matches[1])

        expression = expression[:match.start()] + str(replace) + expression[match.end():]

    return int(expression)


explode_tests = [
    ('[[[[0,7],4],[7,[[8,4],9]]],[1,1]]', '[[[[0,7],4],[15,[0,13]]],[1,1]]'),
    ('[[[[5,0],[[9,7],[9,6]]],[[4,[1,2]],[[1,4],2]]],[[[5,[2,8]],4],[5,[[9,9],0]]]]',
     '[[[[5,9],[0,[16,6]]],[[4,[1,2]],[[1,4],2]]],[[[5,[2,8]],4],[5,[[9,9],0]]]]'),
    ('[[6,[5,[14,[3,2]]]],13]', '[[6,[5,[17,0]]],15]'),
    ('[[[[[13,11],1],2],3],4]', '[[[[0,12],2],3],4]'),
    ('[[[[[9,8],1],2],3],4]', '[[[[0,9],2],3],4]'),
    ('[7,[6,[5,[4,[3,2]]]]]', '[7,[6,[5,[7,0]]]]'),
    ('[[6,[5,[4,[3,2]]]],1]', '[[6,[5,[7,0]]],3]'),
    ('[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]', '[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]'),
    ('[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]', '[[3,[2,[8,0]]],[9,[5,[7,0]]]]'),
]

for val, expected in explode_tests:
    actual = try_explode(val)
    if actual != expected:
        print("Failed to correctly explode value!", val, "actual", actual, "expected", expected)
        exit()

print("Explode tests passed")

split_tests = [
    ('[10,5]', '[[5,5],5]'),
    ('[11,5]', '[[5,6],5]'),
    ('[5,11]', '[5,[5,6]]'),
    ('[11,11]', '[[5,6],11]'),
]

for val, expected in split_tests:
    actual = try_split(val)
    if actual != expected:
        print("Failed to correctly split value!", val, "expected", expected, "actual", actual)
        exit()

print("Split tests passed")

sum_tests = [
    ('[[[[4,3],4],4],[7,[[8,4],9]]]', '[1,1]', '[[[[0,7],4],[[7,8],[6,0]]],[8,1]]'),
    ('[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]', '[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]',
     '[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]'),
]

for a, b, expected in sum_tests:
    actual = sum_snail_numbers(a, b)
    if actual != expected:
        print(a)
        print("+", b)
        print("Failed to correctly sum values!", "expected", expected, "actual", actual)
        exit()

print("Sum tests passed")

magnitude_tests = [
    ('[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]', 4140)
]

for expr, expected in magnitude_tests:
    actual = calculate_magnitude(expr)
    if actual != expected:
        print("Failed to correctly calculate magnitude!", "actual", actual, "expected", expected)
        exit()

print("Magnitude tests passed")

with open("input") as f:
    lines = f.read().strip().split("\n")

running_sum = None
for line in lines:
    if running_sum is not None:
        running_sum = sum_snail_numbers(running_sum, line)
    else:
        running_sum = line

print("part1", calculate_magnitude(running_sum))

largest = 0
for a in lines:
    for b in lines:
        if a == b:
            continue

        largest = max(largest, calculate_magnitude(sum_snail_numbers(a, b)))

print("part2", largest)

