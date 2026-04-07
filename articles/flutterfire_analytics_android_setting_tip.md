---
title: "firebase_analytics「Missing google_app_id.」エラー解決方法"
emoji: "🔥"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Flutter, Firebase, Android]
published: true
---

# 検証環境

- PC: MacBook Air (M2, 2022)
- OS: macOS Ventura Version 13.4
- Flutter: 3.14.0-7.0.pre.18
- Dart: 3.2.0 (build 3.2.0-83.0.dev)
- flutterfire: 0.2.7
- firebase_core: 2.15.0
- firebase_analytics: 10.4.4

# はじめに

FlutterFireでFirebaseAnalytics導入時、Androidの環境構築でハマったのでメモしておきます。
`flutterfire configure` で環境構築を行うとiOSは問題なく動作するのですが、Androidで以下のエラーが発生します。

```bash エラーログ
I/FirebaseApp(17383): Device unlocked: initializing all Firebase APIs for app [DEFAULT]
E/FA      (17383): Missing google_app_id. Firebase Analytics disabled. See https://goo.gl/NAOOOI
```

※ ログだと「アプリIDが間違っている」と表示されていますが、IDは正しいです。

# 原因

`flutterfire`で自動生成される`build.gradle`の設定不備が原因です。
エラーログで表示されていたドキュメントに準拠し、`build.gradle`を修正したら直ります。
https://firebase.google.com/docs/android/setup#groovy_1

## android/build.gradle

```diff gradle build.gradle
dependencies {
    ...
-   classpath 'com.google.gms:google-services:4.3.10'
+   classpath 'com.google.gms:google-services:4.3.15'
    ...
}
```

## android/app/build.gradle

```diff gradle app/build.gradle
plugins {
    id "com.android.application"
    id "kotlin-android"
    id "dev.flutter.flutter-gradle-plugin"
+   id 'com.google.gms.google-services'
}

...

dependencies {
    implementation "org.jetbrains.kotlin:kotlin-stdlib-jdk7:$kotlin_version"

+   // Import the Firebase BoM
+   implementation(platform("com.google.firebase:firebase-bom:32.2.2"))
+
+   // When using the BoM, you don't specify versions in Firebase library dependencies
+
+   // Add the dependency for the Firebase SDK for Google Analytics
+   implementation("com.google.firebase:firebase-analytics-ktx")
+
+   // TODO: Add the dependencies for any other Firebase products you want to use
+   // See https://firebase.google.com/docs/android/setup#available-libraries
+   // For example, add the dependencies for Firebase Authentication and Cloud Firestore
+   implementation("com.google.firebase:firebase-auth-ktx")
+   implementation("com.google.firebase:firebase-firestore-ktx")
}
```

# まとめ

FlutterFireでFirebaseAnalyticsを導入する際、Android側は手動で`build.gradle`を修正する必要があります。ファイルの自動生成内容が古く中途半端に設定があり、初見だと原因があわからないため、 今後のアップデートで修正されることを期待したいです。
以上、ZOC(現: METAMUSE μ)の[family name](https://www.youtube.com/watch?v=IytBgF3UhP0&list=RD4n9Op9mRs84&index=3)を聴き、環境打破を目指す[@r0227n_](https://twitter.com/r0227n_)でした。
