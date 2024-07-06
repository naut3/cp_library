def prime_factorize(n):
    pf = []

    for p in range(2, n + 1):
        if p * p > n:
            break

        if n % p != 0:
            continue

        e = 0
        while n % p == 0:
            e += 1
            n //= p

        pf.append((p, e))

    if n != 1:
        pf.append((n, 1))

    return pf
