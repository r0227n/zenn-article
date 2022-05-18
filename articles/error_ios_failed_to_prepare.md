---
title: "iPhone実機デバッグ時「Failed to prepare device for development.
This ...」の解決"
emoji: "😹"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Swift, Xcode, Flutter]
published: true
---


Flutterプロジェクトの開発中、iPhoneが認識されず実機検証で躓いたナレッジを共有したいと思います。

# FlutterSDKで実機が認識されない
カメラ周りのテストをしたいと思い、(AndroidStudio上の)デバイス一覧を確認しても接続したiPhoneの名前がない...  
`flutter devices device-id `で確認しても表示されないためXcodeで確認すると認識されていました。

恐る恐るビルドすると
![](https://storage.googleapis.com/zenn-user-upload/960125661760-20220518.png)

```
Failed to prepare device for development.
This operation can fail if the version of the OS on the device is incompatible with the installed version of Xcode. You may also need to restart your mac and device in order to correctly detect compatibility. 
```

見事にエラーを吐き、インストールできませんでした。macOSやXcode、iOSも最新なのにdeviceのversionが悪いというね....

# 解決策
macを再起動しましょう😆

Appleの[Developer Forums](https://developer.apple.com/forums/thread/697883)で同様のエラーに対する質問があり、macを再起動したらビルドできたという先行事例がありました。
実際に試したところ、FlutterSDKが実機を認識し正常にビルドすることができました。