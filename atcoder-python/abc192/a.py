def run():
    num = int(input())
    if num % 100 == 0:
        print(100)
    else:
        a = ((num // 100) + 1) * 100 - num
        print(a)


run()
