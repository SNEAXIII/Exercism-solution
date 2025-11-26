from re import compile

VOWELS_CHARS = "aeiou"
NON_VOWELS = f"^[^{VOWELS_CHARS}]*"
ALL_CASE_PATTERN = f"((?!y){NON_VOWELS})y|({NON_VOWELS}qu)|({NON_VOWELS})"
ALL_CASE_COMPILE = compile(ALL_CASE_PATTERN)

def get_start_with(word: str) -> str:
    if (matches := ALL_CASE_COMPILE.findall(word)[0]):
        return next(match for match in matches if match)

def translate_a_word(word: str) -> str:
    if word[0] in VOWELS_CHARS or word[0:2] in ("xr","yt"):
        return f"{word}ay"
    if (to_move := get_start_with(word)):
        return f"{word.removeprefix(to_move)}{to_move}ay"

def translate(text: str) -> str:
    return " ".join(translate_a_word(word) for word in text.split(" "))