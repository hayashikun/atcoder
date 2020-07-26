import numpy as np


def app():
    _ = input()
    a = np.array(list(map(int, input().split())))
    c = 0
    while True:
        if (a % 2).any():
            break
        else:
            c += 1
            a = a / 2
    print(c)


app()
