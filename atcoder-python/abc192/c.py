def f(ns):
    sns = "".join(sorted(ns))
    min_n = int(sns)
    max_n = int(sns[::-1])
    return max_n - min_n


def run():
    ns, n = input().split()
    for _ in range(int(n)):
        ns = str(f(ns))
    print(ns)


run()
