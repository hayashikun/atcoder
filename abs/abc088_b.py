def app():
    _ = input()
    cards = sorted(map(int, input().split()), reverse=True)
    a = sum(cards[::2])
    b = sum(cards[1::2])
    print(a - b)


app()
