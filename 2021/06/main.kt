import java.io.File
import java.util.Collections

fun main() {
	val initialConfiguration = File("input/1.txt").readLines().single().split(',').map(String::toInt)
	val configuration = FishConfiguration(initialConfiguration)
	
	for (day in 1..80) {
		configuration.advanceToNextDay()
		// configuration.print()
	}
	
	println("Total fish: ${configuration.totalFish}")
}

class FishConfiguration(initialConfiguration: List<Int>) {
	private val state = initialConfiguration.toMutableList()
	private var day = 0
	
	val totalFish
		get() = state.size
	
	fun advanceToNextDay() {
		++day
		
		var newFish = 0
		for ((index, timer) in state.withIndex()) {
			if (timer == 0) {
				++newFish
				state[index] = 6
			}
			else {
				state[index] = timer - 1
			}
		}
		
		state.addAll(Collections.nCopies(newFish, 8))
	}
	
	@Suppress("unused")
	fun print() {
		println("Day ${day.toString().padStart(3)}: ${state.joinToString(",")}")
	}
}
