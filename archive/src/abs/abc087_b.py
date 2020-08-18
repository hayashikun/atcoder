def app():
    print(f_abc(int(input()), int(input()), int(input()), int(input())))


def f_abc(a, b, c, x):
    return sum([f_bc(b, c, x - n * 500) for n in range(min(a, x // 500) + 1)])


def f_bc(b, c, x):
    return sum([f_c(c, x - n * 100) for n in range(min(b, x // 100) + 1)])


def f_c(c, x):
    return int(x % 50 == 0 and c >= x // 50)


app()
