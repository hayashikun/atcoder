def app():
    n, a, b = map(int, input().split())
    r = 0
    for m in range(1, n + 1):
        s = d_sum(m)
        if a <= s <= b:
            r += m
    print(r)


def d_sum(d):
    if d < 10:
        return d
    return d % 10 + d_sum(d // 10)


app()
