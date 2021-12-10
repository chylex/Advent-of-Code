import java.io.File
import java.math.BigInteger
import kotlin.system.measureTimeMillis

fun main() {
	val initialConfiguration = File("input/1.txt").readLines()
		.single()
		.split(',')
		.map(String::toInt)
	
	println("(Took ${measureTimeMillis { part1(initialConfiguration) }} ms)")
	println("(Took ${measureTimeMillis { part2(initialConfiguration) }} ms)")
}

class FishConfiguration(initialConfiguration: List<Int>) {
	private var fishCountByAge = Array(9) { age -> initialConfiguration.count { it == age }.toBigInteger() }
	private var day = 0
	
	private val totalFish
		get() = fishCountByAge.fold(BigInteger.ZERO, BigInteger::add)
	
	fun advance(days: Int) {
		for (day in 1..days) {
			advanceToNextDay()
		}
	}
	
	private fun advanceToNextDay() {
		val newCountByAge = Array(9) { BigInteger.ZERO}
		
		for ((age, count) in fishCountByAge.withIndex()) {
			if (age == 0) {
				newCountByAge[6] += count
				newCountByAge[8] += count
			}
			else {
				newCountByAge[age - 1] += count
			}
		}
		
		fishCountByAge = newCountByAge
		++day
		
		println("Day ${day.toString().padStart(3)}: $totalFish fish")
	}
}

fun part1(initialConfiguration: List<Int>) {
	FishConfiguration(initialConfiguration).advance(80)
}

fun part2(initialConfiguration: List<Int>) {
	FishConfiguration(initialConfiguration).advance(256)
}
