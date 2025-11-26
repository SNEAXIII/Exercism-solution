from json import dumps, loads


class User:
    def __init__(self, name: str):
        self.name = name
        self._owes: dict[str, float] = {}

    @property
    def balance(self) -> float:
        return -sum(self._owes.values())

    @property
    def owes(self) -> dict[str, float]:
        return {user: amount for user, amount in self._owes.items() if amount > 0.0}

    @property
    def owed_by(self) -> dict[str, float]:
        return {user: -amount for user, amount in self._owes.items() if amount < 0.0}

    def update_owes(self, other: str, amount: float):
        self._owes[other] = self._owes.get(other, 0.0) + amount

    def self_to_dict(self) -> dict:
        return {
            "name": self.name,
            "owes": self.owes,
            "owed_by": self.owed_by,
            "balance": self.balance,
        }

    def self_to_str(self) -> str:
        return dumps(self.self_to_dict())


class RestAPI:
    def __init__(self, database: None | dict[str:str] = None):
        self.users = []
        if database:
            users = database.get("users", [])
            for users_data in users:
                user = User(users_data["name"])
                for owed_by_user, amount in users_data.get("owed_by", {}).items():
                    user.update_owes(owed_by_user, -amount)
                for owes_user, amount in users_data.get("owes", {}).items():
                    user.update_owes(owes_user, amount)
                self.users.append(user)

    @staticmethod
    def _to_dict(users: list[User]) -> dict:
        return {
            "users": [
                user.self_to_dict() for user in sorted(users, key=lambda x: x.name)
            ]
        }

    @staticmethod
    def render_unknown_endpoint() -> str:
        return dumps({"error": "Unknown endpoint"})

    def _to_json(self, users: list[User]) -> str:
        return dumps(self._to_dict(users))

    def database_to_json(self) -> str:
        return self._to_json(self.users)

    def get(self, url, payload=None):
        if url == "/users":
            return self.get_with_filter(payload)
        return self.render_unknown_endpoint()

    def post(self, url, payload=None):
        if url == "/add":
            return self.post_add(payload)
        if url == "/iou":
            return self.borrow(payload)
        return self.render_unknown_endpoint()

    def get_with_filter(self, payload):
        if not payload:
            return self._to_json(self.users)
        data = loads(payload)
        if "users" not in data:
            return self._to_json(self.users)
        filtered_users = [user for user in self.users if user.name in data["users"]]
        return self._to_json(filtered_users)

    def post_add(self, payload):
        data = loads(payload)
        if any(user.name == data["user"] for user in self.users):
            return dumps({"error": "User already exists"})
        user = User(data["user"])
        self.users.append(user)
        return user.self_to_str()

    def _find_user_by_name(self, name: str) -> User | None:
        for user in self.users:
            if user.name == name:
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
        lender.update_owes(borrower_name, -amount)
        borrower.update_owes(lender_name, amount)
        users_sorted = sorted([lender, borrower], key=lambda x: x.name)
        return self._to_json(users_sorted)
