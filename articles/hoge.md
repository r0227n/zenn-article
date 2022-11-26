---
title: "Flutter WebをGitHub Pagesにデプロイする方法"
emoji: "🥳"
type: "tech"
topics: [Flutter, GitHubPages, GitHub]
published: true
---

おはこんにちは〜  
Flutterでモバイル以外のアプリ開発をしたいと考えている[Ryo24](https://twitter.com/r0227n_)です。  
GitHub Pages初学者の私がFlutter Webで作ったアプリをデプロイする方法をメモとして残しておきます。

# Flutter WebでGithub Pagesにデプロイする
## ローカルでの作業編
### プロジェクトを作成
```bash
// プラットフォームをWebに設定し、プロジェクトを作成
flutter create <プロジェクト名> --platforms web
// プロジェクトディレクトリに移動
cd  <プロジェクト名>
// プロジェクトを実行
flutter run
```

![](https://storage.googleapis.com/zenn-user-upload/91aec3810a1f-20221015.png)

いつもの画面が表示されましたね。今回はこのページをデプロイします。
(`flutter run`だとローカルホスト上に立っているため、`http://localhost:49535/#/`になります。)

### ビルド
```bash
flutter build web 
```
プラットフォームをWebに指定し、ビルドします。そうすると
```
.
├── README.md
├── analysis_options.yaml
├── build
│   ├── c075001b96339384a97db4862b8ab8db.cache.dill.track.dill
│   ├── flutter_assets
│   └── web
├── flutter_web.iml
├── lib




`push`するときにビルドファイルは`root`ディレクトリ直下の**docs**ディレクトリに移動させてから`push`しましょう。  

以上でローカルでの作業は終了です。
容を変更

今回はGitHub上のGUIでデプロイ作業をしていましたが、`Actions`を使って自動デプロイをする方法もあります。  
`Actions`の使い方を理解次第、自動デプロイの方法＆解説記事を書いてみたいと思います。