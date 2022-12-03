import java.io.File

const val SIZE = 5

fun main() {
	val lineIterator = File("input.txt").readLines().iterator()
	val numbersToDraw = lineIterator.next().split(',').map(String::toInt).toList()
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
	
	part1(numbersToDraw, boards)
	part2(numbersToDraw, boards)
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

private fun part1(numbersToDraw: List<Int>, boards: List<Board>) {
	val numbersLeftToDraw = ArrayDeque(numbersToDraw)
	val drawnNumbers = mutableSetOf<Int>()
	
	while (numbersLeftToDraw.isNotEmpty()) {
		val calledNumber = numbersLeftToDraw.removeFirst()
		drawnNumbers.add(calledNumber)
		
		val winner = boards.firstOrNull { it.checkWin(drawnNumbers) }
		if (winner != null) {
			val score = winner.getScore(drawnNumbers, calledNumber)
			println("Score of first board: $score")
			return
		}
	}
}

private fun part2(numbersToDraw: List<Int>, boards: List<Board>) {
	val numbersLeftToDraw = ArrayDeque(numbersToDraw)
	val drawnNumbers = mutableSetOf<Int>()
	val boardsLeft = boards.toMutableList()
	
	while (numbersLeftToDraw.isNotEmpty()) {
		val calledNumber = numbersLeftToDraw.removeFirst()
		drawnNumbers.add(calledNumber)
		
		while (true) {
			val winner = boardsLeft.firstOrNull { it.checkWin(drawnNumbers) } ?: break
			
			if (boardsLeft.size > 1) {
				boardsLeft.remove(winner)
			}
			else {
				val score = winner.getScore(drawnNumbers, calledNumber)
				println("Score of last board: $score")
				return
			}
		}
	}
}
