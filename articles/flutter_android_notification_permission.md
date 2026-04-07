---
title: "【Flutter】AndroidのPush通知使用許諾を表示する方法"
emoji: "📢"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Flutter, Android]
published: true
---

どうもこんにちは、[Ryo24](https://twitter.com/r0227n_)です。

iOSでは、通知に関する実行権限を設定すると、自動的にユーザーに許可を求めるダイアログ(以後、`通知使用許諾`と呼称)が表示されます。一方、Androidでは、権限を設定しても自動的に通知使用許諾が表示されないため、**起動時に表示するコード**を書かなければなりません。

本記事では、Flutterを用いて作成したAndroidアプリで起動時に通知使用許諾を表示する方法について解説します
設定内容のみ知りたい方は、[こちら](https://github.com/r0227n/zenn-article/pull/14/commits/7c156f059e9af0ccfb693e63a4ef85d339b70315)からどうぞ。

(本記事は[Flutter Advent Calendar 2023 6日目](https://qiita.com/advent-calendar/2023/flutter)となります。)

# 通知設定

[POST_NOTIFICATIONS](https://developer.android.com/reference/android/Manifest.permission#POST_NOTIFICATIONS)(通知に関する実行権限)は**Android 13（API レベル 33）以上**をサポートしており、ドキュメントにもプロジェクトは13以上で作成することをを推奨しています。

> プラットフォームの API を利用して権限をリクエストするには、Android 13 以降をターゲットとするようにアプリを更新することを強くおすすめします。
> 引用元: [Android 12L（API レベル 32）以下をターゲットとするアプリの通知権限](https://firebase.google.com/docs/cloud-messaging/android/client?hl=ja#notification_permissions_for_apps_targeting_android_12l_api_level_32_or_lower)

## 通知使用許諾をリクエスト

:::details 通知使用許諾の最小限コード

```kotlin: MainActivity.kt
// Declare the launcher at the top of your Activity/Fragment:
private val requestPermissionLauncher = registerForActivityResult(
    ActivityResultContracts.RequestPermission(),
) { isGranted: Boolean ->
    if (isGranted) {
        // FCM SDK (and your app) can post notifications.
    } else {
        // TODO: Inform user that that your app will not show notifications.
    }
}

private fun askNotificationPermission() {
    // This is only necessary for API level >= 33 (TIRAMISU)
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
        if (ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) ==
            PackageManager.PERMISSION_GRANTED
        ) {
            // FCM SDK (and your app) can post notifications.
        } else if (shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS)) {
            // TODO: display an educational UI explaining to the user the features that will be enabled
            //       by them granting the POST_NOTIFICATION permission. This UI should provide the user
            //       "OK" and "No thanks" buttons. If the user selects "OK," directly request the permission.
            //       If the user selects "No thanks," allow the user to continue without notifications.
        } else {
            // Directly ask for the permission
            requestPermissionLauncher.launch(Manifest.permission.POST_NOTIFICATIONS)
        }
    }
}
```

:::

この中で**registerForActivityResult()** が登場しており、これが一番のポイントです。
デフォルトの`FlutterActivity()` は `registerForActivityResult()` をサポートしていないため、他のclassを継承しなければなりません。

### registerForActivityResult() とは？

> ComponentActivity または Fragment の Activity Result API には、結果のコールバックを登録するための registerForActivityResult() が用意されています。registerForActivityResult() は ActivityResultContract と ActivityResultCallback を受け取って、他のアクティビティを開始するために使用する ActivityResultLauncher を返します。
> 引用元: [アクティビティの結果に対してコールバックを登録する](https://developer.android.com/training/basics/intents/result?hl=ja#register)

より、プロジェクトは

- ComponentActivity
- Fragment

をサポートしていなければなりません。

### FlutterFragmentActivity

[FlutterFragmentActivity](https://api.flutter.dev/javadoc/io/flutter/app/FlutterFragmentActivity.html)は、**ComponentActivity** を継承しており、**registerForActivityResult()** をサポートしているため、`MainActivity` は `FlutterFragmentActivity` で書き変えなければなりません。

# コード変更点まとめ

## 通知に関する実行権限の設定

```diff xml:AndroidManifest.xml
...
+   <uses-permission android:name="android.permission.POST_NOTIFICATIONS"/>
...
```

## ライブラリやモジュールの追加

```diff groovy:build.gradle
dependencies {
+   implementation "org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.8.20"
+   implementation 'androidx.activity:activity-ktx:1.7.2'
+   implementation 'androidx.fragment:fragment-ktx:1.6.0'
}
```

## MainActivity

https://github.com/r0227n/zenn-article/blob/develop/samples/android_notification_permission/android/app/src/main/kotlin/com/example/android_notification_permission/MainActivity.kt

# まとめ

- `AndroidManifest.xml`に通知に関する実行権限を追加
- `build.gradle`に必要なライブラリやモジュールを追加
- `MainActivity.kt`のコードを更新

上記の3点を実装することで、Flutterアプリで通知使用許諾を表示することができます。
Dart側でコードを書かず、ネイティブ側で実装する必要があるため、Flutterエンジニアにとってはネイティブの知識が必要になります。

# さいごに

[flutter_local_notifications](https://pub.dev/packages/flutter_local_notifications)や[firebase_messaging](https://pub.dev/packages/firebase_messaging)などPush通知ライブラリのドキュメントに通知使用許諾の表示方法が記載されておらず、実装時に困ったため、本記事を書きました。

本記事が通知使用許諾を表示する方法の参考になれば幸いです。

以上、[THE KEBABS](https://kebabsband.com/)の[THE KEBABSを抱きしめて](https://www.youtube.com/watch?v=sFBDTxHeOkY)を聴き、大人になっても青春を謳歌したい[Ryo24](https://twitter.com/r0227n_)でした。

# サンプルプロジェクト

https://github.com/r0227n/zenn-article/tree/develop/samples/android_notification_permission

# 参考

https://developer.android.com/about/versions/13/changes/notification-permission?hl=ja#use
https://firebase.google.com/docs/cloud-messaging/android/client?hl=ja#request-permission13
