INVALID_RESULT = "\"%s\" is not a valid result, it should be in %s"
WIN = "win"
DRAW = "draw"
LOSS = "loss"
RESULT = {
    WIN: LOSS,
    DRAW: DRAW,
    LOSS: WIN
}


class Team:
    def __init__(self, name: str):
        self.name = name
        self.wins = 0
        self.draws = 0
        self.losses = 0

    def get_match_played(self) -> int:
        return sum((self.wins, self.draws, self.losses))

    def get_points(self) -> int:
        return self.wins*3 + self.draws

    def update_result(self, result: str) -> None:
        if result == WIN:
            self.wins += 1
        elif result == DRAW:
            self.draws += 1
        elif result == LOSS:
            self.losses += 1


class Teams:
    def __init__(self, rows: str):
        self.rows = rows
        self.teams: dict[Team] = {}

    def get_formated_row(self, team, match_played, win, draw, loss, points) -> str:
        return f"{team:<30} | {match_played:>2} | {win:>2} | {draw:>2} | {loss:>2} | {points:>2}"

    def get_header(self) -> str:
        return self.get_formated_row("Team", "MP", "W", "D", "L", "P")

    def get_team_result(self, team: Team) -> str:
        return self.get_formated_row(team.name, team.get_match_played(), team.wins, team.draws, team.losses, team.get_points())

    def process_rows(self):
        for row in self.rows:
            team1, team2, result = row.split(";")
            if result not in RESULT:
                raise ValueError(INVALID_RESULT % (result, RESULT.keys()))
            if team1 not in self.teams:
                self.teams[team1] = Team(team1)
            if team2 not in self.teams:
                self.teams[team2] = Team(team2)

            self.teams[team1].update_result(result)
            self.teams[team2].update_result(RESULT[result])

    def get_result_table(self) -> list:
        sorted_teams = sorted(
            self.teams.values(),
            key=lambda team: (-team.get_points(), team.name)
        )
        to_return = [self.get_team_result(team) for team in sorted_teams]
        to_return.insert(0, self.get_header())
        return to_return


def tally(rows: list[str]):
    teams = Teams(rows)
    teams.process_rows()
    return teams.get_result_table()
