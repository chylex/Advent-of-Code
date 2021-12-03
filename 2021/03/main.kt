import java.io.File

fun main() {
	val lines = File("input/1.txt").readLines().filter(String::isNotEmpty)
	
	val totalRows = lines.size
	val totalColumns = lines[0].length
	val totalOnesPerColumn = Array(totalColumns) { column ->
		lines.count { it[column] == '1' }
	}
	
	val gammaRate = totalOnesPerColumn
		.map { if (it > totalRows / 2) 1 else 0 }
		.foldIndexed(0) { index, total, bit -> total or (bit shl (totalColumns - index - 1)) }
	
	val epsilonRate = gammaRate xor ((1 shl totalColumns) - 1)
	
	println("Power consumption: ${gammaRate * epsilonRate}")
}
