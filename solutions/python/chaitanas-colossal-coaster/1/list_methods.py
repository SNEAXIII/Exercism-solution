from copy import deepcopy

def add_me_to_the_queue(express_queue, normal_queue, ticket_type, person_name):
    match ticket_type:
        case 0:
            selected_queue = normal_queue
        case 1: 
            selected_queue = express_queue
        case _:
            raise ValueError(f"The ticket type {ticket_type} doesn't exists")
    selected_queue.append(person_name)
    return selected_queue

def find_my_friend(queue, friend_name):
    return queue.index(friend_name)

def add_me_with_my_friends(queue, index, person_name):
    queue.insert(index, person_name)
    return queue

def remove_the_mean_person(queue, person_name):
    queue.remove(person_name)
    return queue

def how_many_namefellows(queue, person_name):
    return queue.count(person_name)

def remove_the_last_person(queue):
    return queue.pop()

def sorted_names(queue):
    copy = deepcopy(queue)
    copy.sort()
    return copy