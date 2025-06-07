---
title: "Dartで全角数字を半角数字に変換する方法"
emoji: "🔢"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["dart"]
published: true
---

# Dartで全角数字を半角数字に変換する方法

## はじめに

日本語を扱うアプリケーション開発において、全角文字と半角文字の変換は避けて通れない課題の一つです。  
特に、ユーザー入力やURL処理において、全角数字(`０１２３...`)を半角数字(`0123...`)に統一する必要があるケースは多々あります。

本記事では、`Dart`における効率的な全角数字変換の実装方法と、その背景にある`Unicode`文字コードの仕組みについて詳しく解説します。

## 問題の背景

### なぜ全角・半角変換が必要なのか？

- **ユーザビリティの向上**: 日本語入力時に無意識に全角数字が入力されることがある
- **データの統一性**: データベースやAPI処理において文字形式を統一する必要がある
- **URL処理**: ルーティング処理で数値パラメータを正確に解析する必要がある

## 実装の詳細解説

### 1. 基本的なString拡張メソッド

```dart
extension ExtString on String {
  String convertFullWidthToHalfWidth() {
    final buffer = StringBuffer();

    for (final i = 0; i < length; i++) {
      final char = this[i];
      final codeUnit = char.codeUnitAt(0);

      // 全角数字（0xFF10-0xFF19）を半角（0x30-0x39）に変換
      if (codeUnit >= 0xFF10 && codeUnit <= 0xFF19) {
        buffer.write(String.fromCharCode(codeUnit - 0xFEE0));
      } else if (codeUnit >= 0x30 && codeUnit <= 0x39) {
        // 既に半角数字の場合はそのまま
        buffer.write(char);
      } else {
        // 数字以外の文字が含まれている場合
        throw FormatException('$this: $char is not a number');
      }
    }

    return buffer.toString();
  }
}
```

### 2. Unicode文字コードの仕組み

#### 全角数字と半角数字のコード範囲

| 文字種類 | 範囲 | Unicodeコード |
|---------|------|---------------|
| 半角数字 | 0-9 | `0x30` - `0x39` |
| 全角数字 | ０-９ | `0xFF10` - `0xFF19` |

#### 変換の数学的原理

全角数字から半角数字への変換は、単純な算術演算で実現できます：

```txt
半角コード = 全角コード - 0xFEE0
```

##### 例

- 全角の「５」(`0xFF15`) → 半角の「5」(`0x35`)
- 計算: `0xFF15` - `0xFEE0` = `0x35`

### 3. 実装のポイント

#### パフォーマンス最適化

- **StringBuffer使用**: 文字列連結の効率化
- **直接的な文字コード操作**: 正規表現より高速

#### エラーハンドリング

```dart
// 数字以外の文字が含まれている場合
throw FormatException('$this: $char is not a number');
```

厳密な数字チェックにより、予期しない文字の混入を防止します。

## URI処理への応用

### URL路径での全角数字検出

```dart
extension ExtUri on Uri {
  bool containsFullWidthDigits() {
    for (final segment in pathSegments) {
      for (final i = 0; i < segment.length; i++) {
        final codeUnit = segment.codeUnitAt(i);
        if (codeUnit >= 0xFF10 && codeUnit <= 0xFF19) {
          return true;
        }
      }
    }
    return false;
  }
}
```

### パスセグメントの一括変換

```dart
List<String> pathSegmentsFullWidthToHalfWidth() {
  return pathSegments.map((segment) {
    try {
      return segment.convertFullWidthToHalfWidth();
    } on FormatException {
      return segment; // 数字以外のセグメントはそのまま返す
    }
  }).toList();
}
```

## 使用例とテストケース

### 基本的な使用例

```dart
void main() {
  // 基本的な変換
  print('１２３４５'.convertFullWidthToHalfWidth()); // "12345"
  
  // 混在パターン
  print('12３４5'.convertFullWidthToHalfWidth()); // "12345"
  
  // URI処理
  final uri = Uri.parse('/article/１２３');
  print(uri.containsFullWidthDigits()); // true
  print(uri.pathSegmentsFullWidthToHalfWidth()); // [article, 123]
}
```

### エラーケースの処理

```dart
try {
  'abc１２３'.convertFullWidthToHalfWidth();
} on FormatException catch (e) {
  print('エラー: ${e.message}'); // "abc１２３: a is not a number"
}
```

## まとめ

本記事で紹介した全角数字変換メソッドは、以下の特徴を持ちます：

- **高パフォーマンス**: 直接的な文字コード操作による効率的な処理
- **安全性**: 厳密なエラーチェックによる予期しない動作の防止
- **拡張性**: Dart Extensionによる`String`と`Uri`クラスの自然な拡張

日本語を扱うFlutterアプリケーションにおいて、このような文字変換処理は必須の機能です。
`Unicode`文字コードの仕組みを理解することで、より堅牢で効率的な国際化対応アプリケーションを構築できるでしょう。