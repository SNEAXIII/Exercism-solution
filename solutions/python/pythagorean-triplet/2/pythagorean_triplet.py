def triplets_with_sum(n):
    result = []
    if n < 12 or n % 2 != 0:
        return result
    max_a = int(n / 3.3)
    for a in range(3, max_a + 1):
        a_squared = a * a
        remaining = n - a
        for b in range(a, remaining // 2 + 1):
            c = remaining - b
            if a_squared + b * b == c * c:
                result.append([a, b, c])
    return result