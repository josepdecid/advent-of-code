import os
from collections import defaultdict

list_1 = []
list_2 = []

with open(os.path.dirname(__file__) + '/input.txt', mode='r') as f:
    for raw_line in f.readlines():
        values = raw_line.strip().split('   ')
        list_1.append(int(values[0]))
        list_2.append(int(values[1]))

list_2_counts = defaultdict(int)
for number in list_2:
    list_2_counts[number] += 1

score = 0
for number in list_1:
    score += number * list_2_counts[number]

print(score)  # 23046913

