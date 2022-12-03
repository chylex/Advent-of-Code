import java.io.File

fun main() {
	val lines = File("input.txt").readLines().filter(String::isNotEmpty)
	val input = Input(lines)
	part1(input)
	part2(input)
}

class Input(val lines: List<String>) {
	val totalRows = lines.size
	val totalColumns = lines[0].length
}

private fun part1(input: Input) = with(input) {
	val totalOnesPerColumn = Array(totalColumns) { column ->
		lines.count { it[column] == '1' }
	}
	
	val gammaRate = totalOnesPerColumn
		.map { if (it > totalRows - it) 1 else 0 }
		.foldIndexed(0) { index, total, bit -> total or (bit shl (totalColumns - index - 1)) }
	
	val epsilonRate = gammaRate xor ((1 shl totalColumns) - 1)
	
	println("Power consumption: ${gammaRate * epsilonRate}")
}

private fun part2(input: Input) = with(input) {
	
	fun findValueByCriteria(keepMostCommon: Boolean): Int {
		val keptLines = lines.toMutableList()
		
		for (column in 0 until totalColumns) {
			val ones = keptLines.count { it[column] == '1' }
			val zeros = keptLines.size - ones
			
			val keptValue = if (keepMostCommon) {
				if (ones >= zeros) '1' else '0'
			}
			else {
				if (zeros <= ones) '0' else '1'
			}
			
			keptLines.removeAll { it[column] != keptValue }
			
			if (keptLines.size == 1) {
				return keptLines.single().toInt(radix = 2)
			}
		}
		
		throw IllegalStateException()
	}
	
	val oxygenGeneratorRating = findValueByCriteria(keepMostCommon = true)
	val co2ScrubberRating = findValueByCriteria(keepMostCommon = false)
	
	println("Life support rating: ${oxygenGeneratorRating * co2ScrubberRating}")
}
