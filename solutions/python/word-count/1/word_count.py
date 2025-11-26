from re import compile
word_extracter = compile("\w+(?:'?\w)*")
def count_words(sentence):
    words = word_extracter.findall(sentence.lower().replace("_"," "))
    return {key:words.count(key) for key in set(words)}