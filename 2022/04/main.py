from utils.input import read_input_lines


class Assignment:
    def __init__(self, first_section: int, last_section: int):
        self.first_section = first_section
        self.last_section = last_section
    
    def __repr__(self) -> str:
        return f"{self.first_section}-{self.last_section}"

    def contains(self, other: "Assignment") -> bool:
        return self.first_section <= other.first_section and self.last_section >= other.last_section
    
    def overlaps(self, other: "Assignment") -> bool:
        return self.first_section in range(other.first_section, other.last_section + 1) or self.last_section in range(other.first_section, other.last_section + 1)
    
    @staticmethod
    def from_str(line: str) -> "Assignment":
        (first_section, last_section) = line.split("-", maxsplit = 1)
        return Assignment(int(first_section), int(last_section))


class Pair:
    def __init__(self, first: Assignment, second: Assignment):
        self.first = first
        self.second = second
    
    def __repr__(self) -> str:
        return f"{self.first}, {self.second}"
    
    def has_full_overlap(self) -> bool:
        return self.first.contains(self.second) or self.second.contains(self.first)
    
    def has_any_overlap(self) -> bool:
        return self.first.overlaps(self.second) or self.second.overlaps(self.first)
    
    @staticmethod
    def from_line(line: str) -> "Pair":
        (first, second) = line.split(",", maxsplit = 1)
        return Pair(Assignment.from_str(first), Assignment.from_str(second))


lines = read_input_lines()
pairs = list(map(lambda line: Pair.from_line(line), lines))

assignments_with_full_overlap = sum(1 for pair in pairs if pair.has_full_overlap())
print(f"Amount of assignments with full overlap: {assignments_with_full_overlap}")

assignments_with_any_overlap = sum(1 for pair in pairs if pair.has_any_overlap())
print(f"Amount of assignments with any overlap: {assignments_with_any_overlap}")
