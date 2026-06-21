plugins {
    id("com.android.library")
}

group = "com.flutter_rust_bridge.xue_hua_app_badge"
version = "1.0"

android {
    namespace = "com.flutter_rust_bridge.xue_hua_app_badge"
    compileSdk = 34

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    defaultConfig {
        minSdk = 19
    }
}

kotlin {
    compilerOptions {
        jvmTarget = org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_17
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("me.leolin:ShortcutBadger:1.1.22@aar")
}
