---
title: "freezedの名前付き引数の型定義は要注意"
emoji: "🧊"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["flutter", "dart"]
publication_name: "yumemi_inc"
published: true
---

# はじめに

Flutter/Dartでimmutableなデータクラスを生成する際によく使用される[freezed](https://pub.dev/packages/freezed)パッケージ。\
今回は、freezedを使用する際に発生した警告(`unnecessary_non_null_assertionの`)の解決方法について、自分用のメモとして本記事を執筆しました。

# 問題の状況

以下のようなコードで`fuga.freezed.dart`を生成した際、`analyzer`が[unnecessary_non_null_assertion](https://dart.dev/tools/diagnostic-messages#unnecessary_non_null_assertion)の警告を発生させました。

```dart
// fuga.dart

typedef Hoge = Map<String, dynamic>;

@freezed
abstract class Fuga with _$Fuga {
  const factory Fuga({
    required Hoge json,
  }) = _Fuga;
}
```

# 警告の原因

生成された`fuga.freezed.dart`を確認すると、以下のようなコードが生成されていました。

```dart
// fuga.freezed.dart

Fuga get json;

@pragma('vm:prefer-inline') @override $Res call({Object? json = null,}) {
    return _then(_self.copyWith(
        json: null == json ? _self.json! : json // ignore: cast_nullable_to_non_nullable
        as Map<String, dynamic>,
    ));
}
```

問題は、生成されたコードで`json`プロパティに対して`!`演算子（non-null assertion operator）が使用されていることです。これは`unnecessary_non_null_assertion`警告の原因となります。

# 解決方法

この問題を解決するには、名前付き引数の型定義に`typedef`で定義した独自の型を使用せず、SDK標準の型を直接使用します。

```dart
// fuga.dart

@freezed
abstract class Fuga with _$Fuga {
  const factory Fuga({
    required Map<String, dynamic> json,
  }) = _Fuga;
}
```

この変更により、生成されるコードは以下のようになり、警告が解消されます。

```dart
// fuga.freezed.dart

Map<String, dynamic> get json;

@pragma('vm:prefer-inline') @override $Res call({Object? json = null,}) {
    return _then(_self.copyWith(
        json: null == json ? _self.json : json // ignore: cast_nullable_to_non_nullable
        as Map<String, dynamic>,
    ));
}
```

# なぜこの解決方法が有効なのか

`part`と`part of`でコード連携しているため、`fuga.freezed.dart`は`Fuga`クラスを参照できます。しかし、`build_runner`は`typedef`を考慮せずにコードを生成するため、不必要な`!`演算子が生成されてしまいます。

名前付き引数の型定義には、必ずSDK標準の型を使用することで、この問題を回避できます。

# まとめ

freezedを使用する際に名前付きの型を定義する場合は、以下の点に注意しましょう：

1. `typedef`で定義した独自の型は使用せず、SDK標準の型を直接使用する
2. 特に名前付き引数の型定義では、このルールを厳守する

これにより、不要な警告を防ぎ、よりクリーンなコードを維持することができます。
