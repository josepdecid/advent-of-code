import os
import re

result = 0

with open(os.path.dirname(__file__) + '/input.txt', mode='r') as f:
    for raw_line in f.readlines():
        for operation in re.findall(r'mul\(\d+,\d+\)', raw_line):
            values = operation[4:-1].split(',')
            result += int(values[0]) * int(values[1])

print(result)  # 170778545

