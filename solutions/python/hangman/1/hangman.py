# Game status categories
# Change the values as you see fit
STATUS_WIN = 'win'
STATUS_LOSE = 'lose'
STATUS_ONGOING = 'ongoing'


class Hangman:
    def __init__(self, word):
        self.word_to_find = word
        self.masked_word = ["_"]*len(word)
        self.remaining_guesses = 9
        self.status = STATUS_ONGOING

    def guess(self, char):
        if self.status is not STATUS_ONGOING:
            raise ValueError("The game has already ended.")
        # We count the number of remaining char to find
        not_found = self.get_number_not_found()
        right_indexes = [index for index,__char in enumerate(self.word_to_find) if char == __char]
        for index in right_indexes:
            self.masked_word[index] = char
        # We compare the previous and the actual number of remaining caracters
        if not_found == self.get_number_not_found():
            self.remaining_guesses -=1
        if self.word_to_find == self.get_masked_word():
            self.status = STATUS_WIN
        elif self.remaining_guesses < 0:
            self.status = STATUS_LOSE

    def get_number_not_found(self):
        return self.get_masked_word().count("_")
    
    def get_masked_word(self):
        return "".join(self.masked_word)

    def get_status(self):
        return self.status