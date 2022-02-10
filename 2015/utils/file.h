#ifndef ADVENTOFCODE_FILE_H
#define ADVENTOFCODE_FILE_H

#include "stdio.h"
#include "stdlib.h"

#define MAX_READ_SIZE 4096

char* readFile(const char* filename) {
	FILE* file;
	errno_t openErr = fopen_s(&file, filename, "rb");
	if (openErr != 0 || !file) {
		printf("Error opening input file, code %d\n", openErr);
		return NULL;
	}
	
	fseek(file, 0L, SEEK_END);
	int fileSize = ftell(file);
	rewind(file);
	
	if (fileSize == 0) {
		printf("Input file is empty\n");
		fclose(file);
		return NULL;
	}
	
	char* contents = malloc(fileSize + 1);
	if (!contents) {
		printf("Error allocating %d bytes for storing input file\n", fileSize + 1);
		fclose(file);
		return NULL;
	}
	
	size_t position = 0;
	while (position < fileSize) {
		size_t remainingSpace = fileSize - position;
		size_t readBytes = fread_s(contents + position, remainingSpace, 1, min(remainingSpace, MAX_READ_SIZE), file);
		position += readBytes;
		
		int readErr = ferror(file);
		if (readErr) {
			perror("Error reading input file");
			fclose(file);
			free(contents);
			return NULL;
		}
		
		if (readBytes == 0) {
			printf("Error reading input file, read 0 bytes\n");
			fclose(file);
			free(contents);
			return NULL;
		}
	}
	
	fclose(file);
	
	if (position != fileSize) {
		printf("Error reading whole file, read only %d bytes out of %d\n", position, fileSize);
		free(contents);
		return NULL;
	}
	
	contents[fileSize] = 0;
	return contents;
}

#endif //ADVENTOFCODE_FILE_H
