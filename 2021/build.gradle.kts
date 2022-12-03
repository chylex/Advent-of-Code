@file:Suppress("ConvertLambdaToReference")

plugins {
    kotlin("jvm") version "1.6.0"
	idea
}

group = "com.chylex.adventofcode"
version = "1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
}

idea {
	module.excludeDirs.add(file("gradle"))
}

sourceSets {
	fun make(day: Int) {
		val paddedDay = day.toString().padStart(2, '0')
		val sourceFolder = file(paddedDay)
		
		if (!sourceFolder.exists()) {
			sourceFolder.mkdir()
			sourceFolder.resolve("main.kt").writeText("fun main() {\n\t\n}")
			sourceFolder.resolve("input.txt").writeText("")
		}
		
		create(paddedDay) {
			java.setSrcDirs(listOf(sourceFolder))
			resources.setSrcDirs(listOf(sourceFolder))
			resources.include("*.txt")
		}
		
		tasks.register<JavaExec>(paddedDay) {
			group = "run"
			
			mainClass.set("MainKt")
			workingDir = sourceFolder
			classpath = sourceSets[paddedDay].runtimeClasspath
		}
	}
	
	for (day in 1..8) {
		make(day)
	}
}
