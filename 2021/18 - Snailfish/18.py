import math
import re
from typing import Optional


def combine_numbers(a: str, b: str) -> str:
    return f"[{a},{b}]"


def find_deep_nested_bracket(expression: str) -> Optional[tuple[int, int]]:
    depth = 0
    last_opening_bracket = 0
    for i in range(len(expression)):
        if expression[i] == "[":
            last_opening_bracket = i
            depth += 1
        elif expression[i] == "]":
            if depth > 4:
                return last_opening_bracket, i
            depth -= 1

    return None


def try_explode(expression: str) -> str:
    position = find_deep_nested_bracket(expression)
    if position is None:
        return expression

    left, right = position

    # grab the values and then remove the nested expression
    lv = int(expression[left + 1])
    rv = int(expression[right - 1])

    expression = expression[:left] + "0" + expression[right + 1:]
    right = left + 1

    left_pointer = left - 1
    while left_pointer >= 0:
        if expression[left_pointer] in '1234567890':
            left_num = int(expression[left_pointer]) + lv
            if left_num >= 10:
                # everything will have shifted to the right...
                right += 1

            expression = expression[:left_pointer] + str(left_num) + expression[left_pointer + 1:]
            break
        left_pointer -= 1

    right_pointer = right
    while right_pointer < len(expression):
        if expression[right_pointer] in '1234567890':
            right_num = int(expression[right_pointer]) + rv

            expression = expression[:right_pointer] + str(right_num) + expression[right_pointer + 1:]
            break
        right_pointer += 1

    return expression


def try_split(expression: str) -> str:
    matches = re.search(r"\d{2}", expression)

    if matches is None:
        return expression
    else:
        val = int(matches.group())
        l, r = math.floor(val / 2), math.ceil(val / 2)
        replace = f"[{str(l)},{str(r)}]"
        expression = expression.replace(matches.group(), replace)

    return expression


def sum_snail_numbers(a: str, b: str) -> str:
    expr = combine_numbers(a, b)

    while True:
        before = expr
        expr = try_explode(expr)
        if before == expr:
            # we've not changed, try splitting
            expr = try_split(expr)
            if before == expr:
                # we've not exploded or split, we've reached a stable point
                break

    return expr


def calculate_magnitude(expression: str) -> int:
    return len(expression)


with open("example") as f:
    running_sum = None
    for line in f.read().strip().split("\n"):
        if running_sum is not None:
            running_sum = sum_snail_numbers(running_sum, line)
        else:
            running_sum = line

print(running_sum)
