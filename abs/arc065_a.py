def app():
    s = input()
    ps = ['dream', 'dreamer', 'erase', 'eraser']

    while s != '':
        for p in ps:
            if s.endswith(p):
                s = s[:-len(p)]
                break
        else:
            print('NO')
            break
    else:
        print('YES')


app()
