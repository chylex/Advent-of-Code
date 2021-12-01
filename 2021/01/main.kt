import java.io.File

fun main() {
	val lines = File("input/1.txt").readLines().map(String::toInt)
	val totalIncreases = lines.windowed(2).count { it[1] > it[0] }
	println("Total Increases: $totalIncreases")
}
