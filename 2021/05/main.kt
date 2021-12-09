import java.io.File
import kotlin.math.max
import kotlin.math.min

fun main() {
	val lineRegex = Regex("^(\\d+),(\\d+) -> (\\d+),(\\d+)$")
	val lines = File("input/1.txt").readLines()
		.mapNotNull(lineRegex::matchEntire)
		.map { it.groupValues.takeLast(4).map(String::toInt) }
		.map { Line.of(it[0], it[1], it[2], it[3]) }
	
	val straightLines = lines.filter(Line::isStraight)
	val straightLineFloor = Floor(straightLines)
	
	println("Points where at least 2 straight lines overlap: ${straightLineFloor.countPointsWithOverlap(2)}")
}

@Suppress("ProtectedInFinal")
data class Line protected constructor(val x1: Int, val y1: Int, val x2: Int, val y2: Int) {
	companion object {
		fun of(x1: Int, y1: Int, x2: Int, y2: Int): Line {
			return Line(min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2))
		}
	}
	
	val isStraight
		get() = x1 == x2 || y1 == y2
	
	fun contains(x: Int, y: Int): Boolean {
		return if (x1 == x2)
			x == x1 && y in y1..y2
		else if (y1 == y2)
			y == y1 && x in x1..x2
		else
			throw UnsupportedOperationException()
	}
}

class Floor(private val lines: List<Line>) {
	private val minX = lines.minOf { min(it.x1, it.x2) }
	private val maxX = lines.maxOf { max(it.x1, it.x2) }
	private val minY = lines.minOf { min(it.y1, it.y2) }
	private val maxY = lines.maxOf { max(it.y1, it.y2) }
	
	fun countPointsWithOverlap(minOverlap: Int): Int {
		val xs = minX..maxX
		val ys = minY..maxY
		return xs.sumOf { x ->
			ys.count { y ->
				lines.count { it.contains(x, y) } >= minOverlap
			}
		}
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
