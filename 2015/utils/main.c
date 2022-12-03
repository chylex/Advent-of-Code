#include "stdarg.h"
#include "file.h"
#include "benchmark.h"

void entryPoint(char *input);

int enableOutput = 1;

extern void print(const char *format, ...) {
	if (enableOutput) {
		va_list args;
		va_start(args, format);
		vprintf(format, args);
		va_end(args);
	}
}

int main(void) {
	char *input = readFile("input.txt");
	
	if (input == NULL) {
		return 1;
	}
	else {
		entryPoint(input);
		enableOutput = 0;
		runBenchmark(entryPoint, input);
		return 0;
	}
}
