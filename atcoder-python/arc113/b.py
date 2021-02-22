def run(a, b, c):
    for i in "0156":
        if a[-1] == i:
            return i

    if a[-1] in "49":
        if int(b[-1]) % 2 == 0:
            return 10 - int(a[-1])
        else:
            return a[-1]

    if a[-1] in "28":
        m = (int(b[-1]) ** int(c)) % 4
        if m == 0:
            return 6
        elif m == 1:
            return a[-1]
        elif m == 2:
            return 4
        else:
            return 10 - int(a[-1])

    if a[-1] in "37":
        m = (int(b[-1]) ** int(c)) % 4
        if m == 0:
            return 1
        elif m == 1:
            return a[-1]
        elif m == 2:
            return 9
        else:
            return 10 - int(a[-1])


def submit():
    print(run(*input().split()))


submit()
