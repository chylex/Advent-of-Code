from enum import Enum

from utils.input import read_input_lines


class Move(Enum):
    ROCK = 1
    PAPER = 2
    SCISSORS = 3
    
    def __init__(self, points: int):
        self.points = points


class Outcome(Enum):
    LOSS = 0
    DRAW = 3
    WIN = 6
    
    def __init__(self, points: int):
        self.points = points


def play_game_from_line(line: str) -> int:
    (opponent_char, response_char) = line.split(" ", maxsplit = 2)
    
    match opponent_char:
        case "A": opponent = Move.ROCK
        case "B": opponent = Move.PAPER
        case "C": opponent = Move.SCISSORS
        case _: raise ValueError(f"Invalid opponent move: {opponent_char}")
    
    match response_char:
        case "X": response = Move.ROCK
        case "Y": response = Move.PAPER
        case "Z": response = Move.SCISSORS
        case _: raise ValueError(f"Invalid response move: {response_char}")
    
    outcome = play_game(response, opponent)
    return response.points + outcome.points


def play_game(me: Move, opponent: Move) -> Outcome:
    if me == opponent:
        return Outcome.DRAW
    elif me == Move.ROCK:
        return Outcome.WIN if opponent == Move.SCISSORS else Outcome.LOSS
    elif me == Move.PAPER:
        return Outcome.WIN if opponent == Move.ROCK else Outcome.LOSS
    elif me == Move.SCISSORS:
        return Outcome.WIN if opponent == Move.PAPER else Outcome.LOSS
    else:
        raise ValueError(f"Invalid move: {me}")


lines = read_input_lines()
score = sum(play_game_from_line(line) for line in lines)
print(f"Score: {score}")
