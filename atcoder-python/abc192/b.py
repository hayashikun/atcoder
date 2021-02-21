def run():
    s = input()
    y = True

    for i, c in enumerate(s):
        if not ((i % 2 == 0 and c.islower()) or (i % 2 == 1 and c.isupper())):
            y = False

    print("Yes" if y else "No")


run()
