import matplotlib.pyplot as plt

import sys

data = []
for i in range(3, len(sys.argv)):
    with open(sys.argv[i]) as f:
        file_data = f.readlines()

    # if the last line in the file is empty, just remove it
    if file_data[-1] == '':
        file_data.pop()

    file_data = [float(row) for row in file_data]
    file_name = sys.argv[i].split('/')[-1].replace(".txt", "")
    data.append((range(len(file_data)), file_data, file_name))


plt.xlabel('iterations')
plt.ylabel('best individual')
plt.title(sys.argv[1])

for i in range(len(sys.argv) - 3):
    plt.plot(data[i][0], data[i][1], label=data[i][2])

# sort legend entries
handles, labels = plt.gca().get_legend_handles_labels()
labels, handles = zip(*sorted(zip(labels, handles), key=lambda t: t[0]))

plt.legend(handles, labels)
plt.savefig(sys.argv[2], quality=95, optimize=False, dpi=1000)
