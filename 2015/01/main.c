#include "stdio.h"

#define BUFFER_SIZE 256

int walkFloors(int currentFloor, int* totalInstructionCounter, int* firstEnteredBasement, char* instructions, size_t instructionCount);

int main() {
	FILE* file;
	errno_t openErr = fopen_s(&file, "input/1.txt", "rb");
	if (openErr != 0 || !file) {
		printf("Error opening input file, code %d\n", openErr);
		return 1;
	}
	
	int floor = 0;
	int totalInstructionCounter = 0;
	int firstEnteredBasement = -1;
	
	char buffer[BUFFER_SIZE];
	while (!feof(file)) {
		size_t readBytes = fread_s(&buffer, BUFFER_SIZE, 1, BUFFER_SIZE, file);
		
		int readErr = ferror(file);
		if (readErr) {
			printf("Error reading input file, code %d\n", readErr);
			return 1;
		}
		
		floor = walkFloors(floor, &totalInstructionCounter, &firstEnteredBasement, buffer, readBytes);
	}
	
	fclose(file);
	printf("Final floor: %d\n", floor);
	printf("First entered basement: %d\n", firstEnteredBasement);
	return 0;
}
