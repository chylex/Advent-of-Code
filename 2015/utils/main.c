#include "file.h"

void entryPoint(char* input);

int main() {
	char* input = readFile("input/1.txt");
	
	if (input == NULL) {
		return 1;
	}
	else {
		entryPoint(input);
		return 0;
	}
}
