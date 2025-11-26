SYMBOLS: dict[str, str] = {"plus": "+", "minus": "-", "divided by": "//", "multiplied by": "*"}

def transform_expression(raw_expression: list[str]) -> int:
    value = ""
    for index, element in enumerate(raw_expression):
        # If the element is neither a symbol nor an int
        if not (element in SYMBOLS.values() or element.lstrip("-").isdigit()):
            raise ValueError("unknown operation")
        # I add a space to separate integer if no symbol is
        # between them,to produce an error with `eval("6 3")`
        value += f" {element}"
        # In the string called value, we sould have only 2 numbers and one symbol
        if index % 2 == 0:
            value = str(compute_value(value))
    return compute_value(value)

def compute_value(value: str) -> int:
    try:
        return eval(value)
    except SyntaxError:
        raise ValueError("syntax error")

def custom_strip(question: str) -> str:
    return question.lstrip("What is ").rstrip("?")

def replace_symbol(question: str) -> str:
    for text, symbol in SYMBOLS.items():
        question = question.replace(text, symbol)
    return question

def answer(question: str) -> int:
    parsed_expression: list[str] = replace_symbol(custom_strip(question)).split()
    return transform_expression(parsed_expression)
