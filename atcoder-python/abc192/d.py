def base_n_to_10(x, n):
    return sum([int(x[-i]) * (n ** (i - 1)) for i in range(1, len(str(x)) + 1)])


def xn(x1, d0, d1, n):
    return x1 + (n - 1) * (d0 - d1) + d1 / 2 * n * (n - 1)


def run():
    x = input()
    m = int(input())
    d = int(sorted(x)[-1])

    if len(x) == 1:
        if int(x) <= m:
            print(1)
        else:
            print(0)
        return

    n = d + 1
    x1 = base_n_to_10(x, n)
    if x1 > m:
        print(0)
        return

    n += 1
    x2 = base_n_to_10(x, n)
    if x2 > m:
        print(1)
        return

    n += 1
    x3 = base_n_to_10(x, n)
    if x3 > m:
        print(2)
        return

    d0 = x2 - x1
    d1 = x3 - x2 - d0

    n += 1

    for f in range(5, 0, -1):
        while xn(x1, d0, d1, n - d + 10 ** f) < m:
            n += 10 ** f

    while xn(x1, d0, d1, n - d) < m:
        n += 1

    print(n - d - 1)


run()
