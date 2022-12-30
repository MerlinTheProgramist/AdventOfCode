def check_validity(w, z, block_num, to_validate, part_two=False):
    to_validate = to_validate.copy()
    to_validate.append(w)

    # Varied values for this block
    z_value = z_vars[block_num]
    x_value = x_vars[block_num]
    y_value = y_vars[block_num]

    # Check w, or all still valid at this block w values for validity
    if z % 26 + x_value != w and x_value < 0:
        return False

    # Calculate z to get new z value / final validation value
    x = z
    x %= 26
    z //= z_value
    x += x_value
    x = 1 if x == w else 0
    x = 1 if x == 0 else 0
    y = 25
    y *= x
    y += 1
    z *= y
    y = w + y_value
    y = y * x
    z += y

    if block_num == 13:
        if z == 0:
            return to_validate
        else:
            return False

    # Digits are valid but model number not yet complete so recursion
    if not part_two:
        for i in range(9, 0, -1):
            validity_check = check_validity(i, z, block_num + 1, to_validate)
            if type(validity_check) is list:
                return validity_check
    else:
        for i in range(1, 10):
            validity_check = check_validity(i, z, block_num + 1,
                                            to_validate, part_two=True)

            if type(validity_check) is list:
                return validity_check

    return False


with open('ALU.txt', 'r') as aoc_input:
    lines = [line.strip().split(' ') for line in aoc_input.readlines()]


z_vars = []
x_vars = []
y_vars = []

# Collect all numbers that vary for easy use in function via index
i = 4
while i < len(lines):
    z_vars.append(int(lines[i][2]))
    x_vars.append(int(lines[i + 1][2]))
    y_vars.append(int(lines[i + 11][2]))

    i += 18

highest_valid = None
for i in range(9, 0, -1):
    validity_check = check_validity(i, 0, 0, [])
    if type(validity_check) is list:
        highest_valid = ''.join(map(str, validity_check))
        break

# Answer One
print(f'Largest model number accepted by MONAD: {highest_valid}')

lowest_valid = None
for i in range(1, 10):
    validity_check = check_validity(i, 0, 0, [], part_two=True)
    if type(validity_check) is list:
        lowest_valid = ''.join(map(str, validity_check))
        break

# Answer One
print(f'Lowest model number accepted by MONAD: {lowest_valid}')