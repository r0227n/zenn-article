---
title: "Flutter WebをGitHub Pagesにデプロイする方法"
emoji: "🥳"
type: "tech"
topics: [Flutter, GitHubPages, GitHub]
published: true
---



(`flutter run`だとローカルホスト上に立っているため、




```

**build**ディレクトリに`web`ディレクトリが作成されます。この中にWebプラットフォームようにビルドされたアプリが入っています。

<!-- before -->
.com/html/base.ht

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
