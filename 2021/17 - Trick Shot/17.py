def y_pos(initial_y: int, vy: int, steps: int) -> int:
    return int(initial_y + (vy * steps) - ((steps / 2) * (steps + 1)))


def x_pos(initial_x: int, vx: int, steps: int) -> int:
    pos = initial_x
    for _ in range(steps):
        pos += vx
        if vx < 0:
            xd = 1
        elif vx > 0:
            xd = -1
        else:
            xd = 0
        vx += xd
    return pos


target_x = range(102, 157)
target_y = range(-146, -90)

valid = set()
for x in range(14, 158):
    for y in range(-146, 147):
        i = 0
        y_after_i_steps = y_pos(0, y, i)
        x_after_i_steps = x_pos(0, x, i)
        while y_after_i_steps >= -146 and x_after_i_steps <= 157:
            if y_after_i_steps in target_y and x_after_i_steps in target_x:
                valid.add((x, y))
                break
            else:
                i += 1
                y_after_i_steps = y_pos(0, y, i)
                x_after_i_steps = x_pos(0, x, i)

print(len(valid))

# # To calculate the highest y speed, I'm unclear why we can't have arbitrarily high y speeds atm
# highest = 0
# initial_y_velocity = 0
# while True:
#     i = 0
#     y_after_i_steps = y_pos(0, initial_y_velocity, i)
#     while y_after_i_steps >= -146:
#         if y_after_i_steps in target_y:
#             highest = max(highest, initial_y_velocity)
#             print(highest, i)
#             break
#         else:
#             i += 1
#             y_after_i_steps = y_pos(0, initial_y_velocity, i)
#     initial_y_velocity += 1
