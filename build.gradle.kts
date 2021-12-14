plugins {
    kotlin("jvm") version "1.6.0"
	idea
}

group = "com.chylex.adventofcode"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

idea {
	module.excludeDirs.addAll(listOf(
		file(".gradle"),
		file("build"),
		file("cmake-build-debug"),
		file("gradle")
	))
}

sourceSets {
	fun make(year: Int, day: Int) {
		val paddedDay = day.toString().padStart(2, '0')
		
		val sourceFolder = file("$year/$paddedDay")
		val resourceFolder = sourceFolder.resolve("input")
		
		if (!sourceFolder.exists()) {
			sourceFolder.mkdir()
			sourceFolder.resolve("main.kt").writeText("fun main() {\n\t\n}")
			resourceFolder.mkdir()
			resourceFolder.resolve("1.txt").writeText("")
		}
		
		create("$year-$paddedDay") {
			java.setSrcDirs(listOf(sourceFolder))
			resources.setSrcDirs(listOf(resourceFolder))
		}
	}
	
	make(2021, 1)
	make(2021, 2)
	make(2021, 3)
	make(2021, 4)
	make(2021, 5)
	make(2021, 6)
	make(2021, 7)
}
