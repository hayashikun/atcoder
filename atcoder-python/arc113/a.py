def run():
    k = int(input())

    n = 0
    for a in range(1, k + 1):
        for b in range(1, k // a + 1):
            ab = a * b
            for c in range(1, k // ab + 1):
                if ab * c <= k:
                    n += 1

    print(n)


run()
