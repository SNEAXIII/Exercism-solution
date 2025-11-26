def response(hey_bob: str):
    letters = [chr(index) for index in range(65, 91)]
    hey_bob = hey_bob.strip()
    is_quiet = len(hey_bob) == 0
    if is_quiet:
        return "Fine. Be that way!"
    is_question = hey_bob[-1] == "?"
    is_only_contains_upper_case_letter = hey_bob == hey_bob.upper()
    is_contains_any_uppercase_letter = any(cara for cara in hey_bob if cara in letters)
    is_yelling = is_only_contains_upper_case_letter and is_contains_any_uppercase_letter
    if is_yelling and is_question:
        return "Calm down, I know what I'm doing!"
    elif is_question:
        return "Sure."
    elif is_yelling:
        return "Whoa, chill out!"
    return "Whatever."
