plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "{{PACKAGE}}"
    compileSdk = 34

    defaultConfig {
        applicationId = "{{PACKAGE}}"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "{{VERSION}}"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}
