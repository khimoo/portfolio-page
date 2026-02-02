---
title: Typstで履歴書とスキルシート
home_display: false
importance: 2
tags:
  - 雑
  - 個人開発
  - プログラミング
  - 趣味
created_at: "2026-02-03T06:02:55Z"
updated_at: "2026-02-03T12:14:10Z"
---
[リポジトリ](https://github.com/khimoo/typst-resume)

# Typstのyaml機能を使ってみたかった
typstファイルから相対ディレクトリでyamlファイルを参照できる。
```typst
#let data = yaml("data.yaml")
```
こんな感じでyamlファイルを基に辞書が作れる。

```typst
#let calculate-age(birth-year, birth-month, birth-day) = {
  let today = datetime.today()
  let birth-date = datetime(year: birth-year, month: birth-month, day: birth-day)

  let age = today.year() - birth-date.year()

  // 誕生日がまだ来ていない場合は1歳引く
  if today.month() < birth-date.month() or (
    today.month() == birth-date.month() and today.day() < birth-date.day()
  ) {
    age = age - 1
  }

  age
}
```
こんな感じでユーティリティ関数を作れば、生年月日から自動で年齢を計算できる。

# feature
- [ ] CIで自動でpdfにしたい