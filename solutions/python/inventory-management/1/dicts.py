def get_actual_count(inventory:dict, items_to_alter:dict, item:str, factor:int):
    return max((inventory.get(item, 0) + items_to_alter.get(item, 0) * factor),0)


def create_inventory(items : list):
    return {item:items.count(item) for item in set(items)}


def add_items(inventory, items):
    items_to_alter = create_inventory(items)
    all_items = set(inventory) | set(items_to_alter)
    return {item: get_actual_count(inventory, items_to_alter, item, 1) for item in all_items}


def decrement_items(inventory, items):
    return {item: get_actual_count(inventory, create_inventory(items), item, -1) for item in set(inventory)}


def remove_item(inventory, item):
    if item in inventory.keys():
        del inventory[item]
    return inventory


def list_inventory(inventory):
    return list((item, count) for item, count in inventory.items() if count > 0)

