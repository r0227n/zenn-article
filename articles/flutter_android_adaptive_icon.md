---
title: "【Android】アイコン画像の拡大表示を防ぐ方法"
emoji: "🖼️"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Android", "Flutter"]
published: true
---

Flutter製アプリのアイコンを[flutter_launcher_icons](https://pub.dev/packages/flutter_launcher_icons)でカスタマイズ時、デフォルトだとAndroidのアイコン画像だけ拡大表示され、画像全体をアイコン上に表示することができない問題があります。
![](https://storage.googleapis.com/zenn-user-upload/8318c422b182-20240728.png)
本記事は上記問題を解決する方法について説明していきます。
(Flutter前提ですが、内容としてはAndroidのネイティブ側でも共通だと思います。)

# 解決方法

```diff xml android/app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
 <?xml version="1.0" encoding="utf-8"?>
 <adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
   <background android:drawable="@color/ic_launcher_background"/>

-   <foreground android:drawable="@drawable/ic_launcher_foreground"/>

+   <foreground>
+     <inset
+       android:drawable="@drawable/ic_launcher_foreground"
+       android:inset="16%" />
+  </foreground>
 </adaptive-icon>
```

引用元
https://github.com/fluttercommunity/flutter_launcher_icons/issues/96#issuecomment-1325387518

![](https://storage.googleapis.com/zenn-user-upload/c2eee184d440-20240728.png)

## 解説

**『`foreground icon` が拡大表示されている』** ことが問題のため、iconに対しpaddingを設けてあげることで解決します。
そのため、`foreground`に対し、[inset](https://developer.android.com/reference/android/graphics/Insets)タグ(4辺に余白を設定)を設定し、画像全体を表示させることができます。
