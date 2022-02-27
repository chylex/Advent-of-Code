import Position.BOTTOM
import Position.BOTTOM_LEFT
import Position.BOTTOM_RIGHT
import Position.MIDDLE
import Position.TOP
import Position.TOP_LEFT
import Position.TOP_RIGHT
import java.io.File
import java.util.EnumSet

fun main() {
	val records = File("input/1.txt").readLines().map { line ->
		line.split(" | ", limit = 2).map { it.split(' ') }.let { Record(it[0], it[1]) }
	}
	
	part1(records)
	part2(records)
}

fun part1(records: List<Record>) {
	@Suppress("ConvertLambdaToReference")
	val segmentCountToDigits = DIGITS
		.map { it.value to it.positions.size }
		.groupBy { it.second }
		.mapValues { it.value.map { e -> e.first } }
	
	val uniqueSegmentCountToDigits = segmentCountToDigits
		.filterValues { it.size == 1 }
		.mapValues { it.value.single() }
	
	val simpleDigitCount = records.sumOf { r -> r.output.count { it.length in uniqueSegmentCountToDigits } }
	println("Total appearances of 1,4,7,8: $simpleDigitCount")
}

fun part2(records: List<Record>) {
	println("Total output: ${records.sumOf(Record::deduceOutput)}")
}

data class Digit(val value: Int, val positions: EnumSet<Position>)

enum class Position {
	TOP,
	TOP_LEFT,
	TOP_RIGHT,
	MIDDLE,
	BOTTOM_LEFT,
	BOTTOM_RIGHT,
	BOTTOM
}

private val DIGITS = listOf(
	Digit(0, EnumSet.of(TOP, TOP_LEFT, TOP_RIGHT, BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM)),
	Digit(1, EnumSet.of(TOP_RIGHT, BOTTOM_RIGHT)),
	Digit(2, EnumSet.of(TOP, TOP_RIGHT, MIDDLE, BOTTOM_LEFT, BOTTOM)),
	Digit(3, EnumSet.of(TOP, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT, BOTTOM)),
	Digit(4, EnumSet.of(TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT)),
	Digit(5, EnumSet.of(TOP, TOP_LEFT, MIDDLE, BOTTOM_RIGHT, BOTTOM)),
	Digit(6, EnumSet.of(TOP, TOP_LEFT, MIDDLE, BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM)),
	Digit(7, EnumSet.of(TOP, TOP_RIGHT, BOTTOM_RIGHT)),
	Digit(8, EnumSet.of(TOP, TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_LEFT, BOTTOM_RIGHT, BOTTOM)),
	Digit(9, EnumSet.of(TOP, TOP_LEFT, TOP_RIGHT, MIDDLE, BOTTOM_RIGHT, BOTTOM)),
)

data class Record(val patterns: List<String>, val output: List<String>) {
	private fun deduceMapping(): Map<Char, Position> {
		val mapping = mutableMapOf<Char, Position>()
		
		// The digit 1 is made of the top right and bottom right segment.
		val patternFor1 = patterns.first { it.length == 2 }
		
		// We can deduce the mapping for both right segments by counting how many times each segment
		// appears among all of the digits (top right appears in 8 digits, bottom right in 9 digits).
		if (patterns.count { patternFor1[0] in it } == 8) {
			mapping[patternFor1[0]] = TOP_RIGHT
			mapping[patternFor1[1]] = BOTTOM_RIGHT
		}
		else {
			mapping[patternFor1[1]] = TOP_RIGHT
			mapping[patternFor1[0]] = BOTTOM_RIGHT
		}
		
		// The digit 7 is made of both right segments, and the top segment.
		val patternFor7 = patterns.first { it.length == 3 }
		
		// We can deduce the mapping for the top segment,
		// since it is the only segment which is not shared with the digit 1.
		mapping[patternFor7.first { it !in patternFor1 }] = TOP
		
		// The digit 4 is made of both right segments, the top left segment, and the middle segment.
		// Both right segments are already deduced, so we only want the remaining two segments.
		val patternFor4MinusDeduced = patterns.first { it.length == 4 }.toSet().minus(patternFor1.toSet()).toTypedArray()
		
		// We can deduce the mapping for the top left and middle segments by counting how many times each segment
		// appears among all of the digits (top left appears in 6 digits, middle in 7 digits).
		if (patterns.count { patternFor4MinusDeduced[0] in it } == 6) {
			mapping[patternFor4MinusDeduced[0]] = TOP_LEFT
			mapping[patternFor4MinusDeduced[1]] = MIDDLE
		}
		else {
			mapping[patternFor4MinusDeduced[1]] = TOP_LEFT
			mapping[patternFor4MinusDeduced[0]] = MIDDLE
		}
		
		// The digit 8 uses all seven segments, so we use it and remove the five segments we already know,
		// keeping the remaining two segments (bottom left, and bottom).
		val remainingSegments = patterns.first { it.length == 7 }.toSet().minus(mapping.keys.toSet()).toTypedArray()
		
		// We can deduce the mapping for the bottom left and bottom segments by counting how many times each segment
		// appears among all of the digits (bottom left appears in 4 digits, bottom in 7 digits).
		if (patterns.count { remainingSegments[0] in it } == 4) {
			mapping[remainingSegments[0]] = BOTTOM_LEFT
			mapping[remainingSegments[1]] = BOTTOM
		}
		else {
			mapping[remainingSegments[1]] = BOTTOM_LEFT
			mapping[remainingSegments[0]] = BOTTOM
		}
		
		return mapping
	}
	
	fun deduceOutput(): Int {
		val mapping = deduceMapping()
		
		return output.fold(0) { total, digit ->
			val positions = digit.map(mapping::getValue).toSet()
			val value = DIGITS.first { it.positions == positions }.value
			
			(total * 10) + value
		}
	}
}
