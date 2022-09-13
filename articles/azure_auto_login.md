---
title: "Azure自動ログイン方法"
emoji: "🔐"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Azure]
published: true
---

CLIでAzureにログインするとき、ブラウザでアカウントを選択する必要があります。
![ログイン時にブラウザ認証が必要](https://storage.googleapis.com/zenn-user-upload/0431bf1799b8-20220911.jpg)
手動で作業しているときは特に問題ないのですが、`Pipeline`や`Terraform`の**null_resouce**で実行するときには面倒のため、自動でログインできるようにしたいと思います。

ブラウザ認証をスキップしログインする方法として
- サービスプリンシパルを利用したログイン方法
- マネージドIDを利用したログイン方法

上記2種類の方法でログインすることができます。
今回は`az command`と`AzModule`の両方でログインできる方法を紹介します。


## az login
```txt az commandのログイン方法
az login --service-principal --username <sp_id> --password <sp_secret> --tenant <tenant_id>
```



## Connect-AzAccount
```txt Connet-AzAccountのログイン方法
$ApplicationId = <sp_id>
$SecuredPassword = ConvertTo-SecureString <sp_secret> -AsPlainText -Force
$Credential = New-Object -TypeName System.Management.Automation.PSCredential -ArgumentList $ApplicationId, $SecuredPassword
$TenantId = <tenant_id>
Connect-AzAccount -ServicePrincipal -TenantId $TenantId -Credential $Credential
```



## マネージドIDでログイン
VM(Automation, Functionsなど)に割り振られたマネージドIDでログインします。
```txt managed identityのログイン方法
az login --identity
Connect-AzAccount -Identity
```
注意: マネージドIDに適切なロールが割り振られていない場合、VM内での処理中に権限不足によるエラーが発生するためロールの割り当てをしっかりしてあげましょう。