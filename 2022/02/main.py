from enum import Enum
from typing import Callable

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


def play_game_from_line(line: str, strategy: Callable[[Move, str], Move]) -> int:
    (opponent_char, response_char) = line.split(" ", maxsplit = 2)
    
    match opponent_char:
        case "A": opponent = Move.ROCK
        case "B": opponent = Move.PAPER
        case "C": opponent = Move.SCISSORS
        case _: raise ValueError(f"Invalid opponent move: {opponent_char}")
    
    response = strategy(opponent, response_char)
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


# noinspection PyShadowingNames
def play_games_from_all_lines(lines: list[str], strategy) -> int:
    return sum(play_game_from_line(line, strategy) for line in lines)


def strategy_response_char_determines_move(_: Move, response_char: str) -> Move:
    match response_char:
        case "X": return Move.ROCK
        case "Y": return Move.PAPER
        case "Z": return Move.SCISSORS
        case _: raise ValueError(f"Invalid response move: {response_char}")


def strategy_response_char_determines_outcome(opponent: Move, response_char: str) -> Move:
    match response_char:
        case "Y":  # Draw
            return opponent
        case "Z":  # Win
            match opponent:
                case Move.ROCK: return Move.PAPER
                case Move.PAPER: return Move.SCISSORS
                case Move.SCISSORS: return Move.ROCK
        case "X":  # Lose
            match opponent:
                case Move.ROCK: return Move.SCISSORS
                case Move.PAPER: return Move.ROCK
                case Move.SCISSORS: return Move.PAPER
        case _:
            raise ValueError(f"Invalid response move: {response_char}")


lines = read_input_lines()
first_strategy_score = play_games_from_all_lines(lines, strategy_response_char_determines_move)
second_strategy_score = play_games_from_all_lines(lines, strategy_response_char_determines_outcome)

print(f"First strategy score: {first_strategy_score}")
print(f"Second strategy score: {second_strategy_score}")
