---
title: "Nuxt3のディレクトリ構成 一覧"
emoji: "🗒️"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Nuxt3]
published: true
---

最近Nuxt3の勉強を始めたので、ディレクトリ構成を和訳しまとめました。
また、個人的にわかりづらいと思った部分のみ補足として使い方や注意点をまとめました。

# Nuxt3のディレクトリ構造一覧
- `.nuxt`: Nuxt.jsが開発モードで起動されたときに自動的に生成されるディレクトリ。ビルドされたアプリケーションのファイルが格納されます。
- `.output`: nuxt generateコマンドによって静的に生成されたサイトのファイルが出力されるディレクトリ。
- `assets`: スタイルシート、画像、フォントなどの静的なアセットを格納するディレクトリ。
- `components`: 再利用可能なVueコンポーネントを格納するディレクトリ。これらのコンポーネントは自動的にインポートされ、どこからでも使用できます。
- `composables`: Vue 3のComposition APIを使用した再利用可能なロジック（composable）を格納します。アプリケーション内で自動でimportされる。
- `content`: マークダウンファイルやCSVファイルなどのコンテンツを格納するディレクトリ。
- `layouts`: アプリケーションのレイアウトを定義するディレクトリ。レイアウトはページの共通部分（例えばヘッダーやフッター）を定義するために使用されます。
- `middleware`: アプリケーションのミドルウェアを定義するディレクトリ。ミドルウェアは、ページやレイアウトがレンダリングされる前に実行されるカスタム関数です。
- `modules`: Nuxt.jsのモジュールを格納するディレクトリ。モジュールは、Nuxt.jsのコア機能を拡張するためのコードを含むJavaScriptファイルです。
- `node_modules`: プロジェクトの依存関係を格納するディレクトリ。npm installまたはyarn installコマンドを実行すると、このディレクトリに依存関係がインストールされます。
- `pages`: ルーティングとページコンポーネントが自動的に生成されます。各Vueファイルは対応するルートとなります。
- `plugins`: Vue.jsのプラグインを格納するディレクトリ。これらのプラグインは、Nuxt.jsがインスタンス化される前に実行されます。
- `public`: 静的ファイル（画像、スタイルシート、ロボット.txtなど）を格納します。これらのファイルはそのままのパスで公開されます。
- `server`: サーバーサイドのロジックを格納します。このディレクトリ内に配置したファイルはサーバーサイドで実行され、APIエンドポイントとして利用できます。
- `utils`: ユーティリティ関数など、プロジェクト全体で使用するJavaScriptのヘルパー関数を格納します。

## ルートディレクトリに配置されるファイル
- `.env`: 環境変数を定義するファイル。このファイルは、公開されるべきではない秘密情報（APIキーなど）を保管するために使用されます。
- `.gitignore`: Gitバージョン管理から除外するファイルやディレクトリを指定するファイル。
- `.nuxtignore`: Nuxt.jsのビルドプロセスから特定のファイルを除外するためのファイル。
- `app.config.ts`: アプリケーションの設定を行うファイル。Nuxt.jsの設定をカスタマイズするために使用されます。
- `app.vue`: アプリケーションのルートコンポーネントを定義するファイル。このコンポーネントは、すべてのページコンポーネントの親となります。
- `nuxt.config.ts`: Nuxt.jsの設定を行うファイル。ビルド設定、モジュールの設定、プラグインの設定など、アプリケーション全体の設定をこのファイルで行います。
- `package.json`: プロジェクトの依存関係とスクリプトを定義するファイル。npm installまたはyarn installコマンドを実行すると、このファイルに記載された依存関係がインストールされます。
- `tsconfig.json`: TypeScriptのコンパイラオプションとプロジェクトのルートファイルを指定するファイル。このファイルは、TypeScriptを使用するNuxt.jsプロジェクトで必要です。

# 超個人的メモ
## components
UIコンポーネントを格納するディレクトリ。このディレクトリ内に配置したコンポーネントは、どこからでもインポートして使用することができる。

https://nuxt.com/docs/guide/directory-structure/components

:::details ファイル構成
```bash
components/
├── BaseTitle.vue
└── child
    └── ttile.vue
```
:::

```html: index.vue
<template>
    <h1> Hello World </h1>
    <BaseTitle>
        <h2> Hello World </h2>
        <h3> Hello World </h3>
        <h4> Hello World </h4>
    </BaseTitle>
    <ChildTtile>
        <h2> Hello World </h2>
        <h3> Hello World </h3>
        <h4> Hello World </h4>
    </ChildTtile>
</template>
```
![](https://storage.googleapis.com/zenn-user-upload/6c6246738f06-20231201.png)

https://github.com/r0227n/learn_nuxt_app/blob/main/explanation_init_app/components/BaseTitle.vue
https://github.com/r0227n/learn_nuxt_app/blob/main/explanation_init_app/components/child/ttile.vue

## layouts
アプリケーションのレイアウトを定義するし、ページはここからレイアウトを呼び出す。
https://nuxt.com/docs/guide/directory-structure/layouts

```html: app.vue
<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
```
`<NuxtLayout>`で囲まれた範囲がレイアウトの対象となる。`name="<ファイル名>"`を指定することにより、レイアウトを指定することができる。nameを指定しない場合は、`default.vue`が適用される。

## middleware
ルート(ページ)移動が完了する前に実行するプログラムを定義。
基本的に2種類あり、
- `xxx.global.ts`: ルート移動時、絶対呼び出される(以後「global」と呼称)
- xxx.ts: 移動先に定義していないと呼び出されない(以後、「無印」と呼称)

の2種類がある。
また優先度は`global` > `無印`となっている。

https://nuxt.com/docs/guide/directory-structure/middleware

### 基本的な書き方
#### middleware内でコードを実行する方法
```ts
export default defineNuxtRouteMiddleware((to, from) => {
  // to: 遷移元の情報
 // from: 遷移後の情報
})
```
を定義。

#### 無印のためにpage内で宣言する方法
```vue
<script setup lang="ts">
definePageMeta({
    middleware: [
        function (to, from) {
            // to: 遷移元の情報
           // from: 遷移後の情報
       },
        '<ファイル名>', // 「current.ts」なら、current
    ],
});
</script>
```

# まとめ
Nuxt3で意識しなければいけないディレクトリ構成について調べ、一覧化しました。
NuxtやVueは予約語が多く、覚えることが多いので、今後もキャッチアップしていきたいと思います。

以上、[バンドじゃないもん！MAXX NAKAYOSHI](https://banmon.jp/)の[Q.人生それでいいのかい？](https://www.youtube.com/watch?v=mesrQgR2Sy8)を聴きながらNextを勉強したいと強く思う[Ryo24](https://twitter.com/r0227n_)でした。
(筆者は執筆当時、Vueわからない状態でNuxt3のプロジェクトに参画し、仕事で必要に迫られて勉強しています。)