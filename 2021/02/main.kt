import java.io.File

fun main() {
	val lines = File("input/1.txt").readLines()
	val directions = lines.map { line -> line.split(' ', limit = 2).let { it[0] to it[1].toInt() } }
	
	var position = 0
	var depth = 0
	
	for ((direction, distance) in directions) {
		when (direction) {
			"forward" -> position += distance
			"up"      -> depth -= distance
			"down"    -> depth += distance
		}
	}
	
	println("Position: $position")
	println("Depth: $depth")
	println("Multiplied: ${position * depth}")
}
