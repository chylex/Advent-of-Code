import java.io.File

fun main() {
	val records = File("input/1.txt").readLines().map { line ->
		line.split(" | ", limit = 2).map { it.split(' ')  }.let { Record(it[0], it[1]) }
	}
	
	val simpleDigitCount = records.sumOf { r -> r.output.count { it.length in UNIQUE_SEGMENT_COUNT_TO_DIGITS } }
	println("Total appearances of 1,4,7,8: $simpleDigitCount")
}

private data class Record(val patterns: List<String>, val output: List<String>)

private val DIGIT_TO_SEGMENT_COUNT = mapOf(
	0 to 6,
	1 to 2,
	2 to 5,
	3 to 5,
	4 to 4,
	5 to 5,
	6 to 6,
	7 to 3,
	8 to 7,
	9 to 6,
)

@Suppress("ConvertLambdaToReference")
private val SEGMENT_COUNT_TO_DIGITS = DIGIT_TO_SEGMENT_COUNT
	.entries
	.groupBy { it.value }
	.mapValues { it.value.map { e -> e.key } }

private val UNIQUE_SEGMENT_COUNT_TO_DIGITS = SEGMENT_COUNT_TO_DIGITS
	.filterValues { it.size == 1 }
	.mapValues { it.value.single() }
