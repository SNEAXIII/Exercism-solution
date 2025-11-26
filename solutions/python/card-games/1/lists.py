

def get_rounds(number):
    return list(range(number,number+3))

def concatenate_rounds(rounds_1, rounds_2):
    return rounds_1 + rounds_2

def list_contains_round(rounds, number):
    return number in rounds
    
def card_average(hand):
    return sum(hand) / len(hand)


def approx_average_is_average(hand):
    return card_average(hand) in (hand[len(hand)//2], (hand[0]+hand[-1]) / 2)

def average_even_is_average_odd(hand):
    even = {hand[number] for number in range(0, len(hand), 2)}
    odd = {hand[number] for number in range(1, len(hand), 2)}
    return (sum(even) / len(even)) == (sum(odd) / len(odd))

def maybe_double_last(hand):
    if hand[-1] == 11:
        hand[-1]*=2
    return hand