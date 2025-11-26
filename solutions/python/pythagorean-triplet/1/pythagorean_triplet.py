def triplets_with_sum(N):
    result = []
    if N < 12 or N % 2 != 0:
        return result
    for a in range(1, N // 3):
        a_squared = a * a
        for b in range(a, N // 2):
            if (c:=N - a - b) <= b:
                break
            if a_squared + b * b == c * c:
                result.append([a, b, c])
    return result