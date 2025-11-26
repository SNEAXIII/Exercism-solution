from re import compile

VOWELS_CHARS = "aeiou"
NON_VOWELS = f"^[^{VOWELS_CHARS}]*"
ALL_CASE_PATTERN = f"((?!y){NON_VOWELS})y|({NON_VOWELS}qu)|({NON_VOWELS})"
ALL_CASE_COMPILE = compile(ALL_CASE_PATTERN)

def is_letter_vowel(char: str) -> bool:
    return char in VOWELS_CHARS

def is_start_with_vowel(word: str) -> bool:
    return is_letter_vowel(word[0])

def get_start_with(word: str) -> str:
    if (matches := ALL_CASE_COMPILE.findall(word)[0]):
        return next(match for match in matches if match)

def translate_a_word(word):
    if is_start_with_vowel(word) or word[0:2] in ("xr","yt"):
        return f"{word}ay"
    if (to_move := get_start_with(word)):
        return f"{word.removeprefix(to_move)}{to_move}ay"

def translate(text):
    return " ".join(translate_a_word(word) for word in text.split(" "))