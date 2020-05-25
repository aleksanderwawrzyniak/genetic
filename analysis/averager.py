import sys
from operator import add, floordiv

# initialize data
with open(sys.argv[1]) as f:
    data = f.readlines()

if data[-1] == '':
    data.pop()

data[:] = [float(row) for row in data]

for i in range(2, len(sys.argv)):
    with open(sys.argv[i]) as f:
        file_data = f.readlines()

    if file_data[-1] == '':
        file_data.pop()

    file_data = [float(row) for row in data]

    assert len(file_data) == len(data)
    data = list(map(add, data, file_data))

divider = float(len(sys.argv) - 1)
data[:] = [x / divider for x in data]

for x in data:
    print(x)
