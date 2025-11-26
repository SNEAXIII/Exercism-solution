from json import dumps, loads
from typing import Optional


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
        if abs(self._owes[other]) < 1e-10:
            del self._owes[other]

    def to_dict(self) -> dict:
        return {
            "name": self.name,
            "owes": self.owes,
            "owed_by": self.owed_by,
            "balance": self.balance,
        }

    def to_json(self) -> str:
        return dumps(self.to_dict())


class RestAPI:
    def __init__(self, database: Optional[dict] = None):
        self.users: list[User] = []
        if database:
            self._load_database(database)

    def _load_database(self, database: dict):
        for user_data in database.get("users", []):
            user = User(user_data["name"])
            for owed_by_user, amount in user_data.get("owed_by", {}).items():
                user.update_owes(owed_by_user, -amount)
            for owes_user, amount in user_data.get("owes", {}).items():
                user.update_owes(owes_user, amount)
            self.users.append(user)

    def _users_to_dict(self, users: list[User]) -> dict:
        return {"users": [user.to_dict() for user in sorted(users, key=lambda x: x.name)]}

    def _users_to_json(self, users: list[User]) -> str:
        return dumps(self._users_to_dict(users))

    def _error(self, message: str) -> str:
        return dumps({"error": message})

    def _find_user(self, name: str) -> Optional[User]:
        return next((user for user in self.users if user.name == name), None)

    def get(self, url: str, payload: Optional[str] = None) -> str:
        if url == "/users":
            return self._get_users(payload)
        return self._error("Unknown endpoint")

    def post(self, url: str, payload: Optional[str] = None) -> str:
        if url == "/add":
            return self._add_user(payload)
        if url == "/iou":
            return self._create_iou(payload)
        return self._error("Unknown endpoint")

    def _get_users(self, payload: Optional[str]) -> str:
        if not payload:
            return self._users_to_json(self.users)

        try:
            data = loads(payload)
            filter_names = data.get("users", [])
            if not filter_names:
                return self._users_to_json(self.users)

            filtered = [user for user in self.users if user.name in filter_names]
            return self._users_to_json(filtered)
        except Exception:
            return self._error("Invalid payload")

    def _add_user(self, payload: Optional[str]) -> str:
        if not payload:
            return self._error("Missing payload")

        try:
            data = loads(payload)
            username = data.get("user")

            if not username:
                return self._error("Missing user field")

            if self._find_user(username):
                return self._error("User already exists")

            user = User(username)
            self.users.append(user)
            return user.to_json()
        except Exception:
            return self._error("Invalid payload")

    def _create_iou(self, payload: Optional[str]) -> str:
        if not payload:
            return self._error("Missing payload")

        try:
            data = loads(payload)
            lender_name = data.get("lender")
            borrower_name = data.get("borrower")
            amount = data.get("amount")

            if not all([lender_name, borrower_name, amount is not None]):
                return self._error("Missing lender, borrower, or amount")

            lender = self._find_user(lender_name)
            borrower = self._find_user(borrower_name)

            if not lender or not borrower:
                return self._error("User not found")

            lender.update_owes(borrower_name, -amount)
            borrower.update_owes(lender_name, amount)

            return self._users_to_json([lender, borrower])
        except Exception:
            return self._error("Invalid payload")
