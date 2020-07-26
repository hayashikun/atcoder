def app():
    print("{} {} {}".format(*f_xyz(*map(int, input().split()))))


def f_xyz(n, m):
    if m == n * 10000:
        return n, 0, 0
    for x in range(n):
        y, z = f_yz(n - x, m - x * 10000)
        if y >= 0:
            return x, y, z
    return -1, -1, -1


def f_yz(n, m):
    if m == n * 5000:
        return n, 0
    if n == 0:
        return -1, -1
    for y in range(n):
        z = f_z(n - y, m - y * 5000)
        if z >= 0:
            return y, z
    return -1, -1


def f_z(n, m):
    return n if m == n * 1000 else -1


app()
