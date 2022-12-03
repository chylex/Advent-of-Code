import java.io.File

fun main() {
	val lines = File("input.txt").readLines()
	val directions = lines.map { line -> line.split(' ', limit = 2).let { it[0] to it[1].toInt() } }
	
	println("Part 1:")
	part1(directions)
	
	println()
	
	println("Part 2:")
	part2(directions)
}

private fun part1(directions: List<Pair<String, Int>>) {
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

private fun part2(directions: List<Pair<String, Int>>) {
	var position = 0
	var depth = 0
	var aim = 0
	
	for ((direction, distance) in directions) {
		when (direction) {
			"forward" -> {
				position += distance
				depth += aim * distance
			}
			"up"      -> aim -= distance
			"down"    -> aim += distance
		}
	}
	
	println("Position: $position")
	println("Depth: $depth")
	println("Multiplied: ${position * depth}")
}
