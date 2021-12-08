import java.io.File

const val SIZE = 5

fun main() {
	val lineIterator = File("input/1.txt").readLines().iterator()
	val numbersToDraw = ArrayDeque(lineIterator.next().split(',').map(String::toInt))
	val boards = mutableListOf<Board>()
	
	while (lineIterator.hasNext()) {
		require(lineIterator.next().isEmpty())
		
		if (!lineIterator.hasNext()) {
			break
		}
		
		val numbers = Array(SIZE) {
			lineIterator.next().split(' ').filter(String::isNotEmpty).map(String::toInt).toIntArray()
		}
		
		boards.add(Board(numbers))
	}
	
	val drawnNumbers = mutableSetOf<Int>()
	
	while (numbersToDraw.isNotEmpty()) {
		val calledNumber = numbersToDraw.removeFirst()
		drawnNumbers.add(calledNumber)
		
		val winner = boards.firstOrNull { it.checkWin(drawnNumbers) }
		if (winner != null) {
			val score = winner.getScore(drawnNumbers, calledNumber)
			println("Score: $score")
			break
		}
	}
}

class Board(private val numbers: Array<IntArray>) {
	fun checkWin(drawnNumbers: Set<Int>): Boolean {
		for (row in 0 until SIZE) {
			if (numbers[row].all(drawnNumbers::contains)) {
				return true
			}
		}
		
		for (column in 0 until SIZE) {
			if ((0 until SIZE).all { row -> numbers[row][column] in drawnNumbers }) {
				return true
			}
		}
		
		return false
	}
	
	fun getScore(drawnNumbers: Set<Int>, calledNumber: Int): Int {
		return calledNumber * numbers.sumOf { it.filterNot(drawnNumbers::contains).sum() }
	}
}
