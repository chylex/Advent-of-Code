plugins {
    kotlin("jvm") version "1.5.10"
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
		file("cmake-build-debug"),
		file("gradle")
	))
}

sourceSets {
	fun make(year: Int, day: Int) {
		val paddedDay = day.toString().padStart(2, '0')
		val folder = file("$year/$paddedDay")
		
		create("$year-$paddedDay") {
			java.setSrcDirs(listOf(folder))
			resources.setSrcDirs(listOf(folder.resolve("input")))
		}
	}
	
	make(2021, 1)
}
