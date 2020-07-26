import numpy as np


def app():
    n = int(input())
    ts = np.empty(shape=(n + 1,), dtype=int)
    xs = np.empty_like(ts)
    ys = np.empty_like(ts)
    ts[0], xs[0], ys[0] = 0, 0, 0
    for i in range(n):
        ts[i + 1], xs[i + 1], ys[i + 1] = map(int, input().split())
    d = np.abs(np.diff(xs)) + np.abs(np.diff(ys))
    td = np.diff(ts)
    if ((d + td) % 2 == 0).all() and (td >= d).all():
        print('Yes')
    else:
        print('No')


app()
