---
title: "Flutter WebをGitHub Pagesにデプロイする方法"
emoji: "🥳"
type: "tech"
topics: [Flutter, GitHubPages, GitHub]
published: true
---



(`flutter run`だとローカルホスト上に立っているため、


│   └── widget_test.dart
└── web
    ├── favicon.png
    ├── icons
    ├── index.html
    └── manifest.json
```

**build**ディレクトリに`web`ディレクトリが作成されます。この中にWebプラットフォームようにビルドされたアプリが入っています。


### baseタグの修正
build -> web -> index.htmlのbaseタグを修正します。
```html:index.html
<!-- before -->
<base href="/">

<!-- after -->
<base href="/<リポジトリ名>/">
```
[baseタグ](https://www.tohoho-web.com/html/base.htm)の解説は省きますが、この修正をしないと`404 NotFound`になるので注意してください。

### push
```bash
// docsディレクトリにビルドファイルを移動
mv build/web ../docs

git add .
<以下省略>
```

`push`するときにビルドファイルは`root`ディレクトリ直下の**docs**ディレクトリに移動させてから`push`しましょう。  
GitHub Pagesでは`root`ディレクトリもしくは**docs**ディレクトリの内容をデプロイするため、プロジェクトファイルとセットのリポジトリで管理するためには**docs**ディレクトリにビルドファイルを移動させる必要があります。

以上でローカルでの作業は終了です。

## GitHub 上での作業編
### GitHub Pagesの設定
プロジェクトを管理しているリポジトリを開きます。
![](https://storage.googleapis.com/zenn-user-upload/e385deb74d59-20221015.png)
1. `Settings`から`Pages`を選択します。
2. `Build and deployment`の内容を変更
   1. `Source`を`Deploy fron a branch`に変更
   2. `Branch`を作業中のブランチに変更し、`/docs`を指定
3. `Save`をクリック
4. (数分待つ)
5. `Visit site`をクリック
6. (無事にアプリが表示されたら成功です)

![](https://storage.googleapis.com/zenn-user-upload/b2fbe373e7af-20221015.png)
(GitHub Pagesにホスティングされているため、URLは`https://<GitHubのユーザー名>.github.io/<リポジトリ名>/#/`になっていますね)
