import java.io.File
import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min
import kotlin.system.measureTimeMillis

fun main() {
	val lineRegex = Regex("^(\\d+),(\\d+) -> (\\d+),(\\d+)$")
	val lines = File("input/1.txt").readLines()
		.mapNotNull(lineRegex::matchEntire)
		.map { it.groupValues.takeLast(4).map(String::toInt) }
		.map { Line(it[0], it[1], it[2], it[3]) }
	
	println("(Took ${measureTimeMillis { part1(lines) }} ms)")
	println("(Took ${measureTimeMillis { part2(lines) }} ms)")
}

@Suppress("ProtectedInFinal", "MemberVisibilityCanBePrivate")
data class Line(val x1: Int, val y1: Int, val x2: Int, val y2: Int) {
	val minX = min(x1, x2)
	val minY = min(y1, y2)
	val maxX = max(x1, x2)
	val maxY = max(y1, y2)
	
	init {
		require(isStraight || is45Degrees) { "Line must be straight or have a slope of 45 degrees!" }
	}
	
	val isStraight
		get() = x1 == x2 || y1 == y2
	
	val is45Degrees
		get() = abs(x2 - x1) == abs(y2 - y1)
	
	val length = 1 + max(abs(x2 - x1), abs(y2 - y1))
	
	private val slopeX = Integer.signum(x2 - x1)
	private val slopeY = Integer.signum(y2 - y1)
	
	fun contains(x: Int, y: Int): Boolean {
		if (x1 == x2) {
			return x == x1 && y in minY..maxY
		}
		
		if (y1 == y2) {
			return y == y1 && x in minX..maxX
		}
		
		if (x !in minX..maxX || y !in minY..maxY) {
			return false
		}
		
		for (i in 0 until length) {
			if (x == x1 + (slopeX * i) && y == y1 + (slopeY * i)) {
				return true
			}
		}
		
		return false
	}
}

class Floor(private val lines: List<Line>) {
	private val minX = lines.minOf { min(it.x1, it.x2) }
	private val maxX = lines.maxOf { max(it.x1, it.x2) }
	private val minY = lines.minOf { min(it.y1, it.y2) }
	private val maxY = lines.maxOf { max(it.y1, it.y2) }
	
	fun countPointsWithOverlap(minOverlap: Int): Int {
		val xs = minX..maxY
		val ys = minY..maxY
		val xy = xs.flatMap { x -> ys.map { y -> x to y } }
		return xy.parallelStream().filter { (x, y) -> hasOverlapOf(x, y, minOverlap) }.count().toInt()
	}
	
	private fun hasOverlapOf(x: Int, y: Int, minOverlap: Int): Boolean {
		var count = 0
		
		for (line in lines) {
			if (line.contains(x, y) && ++count >= minOverlap) {
				return true
			}
		}
		
		return false
	}
	
	@Suppress("unused")
	fun printDiagram() {
		for (y in minY..maxY) {
			for (x in minX..maxX) {
				val count = lines.count { it.contains(x, y) }
				if (count == 0) {
					print('.')
				}
				else {
					print(count)
				}
			}
			
			println()
		}
	}
}

fun part1(lines: List<Line>) {
	val straightLines = lines.filter(Line::isStraight).ifEmpty { return }
	val straightLineFloor = Floor(straightLines)
	println("Points where at least 2 straight lines overlap: ${straightLineFloor.countPointsWithOverlap(2)}")
}

fun part2(lines: List<Line>) {
	val floor = Floor(lines)
	println("Points where at least 2 lines overlap: ${floor.countPointsWithOverlap(2)}")
}
