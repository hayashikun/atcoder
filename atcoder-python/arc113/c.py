def val(s):
    c = 0
    ss = list(s)
    size = len(s)

    found = True
    while found:
        found = False
        for i in range(size - 3, -1, -1):
            if ss[i] == ss[i + 1] and ss[i] != ss[i + 2]:
                ss[i + 2] = ss[i]
                found = True
                c += 1

    return c


def run(s):
    size = len(s)
    ss = list(s)
    c = 0

    idx = -1
    for i in range(size - 3, -1, -1):
        if ss[i] == ss[i + 1] and ss[i] != ss[i + 2]:
            c += (size - i - 2)

            if idx != -1:
                if ss[idx] == ss[i]:
                    c -= (size - idx - 1)
                c -= len([_s for _s in ss[i + 2:idx + 1] if _s == ss[i]])

            idx = i

    return c


def check():
    print(run("accept"), 3)
    print(run("atcoder"), 0)
    print(run("anerroroccurred"), 16)
    print(run("anerrccoroccurred"), 22)


def submit():
    print(run(input()))


# check()
submit()
