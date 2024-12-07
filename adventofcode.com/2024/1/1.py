import os

list_1 = []
list_2 = []

with open(os.path.dirname(__file__) + '/input.txt', mode='r') as f:
    for raw_line in f.readlines():
        values = raw_line.strip().split('   ')
        list_1.append(int(values[0]))
        list_2.append(int(values[1]))

list_1.sort()
list_2.sort()

distance = 0
for i in range(len(list_1)):
    distance += abs(list_1[i] - list_2[i])

print(distance)  # 1580061

