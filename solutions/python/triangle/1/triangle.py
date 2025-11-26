def is_correct_triangle(sides):
    a, b, c = sides
    if not all((a + b >= c, b + c >= a, a + c >= b)):
        return False
    return all(side > 0 for side in sides)

def equilateral(sides):
    if not is_correct_triangle(sides):
        return False
    a, b, c = sides
    if not (a == b == c):
        return False
    return True

def isosceles(sides):
    if not is_correct_triangle(sides):
        return False
    a, b, c = sides
    if not any((a == b, b == c, a == c)):
        return False
    return True


def scalene(sides):
    if not is_correct_triangle(sides):
        return False
    a, b, c = sides
    if not (a != b != c != a):
        return False
    return True