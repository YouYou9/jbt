# Japanese Bible Tool
## 概要
聖書をcoeiroinkで朗読させます。
## 使用方法
.\jbt.exe John 6 1 4016
引数：本 章 節 聖書のバージョン

聖書のバージョンは
https://www.bible.com/bible/1820/GEN
で表示される4つの数字です。
本の名前は最後の3の文字列です。

## 必須要件
＊YouVersion-API　　
＊COEIROINK

必須要件には、APIサーバーで接続します。ポート番号を意図的に変えると、使えなくなります。

## ビルド方法
rustで普通にビルドすればokです。