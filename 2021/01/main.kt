import java.io.File

fun main() {
	val lines = File("input.txt").readLines().map(String::toInt)
	
	val totalIncreases = lines.windowed(2).count { it[1] > it[0] }
	val windowedIncreases = lines.windowed(3).map(List<Int>::sum).windowed(2).count { it[1] > it[0] }
	
	println("Total Increases: $totalIncreases")
	println("Windowed Increases: $windowedIncreases")
}
