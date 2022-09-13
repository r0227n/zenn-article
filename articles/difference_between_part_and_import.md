---
title: "[Dart]partとimportの違い"
emoji: "😔"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["dart", "flutter"]
published: true
---

# はじめに
こんにちは、[Ryo24](https://twitter.com/r0227n_)です。
Dartは`part`と`import`の2種類でファイルを参照することができます。  
本記事は2つの違いと使い分けに関して自分用のメモとして書きました。

## partとimportの違いは何？
## partとは
`part`は`part of`とセットで使い、プログラムを分割します。
```php: one.dart
part 'two.dart';

test();  // print 'test'
```

```php: two.dart
part of 'one.dart';

void test() {
    print('test');
}
```

`one.dart`(part)の拡張として`two.dart`(part of)があります。  

## importとは
`import`はファイルを参照します。
```php: one.dart
import 'two.dart';

test();  // print 'test'
```

```php: two.dart
void test() {
    print('test');
}
```

## 違いは何？
一見、`part`と`import`の違いわかりません...
この2つの違いは、
> In Dart, private members are accessible within the same library. With import you import a library and can access only its public members. With part/part of you can split one library into several files and private members are accessible for all code within these files.  
> 引用元: [When to use part/part of versus import/export in Dart?](https://stackoverflow.com/questions/27763378/when-to-use-part-part-of-versus-import-export-in-dart#:~:text=In%20Dart%2C%20private%20members%20are,all%20code%20within%20these%20files.)

つまり、
- `part` : **public**と**private**どちらも読み込む
- `import` : **public**しか読み込まない

上記の違いがあり、参照内容が完全に異なることがわかります。

# 結論
基本的に`import`を使いましょう。
## partを使うタイミング
`freezed`をはじめとする、コードを自動生成する実装で使う。
:::message
ライブラリ作成時、partではなくミニアプリを作成しimportすることを推奨されています。
:::
## importを使うタイミング
コンポーネントを活用し、機能を追加するときに使う(= 基本的にこいつを使う)。

# 参考文献
https://stackoverflow.com/questions/27763378/when-to-use-part-part-of-versus-import-export-in-dart

https://dart.dev/guides/libraries/create-library-packages#organizing-a-library-package
