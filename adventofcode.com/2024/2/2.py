import os

def check_is_safe(values, rec=False):
    if len(values) < 2:
        return True

    increasing = values[0] < values[1]
    for i in range(len(values) - 1):
        if increasing:
            if values[i + 1] <= values[i] or values[i + 1] - values[i] > 3:
                break
        else:
            if values[i] <= values[i + 1] or values[i] - values[i + 1] > 3:
                break
    else:
        return True

    if rec:
        for fix in (values[:i] + values[i+1:] for i in range(len(values))):
            if check_is_safe(fix):
                return True

    return False

safe_count = 0
with open(os.path.dirname(__file__) + '/input.txt', mode='r') as f:
    for raw_line in f.readlines():
        values = list(map(int, raw_line.split(' ')))
        if check_is_safe(values, rec=True):
            safe_count += 1

print(safe_count)
            

