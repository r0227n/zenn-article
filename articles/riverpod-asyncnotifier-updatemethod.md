---
title: "[Riverpod]AsyncNotifierの正しいState更新方法"
emoji: "🧘‍♂️"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Flutter", "Riverpod"]
published: true
---

[Remi Rousselet](https://github.com/rrousselGit)さんが[update method](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncNotifier/update.html)の[正しい使い方](https://github.com/rrousselGit/riverpod/issues/2205)を紹介していたため、備忘録として残しておきます。

https://github.com/rrousselGit/riverpod/issues/2205

# AsyncNotifierとは
[Riverpod](https://docs-v2.riverpod.dev/)(Flutterの状態管理ライブラリ)における、非同期処理を管理するための基底クラスです。
`State`を[AsyncValue](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncValue-class.html)でラップし、
- [AsyncLoading()](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncLoading-class.html)
- [AsyncData()](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncData-class.html)
- [AsyncError()](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncError-class.html)

の3つの状態で`State`を管理します。

## AsyncValueとは
AsyncValueは、
- [AsyncLoading()](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncLoading-class.html): 値がまだ利用可能でないことを示す「ロード中」の状態
- [AsyncData()](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncData-class.html): 値が正常に読み込まれたことを示す「完了」の状態
- [AsyncError()](https://pub.dev/documentation/riverpod/latest/riverpod/AsyncError-class.html):エラーが発生したことを示す「エラー」の状態

のいずれかを持つことができ、状態に応じて処理を分岐させるメソッドを提供しています。

詳しいことは以下の記事を参照してください。
https://zenn.dev/tsuruo/articles/52f62fc78df6d5


## update methodとは
`AsyncNotifier`で管理されている`State`を更新するためのメソッドです。
このメソッドは、`AsyncValue`を更新及びエラーハンドリングを一括で宣言することができます。
```dart
update((data) {
  state = const AsyncLoading();
  // 更新処理
}, onError: (error, stack) {
  state = AsyncError(error, stack);
  // エラーハンドリング
}
```

# update methodの正しい使い方
```dart
  /// ケースA: アンチパターン
  /// Stateが常時[AsyncLoading()]になるため、「非同期処理が永遠に続いている」状態になる
  void caseA() {
    state = const AsyncLoading();
    update((data) => data = 2);
  }

  /// ケースB: 良い実装
  /// Stateに[AsyncLoading()]を代入し、非同期処理が開始します。その後、処理の結果が[AsyncData()]もしくは[AsyncError()]としてStateに代入される
  void caseB() {
    update((data) {
      state = const AsyncLoading();
      return data += 1;
    });
  }

  /// ケースC: 非推奨の実装
  /// Stateに[AsyncData()]もしくは[AsyncError()]の値が代入される。だが、[AsyncLoading()]にならないため、「非同期処理を実行中」の状態を把握しておらず、状態管理として不十分。
  void caseC() {
    update((unused) => unused += 1);
  }
```

上記3種類の実装例が考えられるが、**caseB**の実装方法が非同期の状態を管理する方法として最適である。

## caseA
![](https://storage.googleapis.com/zenn-user-upload/e9bf4a9a69bf-20230328.gif)

`update`内での処理が反映されず、`State`は`AsyncLoading()`の状態で更新されない状態になります。

## caseB & caseC
![](https://storage.googleapis.com/zenn-user-upload/81f8bec3dee0-20230328.gif)


`update`内での処理が反映され、`State`に`AsyncData()`(もしくは`AsyncError()`)が代入され値が更新されます。
caseCの場合、`AsyncLoading()`の状態を管理できないため不十分な実装になります。

# Stateの更新方法使い分け
```dart
/// [update method]を使ったケース
update((data) {
  state = const AsyncLoading();
  return data += 1;
});

---

/// [AsyncValue]を使ったケース
state = const AsyncLoading();
state = AsyncValue.guard(() {
  return state.when(data);
})
```

`update method`は **「このAsyncNotifierで管理されている情報を更新しているぞ！」** と明示的に表現することができるため、`AsynNotifier`の`State`のみ変更するユースケースでは積極的に使いますが、
- `別State`の更新
- 他Providerを参照し、`State`を更新する

など更新処理が複雑だと可読性が低下し、エラーハンドリングが困難になります。これらの問題を回避するため、`AsyncValueを使ったケース`を使用します。