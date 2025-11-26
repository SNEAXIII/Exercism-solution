ACE = "A"
SPECIAL_CARD_CONVERTER = {"J": 10, "K": 10, "Q": 10, ACE: 1}


def value_of_card(card: str):
    if not isinstance(card, str):
        raise ValueError(f"The {card = } doesn't exists !")
    if (number := SPECIAL_CARD_CONVERTER.get(card,None)):
        return number
    if 2 <= (number := int(card)) <= 10:
        return number


def higher_card(card_1: str, card_2: str):
    if (value_1 := value_of_card(card_1)) == (value_2 := value_of_card(card_2)):
        return card_1, card_2
    if value_1 >= value_2:
        return card_1
    return card_2


def value_of_ace(card_1: str, card_2: str):
    return 1 if value_of_card(card_1)+value_of_card(card_2) > 10 or ACE in (card_1, card_2) else 11


def is_blackjack(card_1: str, card_2: str):
    is_contain_10_power_card = max(value_of_card(card_1), value_of_card(card_2)) == 10
    return is_contain_10_power_card and ACE in (card_1, card_2)


def can_split_pairs(card_1: str, card_2: str):
    return value_of_card(card_1) == value_of_card(card_2)


def can_double_down(card_1, card_2):
    return 9 <= value_of_card(card_1) + value_of_card(card_2) <= 11
