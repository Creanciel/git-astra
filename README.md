# git-astra

ながい旅の彼方にゆきつく場所 ——

## About

git clone コマンドのラッパー

複数のgitアカウントで運用するときの

- リポジトリ
-  `git config` の `user`

を考慮しつつcloneします。

## How to

基本的には `git clone` のラッパーであるため従来

```sh
git clone git@~
```

としていたところを

```sh
git astra git@~
```

と使います。

`http` cloneはサポートしていません。通常の `git clone` を使用してください。


## Install

1. ビルド

    cargo が必要

    ```
    ./astra.sh build
    ```

2. インストール

    ```
    sudo ./astra.sh install
    ```

## Settings

設定ファイルが `~/.config/git-astra/config.json` から情報を取得します。存在しない時は自動で生成されます。

以下のようにリポジトリごとに記載してください。

`host` と `owner` をキーとして設定をマッチングしています。`id` 以外の項目は必須フィールドとなっています。

`owner` とはリポジトリオーナーのことを指します。
通常のアカウントでしたら自分のアカウント名ですがエンタープライズのときはその組織のリポジトリ名になります。
つまり自分のリポジトリか企業のリポジトリで `user` 情報を切り替えられます。

```json
{
    "list": [
        {
            "host": "github.com",
            "owner": "Creanciel",
            "id": "creanciel",
            "user": {
                "name": "Creanciel",
                "mail": "creanciel@example.com"
            }
        },
        {
            ...
        }
    ]
}
```


`id` は ssh のアドレスとして使われます。 ssh_config のHOSTに当たる部分で使われるものとそろえてください。

```config
Host <ASTRA_ID_1>.github.com
    HostName: github.com
    User: git
    IdentityFile: <SECRET_KEY_PATH_1>

Host <ASTRA_ID_2>.github.com
    HostName: github.com
    User: git
    IdentityFile: <SECRET_KEY_PATH_2>
```
