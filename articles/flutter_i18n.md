---
title: "【Flutter】多言語化対応のいろは（仮）"
emoji: "🌍"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Flutter", "i18n"]
published: false
---
https://docs.flutter.dev/ui/accessibility-and-internationalization/internationalization

```bash
flutter pub add flutter_localizations --sdk=flutter
flutter pub add intl:any
```

<!-- TODO: 公式ドキュメントで取り上げられていないが、Info.plistの内容やアプリ上で直接表示言語を変える方法が存在するが、まとまった記事がない -->

# Info.plist(iOS&macOSのネイティブAPI処理形)の多言語化対応

`Info.plist`は、デバイスの機能に対するアクセス許可やアプリの表示名などを記述する設定ファイルです。本節はデバイスの言語設定に連動し、設定ファイルから参照される値を変更する方法について説明します。

最初に以下のコマンドをプロジェクトのルートディレクトリで実行し、Xcodeを起動します。

```bash
open ios/Runner.xcworkspace/
```

続いて`⌘+N`(新規ファイル作成ショートカットキー)を押下し、右上の`Filter`に「String」と入力し、**String Catalog**を見つけ選択し、**InfoPlist.xcstrings**ファイルを作成する。
最後に多言語化対象を追加します。画面左下の「＋」アイコンを押下し、対象の言語を選択することにより、多言語化対応の対象を追加することができます。

以上で`Info.plist`の`<value>`を変更する準備は整いました。
あとは

1. 一度プロジェクトを実行する
2. 1を実行すると、自動で`Info.plist`のkey/valueが`InfoPlist.xcstrings`に反映される
3. 2で反映された内容を編集し、言語ごとの表示内容を編集する

以上で多言語化作業が完了します。

# アプリ上で表示言語(Locale)の変更方法

<!-- TODO: iOSはアプリごとに表示言語をカスタマイズ可能だが、Androidは変更可能じゃない。そのため、アプリ上で変更することができる機能を追加する時がある。 -->

```dart
MaterialApp(
  localizationsDelegates: AppLocalizations.localizationsDelegates,
  cales: const [
    Locale('ja'),
    Locale('en'),
  ],
  locale: Locale('ja'), // TODO: ここの値を更新する!
  home: const MyHomePage(),
);
```

<!-- TODO: URLを挿入する -->
[MaterialApp Widget]()の[locale]()プロパティに代入する値を切り替えることにより、アプリ上の表示言語を切り替えることができます。

<!-- TODO: Localeの説明 -->

## Riverpodを用いたLocale変更方法のサンプル

<!-- TODO: shared_preferenceやRiverpod周りのflutter pub add -->
```dart preferences_provider.dart
import 'package:riverpod_annotation/riverpod_annotation.dart';

import 'package:shared_preferences/shared_preferences.dart';

part 'preferences_provider.g.dart';

@Riverpod(keepAlive: true)
SharedPreferences sharedPreferences<SharedPreferences>(SharedPreferencesRef ref) =>
    throw UnimplementedError();
```

アプリの設定周りをshared_preferenceで記録し、参照するため`keepAlive`を`true`に設定し、状態を常に保持するようにしています。
また、初期化処理が失敗した状態で設定周りを参照するのはエラーの原因になり得るため、インスタンスを上書きするのに失敗したら`throw`するようにしています。

<!-- TODO: main.dartでインスタンスの保持方法についてExampleコード -->
<!-- TODO: Localeを実際に変更するコードのサンプル -->
<!-- TODO: Exampleコードのリンク -->

# まとめ
<!-- TODO: .arbファイルの変更方法について解説はあるが、それ以外の多言語化対応についての記事が少ないから書いたという感じの内容を書く -->
