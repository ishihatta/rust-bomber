# これは何？
Rust で書いた簡単な対戦2Dアクションゲームです。Windows, MacOS, Linux 等のデスクトップで動作し、キーボードで操作します。

[以前 Kotlin で書いたゲーム](https://github.com/ishihatta/kotlin-bomber/) の Rust への移植になります。Rust の習得のために作成しました。

# ゲームの内容
二人対戦専用のボン○ーマンです。人間対人間、人間対AI、AI対AIの対戦ができます。ルールは以下の通りです。

* アイテムは火力アップのみ
* 爆弾は無限に置ける
* 死んだら負け

# プレイ動画
## AI対AI

https://user-images.githubusercontent.com/40629744/229340824-487efd17-6290-441f-91f2-88b87e9acb5c.mp4

キャプチャ品質が悪くカクついていますが実際は60FPSでなめらかに動きます。

# ビルド方法
## ランタイムライブラリの導入
ゲームライブラリとして [SDL2](https://www.libsdl.org/) を使用しているため以下のランタイムライブラリをシステムにインストールしている必要があります。

* [SDL](https://github.com/libsdl-org/SDL)
* [SDL_mixer](https://github.com/libsdl-org/SDL_mixer)
* [SDL_ttf](https://github.com/libsdl-org/SDL_ttf)
* [SDL_image](https://github.com/libsdl-org/SDL_image)

たとえば Ubuntu では以下のコマンドでこれらのライブラリをインストールできます。

```sh
sudo apt install libsdl2-dev libsdl2-mixer-dev libsdl2-ttf-dev libsdl2-image-dev
```

## ビルド

```sh
cargo build
```

## 実行

```sh
cargo run
```

# 操作方法（キーアサイン）

|       | Player 1 | Player 2 |
|-------|----------|----------|
| 上に移動  | W        | カーソル上    |
| 右に移動  | D        | カーソル右    |
| 下に移動  | S        | カーソル下    |
| 左に移動  | A        | カーソル左    |
| 爆弾を置く | 1        | /        |

# 使用素材
## 画像
以下のサイトで無償配布されている画像を使わせていただいています。

* [ぴぽや倉庫](https://pipoya.net/sozai/)

## サウンド
以下のサイトで無償配布されている効果音およびBGMの音源を使わせていただいています。

* [DOVA-SYNDROME](https://dova-s.jp/)
