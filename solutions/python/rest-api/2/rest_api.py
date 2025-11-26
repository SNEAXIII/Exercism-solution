from itertools import product
from json import dumps, loads


class RestAPI:
    def __init__(self, database: None | dict[str:str] = None):
        self.database = database
        if not self.database:
            self.database = {"users": []}
        for elem in self.database["users"]:
            print(
                f"{elem['name']}:\n    owes: {elem['owes']}\n    owed_by: {elem['owed_by']}\n    balance: {elem['balance']}"
            )

    def get(self, url, payload=None):
        if url == "/users":
            return self.get_with_filter(payload)

    def post(self, url, payload=None):
        if url == "/add":
            return self.post_add(payload)
        if url == "/iou":
            return self.borrow(payload)

    def get_with_filter(self, payload):
        if not payload:
            return dumps(self.database)
        data = loads(payload)
        if "users" not in data:
            return dumps(self.database)
        filtered_users = [
            user for user in self.database["users"] if user["name"] in data["users"]
        ]
        return dumps({"users": filtered_users})

    def post_add(self, payload):
        data = loads(payload)
        user = {
            "name": data["user"],
            "owes": {},
            "owed_by": {},
            "balance": 0.0,
        }
        self.database["users"].append(user)
        return dumps(user)

    @staticmethod
    def clean_up_zeros_users(users: list):
        for key, user in product(["owes", "owed_by"], users):
            to_delete = set()
            for user_name in user[key].keys():
                if user[key][user_name] <= 0.0:
                    to_delete.add((key, user_name))
            for item in to_delete:
                del user[item[0]][item[1]]

    def borrow(self, payload):
        data = loads(payload)

        lender, lender_name = None, data.get("lender")
        borrower, borrower_name = None, data.get("borrower")
        amount = data.get("amount")
        if not any((lender_name, borrower_name, amount)):
            raise ValueError("Missing lender, borrower, or amount in payload")
        for user in self.database["users"]:
            if all((lender, borrower)):
                break
            if user["name"] == lender_name:
                lender = user
            if user["name"] == borrower_name:
                borrower = user

        if not any((lender, borrower)):
            raise ValueError("Lender or borrower not found in database")
        lender["balance"] += amount
        borrower["balance"] -= amount
        lender_owes = lender["owes"].get(borrower_name, 0.0)
        if lender_owes > 0.0:
            lender_owes_actual = min(lender_owes, amount)
            lender["owes"][borrower_name] -= lender_owes_actual
            borrower["owed_by"][lender_name] -= lender_owes_actual
            amount -= lender_owes_actual
        amount_to_add = lender["owed_by"].get(borrower_name, 0.0) + amount
        if amount_to_add:
            lender["owed_by"][borrower_name] = amount_to_add
            borrower["owes"][lender_name] = amount_to_add
        self.clean_up_zeros_users([lender, borrower])
        return dumps({"users": sorted([lender, borrower], key=lambda x: x["name"])})
