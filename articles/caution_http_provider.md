---
title: "「flutter: Connection attempt cancelled, host: ...」の対処法"
emoji: "🐡"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Flutter, Dart, http]
published: true
---

筆者はhttp通信の実装時、[httpパッケージ](https://pub.dev/packages/http)を活用します。\
今回、`port 443`でエラーを吐き解決するのに苦労したため知見を共有したいと思い記事にしました。

# 「SocketException: Connection attempt cancelled, host: www.googleapis.com, port: 443」を吐く...

:::details エラーを吐くコード

```dart: bad.dart
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:http/http.dart' as http;

final googleBooksApiClientProvider = Provider.autoDispose<GoogleBooksApiClient>((ref) {
  final http.Client client = http.Client();
  ref.onDispose(() => client.close());

  return GoogleBooksApiClient(client);
});

class GoogleBooksApiClient {
  GoogleBooksApiClient(this._client);
  final http.Client _client;

  Future<void> getData() async {
    final endpoint = Uri.https('www.googleapis.com', 'books/v1/volumes', {'q': '9784023340121', 'maxResults': '40', 'langRestrict': 'ja'});
    try {
      await _client.get(endpoint);
    } catch(error) {
      print(error);
      return;
    }
    print('ok');
  }
}
```

:::

```txt: エラー文
flutter: SocketException: Connection attempt cancelled, host: www.googleapis.com, port: 443
```

## 「port 443」とは？

> HTTPSでWebブラウザなどと通信するために用いる
> 引用元: https://e-words.jp/w/443%E7%95%AA%E3%83%9D%E3%83%BC%E3%83%88.html

のため、通信処理自体が失敗していると思われる。

## flutter doctor -v

```
[✓] HTTP Host Availability
    • All required HTTP hosts are available
```

`doctor`コマンドで確認したが、HTTPリクエストを全て利用できていることが確認できました。

# 原因

**Automatic field initialization**で`Client`のインスタンスを取得していることが原因でした...
試しに`late`で宣言し、constructorで初期化しました。その結果、うまく動作しました！
:::details 正常に動くコード

```dart: good.dart
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:http/http.dart' as http;

final googleBooksApiClientProvider = Provider.autoDispose<GoogleBooksApiClient>((ref) {
  return GoogleBooksApiClient();
});

class GoogleBooksApiClient {
  GoogleBooksApiClient() {
    _client = http.Client();
  }
  late final http.Client _client;

  Future<void> getData() async {
    final endpoint = Uri.https('www.googleapis.com', 'books/v1/volumes', {'q': '9784023340121', 'maxResults': '40', 'langRestrict': 'ja'});
    try {
      await _client.get(endpoint);
    } catch(error) {
      print(error);
      return;
    }
    print('ok');
  }
}
```

:::

# まとめ

`Provider`内でインスタンスを発行し、`onDispose`で`close()`ではなくclass内で`close()`しましょう。
