import java.io.File
import kotlin.math.abs

fun main() {
	val originalPositions = File("input/1.txt").readLines().single().split(',').map(String::toInt).toIntArray()
	
	val p1 = originalPositions.minOrNull() ?: return
	val p2 = originalPositions.maxOrNull() ?: return
	val candidates = p1..p2
	
	part1(originalPositions, candidates)
	part2(originalPositions, candidates)
}

fun part1(originalPositions: IntArray, candidates: IntRange) {
	val cheapestFuel = candidates.minOf { p ->
		originalPositions.sumOf { abs(it - p) }
	}
	
	println("Cheapest fuel at constant fuel usage: $cheapestFuel")
}

fun part2(originalPositions: IntArray, candidates: IntRange) {
	val cheapestFuel = candidates.minOf { p1 ->
		originalPositions.sumOf { p2 ->
			abs(p2 - p1).let { steps ->
				(steps * (steps + 1)) / 2
			}
		}
	}
	
	println("Cheapest fuel at polynomial fuel usage: $cheapestFuel")
}
