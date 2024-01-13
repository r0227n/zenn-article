---
title: "[Google Play Console] サポートデバイスの設定方法"
emoji: "📱"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["android", "googleplay"]
published: true
---

どうも、[Ryo24](https://twitter.com/r0227n_)です。

![](https://storage.googleapis.com/zenn-user-upload/a8893faaa20d-20240113.png)

Google Play ConsoleでAndroidアプリを公開しようとすると、画像のように`テレビ`や`車`、`Chromebook`も公開対象になります。
これだと、サポート外のデバイスに対してもアプリを公開してしまうので、スマホとタブレットのみに公開するように設定を変更します。

# サポートデバイスの設定変更方法
1. Google Play Consoleにログイン
2. 対象のアプリを選択
3. `リリースとデバイス` -> `デバイスカタログ`を選択
4. `フィルタを追加`を選択 -> `フォームファクタ`項目で`テレビ`と`車`、`Chromebook`を選択する
![](https://storage.googleapis.com/zenn-user-upload/323a869311af-20240113.png)
![](https://storage.googleapis.com/zenn-user-upload/48159da7552e-20240113.png)
5. `デバイスを管理`を選択 -> `デバイスを除外`を選択する (チェックボックスが表示されます)
![](https://storage.googleapis.com/zenn-user-upload/75d910d0ec12-20240113.png)
6. 表示件数を`500`に変更する
7. 「デバイスのモデル」の左にあるチェックボックスをクリックし、全てのデバイスを選択する
8. 右下の「デバイスの除外」ボタンをクリックする
![](https://storage.googleapis.com/zenn-user-upload/6bbbb28750dc-20240113.png)

以上のステップでスマホとタブレットのみに公開するように設定できます。
![](https://storage.googleapis.com/zenn-user-upload/2ff85ca01e42-20240113.png)

# 最後に
今回は、Google Play ConsoleでAndroidアプリをスマホとタブレットのみに公開する方法を紹介しました。
デバイスの公開範囲を変更する方法について、ピンポイントで解説している記事がない&方法がわからず困ったので、今回の記事としてまとめました。
以上、[神聖かまってちゃん](https://shinseikamattechan.jp/)の[へっぽこ聖剣士](https://www.youtube.com/watch?v=1mcYxkP1p2A)で元気をもらっている[Ryo24](https://twitter.com/r0227n_)でした。

