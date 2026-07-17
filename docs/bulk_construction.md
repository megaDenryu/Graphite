# 一括構築 API — 実行時データからの宣言的構築 (Fudaba #9)

2026-07-18 オーケストレータ決定 (ユーザーから全面委任)。#9 の選択肢 (b) を採用。

## 問題

graph! リテラルは静的な図示には完璧だが、実行時データ (合成生成器・CSV 等) からの
構築は builder + for ループしかなく、グラフの形が制御フローに埋まる
(org-analyzer/dataset.rs、async-dag/fixtures.rs)。「三大敵」で倒したはずの敵が
構築コードに残る自己矛盾。

## 決定: builder に一括挿入を追加 (選択肢 b)

既存の要素単位 API (`insert` / 辺の追加) の**イテレータ版**を足すだけ。
ループと変換は普通の Rust (map 等) に寄せ、グラフ構築は 1〜2 呼び出しに集約する。

```rust
let g = Org::create(|b| {
    b.extend_nodes(people.into_iter().map(|p| (p.code.clone(), p)));
    b.extend_edges(pairs.into_iter().enumerate()
        .map(|(i, (a, c))| (format!("dep{i}"), DependsOn(a, c))));
});
```

仕様:

- `extend_nodes(impl IntoIterator<Item = (K, N)>)` — K: Into<String>、N は既存の
  {Schema}Node trait 境界 (insert と同じ)。戻り値 `Vec<N::Id>` (挿入順)
- `extend_edges(impl IntoIterator<Item = (K, E)>)` — E は既存の辺追加と同じ trait
  経路。戻り値 `Vec<E::Id>` (挿入順)
- 意味論は要素単位 API の反復と完全に同一 (重複キー・検証は freeze 時に従来どおり)。
  挿入順保持もそのまま
- 命名は原則3 (std の Extend::extend に倣う) の範囲で実装時に微調整可
- graph! へのスプライス構文 (選択肢 c) は本仕様の糖衣として将来足せるため今回は
  見送り (Fudaba #9 に記録)

## 実証 (完了条件)

org-analyzer/dataset.rs と async-dag/fixtures.rs の構築ループを extend 形に
書き直し、for ループが「データを作る部分」だけに縮退することを示す。
既存テストは全通過のこと。
