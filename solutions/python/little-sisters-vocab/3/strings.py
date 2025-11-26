"""Functions for creating, transforming, and adding prefixes to strings."""


def add_prefix_un(word):
    return f"un{word}"


def make_word_groups(vocab_words):
    return f"{vocab_words[0]} :: "+" :: ".join(f"{vocab_words[0]}{word}" for word in vocab_words[1:])


def remove_suffix_ness(word):
    return (cutted_word:=word.removesuffix("ness"))[:-1]+cutted_word[-1].replace("i","y")


def adjective_to_verb(sentence, index):
    return sentence.split(" ")[index].removesuffix(".")+"en"
