import os
import re

result = 0
enabled = True

with open(os.path.dirname(__file__) + '/input.txt', mode='r') as f:
    for raw_line in f.readlines():
        for operation in re.findall(r'mul\(\d+,\d+\)|do\(\)|don\'t\(\)', raw_line):
            if operation == 'do()':
                enabled = True
            elif operation == 'don\'t()':
                enabled = False
            else:
                if enabled:
                    values = operation[4:-1].split(',')
                    result += int(values[0]) * int(values[1])

print(result)  # 82868252

