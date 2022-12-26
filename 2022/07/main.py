from typing import Optional

from utils.input import read_input_lines


class Directory:
    def __init__(self, name: str, parent: Optional["Directory"]) -> None:
        self.name = name
        self.parent = parent if parent is not None else self
        self.child_directories = dict()
        self.child_files = dict()
        self.total_size = None
    
    def record_directory(self, name: str) -> None:
        if name not in self.child_directories:
            self.child_directories[name] = Directory(name, self)
            self.total_size = None
    
    def record_file(self, name: str, size: int) -> None:
        self.child_files[name] = size
        self.total_size = None
    
    def get_size(self) -> int:
        if self.total_size is None:
            self.total_size = sum(self.child_files.values()) + sum(child.get_size() for child in self.child_directories.values())
        
        return self.total_size
    
    def collect_directories(self, target_list: list["Directory"]) -> None:
        target_list.append(self)
        
        for child in self.child_directories.values():
            child.collect_directories(target_list)


class Session:
    def __init__(self) -> None:
        self.root_directory = Directory("/", None)
        self.current_directory = self.root_directory
    
    def handle_command(self, command: str) -> None:
        if command == "ls":
            return
        
        (left, right) = command.split(" ", maxsplit = 1)
        
        if left == "cd":
            if right == "/":
                self.current_directory = self.root_directory
            elif right == "..":
                self.current_directory = self.current_directory.parent
            else:
                self.current_directory = self.current_directory.child_directories[right]
    
    def handle_listing(self, size_or_type: str, name: str) -> None:
        if size_or_type == "dir":
            self.current_directory.record_directory(name)
        else:
            self.current_directory.record_file(name, int(size_or_type))


session = Session()

for line in read_input_lines():
    (left, right) = line.split(" ", maxsplit = 1)
    
    if left == "$":
        session.handle_command(right)
    else:
        session.handle_listing(left, right)

all_directories = list()
session.root_directory.collect_directories(all_directories)

total_size_of_interesting_directories = sum(directory.get_size() for directory in all_directories if directory.get_size() <= 100_000)
print(f"Total size of interesting directories: {total_size_of_interesting_directories}")

total_disk_size = 70_000_000
needed_free_disk_size = 30_000_000

current_free_disk_size = total_disk_size - session.root_directory.get_size()
minimum_folder_size_to_delete = needed_free_disk_size - current_free_disk_size

size_of_deleted_folder = min(folder.get_size() for folder in all_directories if folder.get_size() >= minimum_folder_size_to_delete)
print(f"Size of deleted folder: {size_of_deleted_folder}")
