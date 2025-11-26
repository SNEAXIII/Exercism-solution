def saddle_points(matrix):
    if not matrix:
        return []
    line_size = len(matrix[0])
    for line in matrix:
        if len(line) != line_size:
            raise ValueError("irregular matrix")
    results = set()
    for y, line in enumerate(matrix):
        maxi = max(line)
        for x, number in enumerate(line):
            column = [_line[x] for _line in matrix]
            mini = min(column)
            if number == maxi and number == mini:
                results.add((x, y))
    return [{"column": x + 1, "row": y + 1} for x, y in results]