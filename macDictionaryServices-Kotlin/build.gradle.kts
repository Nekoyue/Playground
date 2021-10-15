plugins {
    kotlin("multiplatform") version "1.5.31"
}

group = "moe.nekoyue"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

kotlin {
    macosX64("native").apply {
        binaries {
            executable {
                entryPoint = "main"
            }
        }
    }
    sourceSets {
        val nativeMain by getting
    }
}
