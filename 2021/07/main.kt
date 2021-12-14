import java.io.File
import kotlin.math.abs

fun main() {
	val originalPositions = File("input/1.txt").readLines().single().split(',').map(String::toInt).toIntArray()
	
	val p1 = originalPositions.minOrNull() ?: return
	val p2 = originalPositions.maxOrNull() ?: return
	
	val cheapestFuel = (p1..p2).minOf { p ->
		originalPositions.sumOf { abs(it - p) }
	}
	
	println("Cheapest fuel: $cheapestFuel")
}
