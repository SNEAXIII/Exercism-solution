from itertools import product
from json import dumps, loads


class RestAPI:
    def __init__(self, database: None | dict[str:str] = None):
        self.database = database if database else {"users": []}

    def get(self, url, payload=None):
        if url == "/users":
            return self.get_with_filter(payload)
        return dumps({"error": "Unknown endpoint"})

    def post(self, url, payload=None):
        if url == "/add":
            return self.post_add(payload)
        if url == "/iou":
            return self.borrow(payload)
        return dumps({"error": "Unknown endpoint"})

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
        if any(user["name"] == data["user"] for user in self.database["users"]):
            return dumps({"error": "User already exists"})
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
            to_delete = []
            for user_name, amount in user[key].items():
                if amount <= 0.0:
                    to_delete.append(user_name)
            for user_name in to_delete:
                del user[key][user_name]

    def _find_user_by_name(self, name: str) -> dict | None:
        for user in self.database["users"]:
            if user["name"] == name:
                return user
        return None

    def borrow(self, payload):
        data = loads(payload)
        lender_name = data.get("lender")
        borrower_name = data.get("borrower")
        amount = data.get("amount")
        if not all((lender_name, borrower_name, amount)):
            raise ValueError("Missing lender, borrower, or amount in payload")
        lender = self._find_user_by_name(lender_name)
        borrower = self._find_user_by_name(borrower_name)
        if not lender or not borrower:
            raise ValueError("Lender or borrower not found in database")
        lender["balance"] += amount
        borrower["balance"] -= amount
        lender_owes = lender["owes"].get(borrower_name, 0.0)
        if lender_owes > 0.0:
            lender_owes_actual = min(lender_owes, amount)
            lender["owes"][borrower_name] -= lender_owes_actual
            borrower["owed_by"][lender_name] -= lender_owes_actual
            amount -= lender_owes_actual
        if amount > 0:
            amount_to_add = lender["owed_by"].get(borrower_name, 0.0) + amount
            lender["owed_by"][borrower_name] = amount_to_add
            borrower["owes"][lender_name] = amount_to_add
        self.clean_up_zeros_users([lender, borrower])
        return dumps({"users": sorted([lender, borrower], key=lambda x: x["name"])})
