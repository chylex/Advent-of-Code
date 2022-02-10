#ifndef ADVENTOFCODE_BENCHMARK_H
#define ADVENTOFCODE_BENCHMARK_H

#include "stdio.h"

#if WIN32
#include "windows.h"
#else
#include "time.h"
#endif

static int sortDoubles(const void* a, const void* b) {
	double x = *(double*) a;
	double y = *(double*) b;
	
	if (x > y) {
		return 1;
	}
	else if (x < y) {
		return -1;
	}
	else {
		return 0;
	}
}

static void formatMillis(const char* text, const double ms) {
	if (ms >= 0.1) {
		printf("\n%s: %.2f ms", text, ms);
	}
	else {
		printf("\n%s: %.2f us", text, ms * 1000.0);
	}
}

typedef void (*entryPointCallback)(char* input);

#define BENCHMARK_RUNS 50001

void runBenchmark(const entryPointCallback ep, char* input) {
	double benchmarkResults[BENCHMARK_RUNS];
	for (int run = 0; run < BENCHMARK_RUNS; run++) {
		#if WIN32
		LARGE_INTEGER startTime, endTime, frequency;
		QueryPerformanceFrequency(&frequency);
		QueryPerformanceCounter(&startTime);
		ep(input);
		QueryPerformanceCounter(&endTime);
		benchmarkResults[run] = (double) (endTime.QuadPart - startTime.QuadPart) * 1000.0 / (double) frequency.QuadPart;
		#else
		clock_t startTime = clock();
		ep(input);
		clock_t endTime = clock();
		benchmarkResults[run] = ((endTime - startTime) * 1000.0) / CLOCKS_PER_SEC;
		#endif
	}
	
	qsort(benchmarkResults, BENCHMARK_RUNS, sizeof(double), sortDoubles);
	
	formatMillis("Median elapsed time", benchmarkResults[BENCHMARK_RUNS / 2]);
	formatMillis("Q1 elapsed time", benchmarkResults[(BENCHMARK_RUNS * 25) / 100]);
	formatMillis("Q3 elapsed time", benchmarkResults[(BENCHMARK_RUNS * 75) / 100]);
}

#undef BENCHMARK_RUNS

#endif //ADVENTOFCODE_BENCHMARK_H
