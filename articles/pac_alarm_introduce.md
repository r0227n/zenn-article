---
title: "【Flutter】アラーム機能を実現するためのalarmパッケージの紹介"
emoji: "⏰"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Flutter"]
published: true
---

Android/iOSでアラーム機能を実装する[alarm](https://pub.dev/packages/alarm) パッケージを紹介します。
このパッケージは
- [android_alarm_manager_plus](https://pub.dev/packages/android_alarm_manager_plus)
- [flutter_local_notifications](https://pub.dev/packages/flutter_local_notifications)
- [just_audio](https://pub.dev/packages/just_audio)
- [shared_preferences](https://pub.dev/packages/shared_preferences)

上記4つのパッケージを組み合わせ、アラーム機能を実装しています。

# サポートされている機能
| シチュエーション            | サウンド再生 | バイブレーション | 通知               |
|--------------------------|--------------|-----------------|-------------------|
| ロックされた画面         | ✅          | ✅             | ✅               |
| サイレント / ミュート     | ✅          | ✅             | ✅               |
| ノットディスターブ       | ✅          | ✅             | サイレンス        |
| スリープモード           | ✅          | ✅             | サイレンス        |
| 他のメディアを再生中     | ✅          | ✅             | ✅               |
| アプリが終了した状態     | ❌          | ❌             | ✅               |

iOSの仕様上、アプリ起動中以外はアラームを鳴らすことができません。そのため、プッシュ通知で時間経過を知らせることでアラームの代用としています。

↓こちらの記事にiOSでアラーム機能実装の問題点をまとめています。
https://qiita.com/K_Kenty/items/d589a0f0e1949e167aab#%E6%9C%80%E5%A4%A7%E3%81%AE%E8%90%BD%E3%81%A8%E3%81%97%E7%A9%B4

# パッケージのセットアップ
```bash
flutter pub add alarm
```
パッケージを追加し、
- Androidは以下のファイルを編集
  - `android/app/src/main/AndroidManifest.xml`
  - `android/app/src/main/AndroidManifest.xml`
- iOSはXcodeで以下の設定を行う
  - `Audio, AirPlay, and Picture in Picture`を有効にする
## Android
`android/app/build.gradle`
```gradel
android {
  compileSdkVersion 33
  [...]
  defaultConfig {
    [...]
    multiDexEnabled true
  }
}
```


`android/app/src/main/AndroidManifest.xml`
```xml
<!--  <manifest></manifest>に追加 -->
<uses-permission android:name="android.permission.RECEIVE_BOOT_COMPLETED"/>
<uses-permission android:name="android.permission.WAKE_LOCK"/>
<!-- For apps with targetSDK=31 (Android 12) -->
<uses-permission android:name="android.permission.SCHEDULE_EXACT_ALARM"/>


<!--  <application></application> に追加 -->
<service
    android:name="dev.fluttercommunity.plus.androidalarmmanager.AlarmService"
    android:permission="android.permission.BIND_JOB_SERVICE"
    android:exported="false"/>
<receiver
    android:name="dev.fluttercommunity.plus.androidalarmmanager.AlarmBroadcastReceiver"
    android:exported="false"/>
<receiver
    android:name="dev.fluttercommunity.plus.androidalarmmanager.RebootBroadcastReceiver"
    android:enabled="false"
    android:exported="false">
    <intent-filter>
        <action android:name="android.intent.action.BOOT_COMPLETED" />
    </intent-filter>
</receiver>
```

## iOS
Xcodeで`Runner -> Signing & Capabilities -> Capabillity -> Background Modes -> Audio, AirPlay, and Picture in Picture`を有効にし、アプリがバックグラウンドにあるときにオーディオを再生できるようにします。
![](https://storage.googleapis.com/zenn-user-upload/e21cfddb3858-20230727.png)
![](https://storage.googleapis.com/zenn-user-upload/e7b28272a056-20230727.png)
![](https://storage.googleapis.com/zenn-user-upload/ef6511d58157-20230727.png)



# 使い方
[Alarm class](https://pub.dev/documentation/alarm/latest/alarm/Alarm-class.html)のmethodを使い、アラームの操作を行います。

下記4種類の処理が実装されています。

- 登録
- 止める&削除
- 取得
- アラームが鳴動したときの処理



## 登録
```dart アラートの登録
// アラームの設定を作成
final alarmSettings = AlarmSettings(
  id: 42,
  dateTime: dateTime,
  assetAudioPath: 'assets/alarm.mp3',
  loopAudio: true,
  vibrate: true,
  fadeDuration: 3.0,
  notificationTitle: 'This is the title',
  notificationBody: 'This is the body',
  enableNotificationOnKill: true,
);

// アラームを登録
await Alarm.set(settings: alarmSettings);
```
基本的には`AlarmSettings`クラスのインスタンスを作成し、`Alarm`クラスの`set`methodに渡すことでアラームを登録できます。

### 設定項目
| Property                | Type      | Description                                                                                                                       |
|-------------------------|-----------|-----------------------------------------------------------------------------------------------------------------------------------|
| id                      | int       | ユニークな識別子。                                                                                                 |
| alarmDateTime           | DateTime  | アラームが鳴動する日時。                                                                                                 |
| assetAudio              | String    | 着信音として使用する音源へのパス。
| loopAudio               | bool      | `true`: アラーム音が停止するまで無限に繰り返す / `false`: オーディオが終了すると振動も停止し              |
| vibrate                 | bool      | `true`: アラームが停止するまでデバイスが無限に振動する。[loopAudio]がfalseに設定されている場合、振動はオーディオの終了と同時に停止する。|
| fadeDuration            | double    | アラーム音の音量をフェードする秒数。デフォルトは0で、フェードしない。
| notificationTitle       | String    | アプリがバックグラウンドにある場合、アラームが鳴動したときにトリガされる通知のタイトル。                                       |
| notificationBody        | String    | アプリがバックグラウンドにある場合、アラームが鳴動したときにトリガされる通知の本文。                                                                                                                       |
| enableNotificationOnKill | bool      | アプリが終了したときにアラームが鳴らない可能性をユーザーに警告するために通知を表示するかどうか。デフォルトで`true`。     |
| stopOnNotificationOpen  | bool      | 受信した通知を開いたときにアラームを停止するかどうか。デフォルトで`true`。                                           |

## 止める&削除
```dart アラームを止める&削除
await Alarm.stop(id: 42);
```
`Alarm.stop`methodにアラームのidを渡すことでアラームを止めることができます。
(`shared_preferences`で管理している`AlarmSettings`を削除し、アラームを止める&削除を実装しています。)

## 取得
```dart アラームの取得
final alarm = await Alarm.getAlarm(id: 42);
```
`Alarm.getAlarm`methodにアラームのidを渡すことでアラームを取得することができます。

## アラームが鳴動したときの処理
```dart アラームが鳴動したときの処理
Alarm.ringStream.stream.listen((_) {
    // アラームが鳴動したときの処理
});
```
`Alarm.ringStream`を`listen`することでアラームが鳴動したときの処理を実装することができます。

# 課題点
- 依存パッケージが多い
- 消音モードでもアラーム音が再生される

上記2点が目立つ課題です。
アラーム機能に必要な既存パッケージを組み合わせて実装しているため、依存関係やバージョン管理などが複雑でアップデート時の動作確認が大変です。また`alarm`ではなく [audio_session](https://pub.dev/packages/audio_session)の問題ですが、消音モードでもアラーム音が再生されるため、アラーム機能としては不十分な状態です。

# まとめ
`alarm`パッケージを使うことで、簡単にAndroid/iOSでアラーム機能を実装することができます。しかし、依存関係が複雑かつ消音モードでもアラーム音が再生されるため、短期的なプロジェクト&開発コストを抑えたい場合には向いていると思いますが、長期的なプロジェクトには向いていないと思います。
開発中の機能で安定はしないと思いますが、[ffigen](https://dart.dev/guides/libraries/objective-c-interop)や[jnigen](https://dart.dev/guides/libraries/java-interop)でネイティブコードを呼び出し、外部パッケージを使わずにアラーム機能を実装したいですね。