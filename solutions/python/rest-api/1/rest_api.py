from json import dumps, loads
from functools import wraps


def debug(func):
    """Décorateur pour afficher les inputs et outputs des fonctions"""

    @wraps(func)
    def wrapper(*args, **kwargs):
        # Afficher les inputs
        print(f"[DEBUG] Appel de {func.__name__}")
        print(f"[DEBUG] Args: {args}")
        print(f"[DEBUG] Kwargs: {kwargs}\n")

        # Exécuter la fonction
        result = func(*args, **kwargs)

        # Afficher l'output
        print(f"[DEBUG] Retour: {result}")
        print(f"[DEBUG] Fin de {func.__name__}\n\n")

        return result

    return wrapper


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

    @debug
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

    def borrow(self, payload):
        data = loads(payload)
        print("data in function borrow:")
        print(self.database)
        lender_name = data.get("lender")
        borrower_name = data.get("borrower")
        amount = data.get("amount")
        if not any((lender_name, borrower_name, amount)):
            raise ValueError("Missing lender, borrower, or amount in payload")
        lender = next(
            (user for user in self.database["users"] if user["name"] == lender_name),
            None,
        )
        borrower = next(
            (user for user in self.database["users"] if user["name"] == borrower_name),
            None,
        )
        lender["balance"] += amount
        borrower["balance"] -= amount
        if not any((lender, borrower)):
            raise ValueError("Lender or borrower not found in database")
        lender_owes = lender["owes"].get(borrower_name, 0.0)
        if lender_owes > 0.0:
            lender_owes_actual = min(lender_owes, amount)
            lender["owes"][borrower_name] -= lender_owes_actual
            borrower["owed_by"][lender_name] -= lender_owes_actual
            if lender["owes"][borrower_name] <= 0.0:
                del lender["owes"][borrower_name]
                del borrower["owed_by"][lender_name]
            amount -= lender_owes_actual
        amount_to_add =lender["owed_by"].get(borrower_name, 0.0) + amount
        if amount_to_add:
            lender["owed_by"][borrower_name] = amount_to_add
            borrower["owes"][lender_name] = amount_to_add

        return dumps({"users": sorted([lender, borrower], key=lambda x: x["name"])})
