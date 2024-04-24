import random
import numpy as np

N = 20

coefficients = [[random.randint(-10, 10) for _ in range(N)] for _ in range(N)]
constatns = [random.randint(-10, 10) for _ in range(N)]

for i in coefficients:
    for j in i:
        print(f"{j:.1f}", end=', ')
    print()

print("\n\n")
for i in constatns:
    print(f"{i:.1f}", end=", ")
print()


print(np.linalg.solve(np.array(coefficients), np.array(constatns)))