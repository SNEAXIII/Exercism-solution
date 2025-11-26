digits = "zero one two three four five six seven eight nine".split()
digits_plus_10 = "ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen".split()
tens = "_ ten twenty thirty forty fifty sixty seventy eighty ninety".split()
power_conversion = (9, "billion"), (6, "million"), (3, "thousand")

def say_under_1000(number: int, temp_build: list, string: str | None = None) -> None:
    if number >= 1000:
        raise ValueError(
            f"The number {number} is not valid, it should be between 0 and 999")
    if hundred := number // 100:
        temp_build.append(f"{digits[hundred]} hundred")
        number -= hundred * 100
    ten, digit = number // 10, number % 10
    if 20 > number >= 10:
        temp_build.append(digits_plus_10[number-10])
    elif ten and digit:
        temp_build.append(f"{tens[ten]}-{digits[digit]}")
    else:
        if ten:
            temp_build.append(tens[ten])
        if digit:
            temp_build.append(digits[digit])
    if string:
        temp_build.append(string)


def say(number: int) -> str:
    temp_build = []
    if number == 0:
        return "zero"
    if number < 0 or number >= 10 ** 12:
        raise ValueError("input out of range")
    for power, string in power_conversion:
        if number >= (pow10 := 10 ** power):
            cutted_number = number // pow10
            say_under_1000(cutted_number, temp_build, string)
            number -= pow10 * cutted_number
    say_under_1000(number, temp_build)
    return " ".join(temp_build)
