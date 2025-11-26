# Globals for the directions
# Change the values as you see fit
EAST = "EAST"
NORTH = "NORTH"
WEST = "WEST"
SOUTH = "SOUTH"
directions = [
    (EAST, lambda coo: (coo[0]+1, coo[1])),
    (SOUTH, lambda coo: (coo[0], coo[1]-1)),
    (WEST, lambda coo: (coo[0]-1, coo[1])),
    (NORTH, lambda coo: (coo[0], coo[1]+1))
]
sides = {"L": -1, "R": 1}


class Robot:
    def __init__(self, direction=NORTH, x_pos=0, y_pos=0):
        self.coordinates = (x_pos, y_pos)
        self.direction = direction
        self.i_direction = next(index for index, (direction, f) in enumerate(directions) if self.direction == direction)

    def move(self, actions):
        for action in actions:
            if action == "A":
                self.coordinates = directions[self.i_direction][1](self.coordinates)
            if action in sides:
                self.i_direction = (self.i_direction + sides[action]) % len(directions)
                self.direction = directions[self.i_direction][0]
