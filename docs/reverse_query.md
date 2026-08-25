# 役割ベースの逆引きクエリ (Fudaba #5)

有向辺は始点・終点という固定語彙ではなく、スキーマに書いた役割名を両側で対称に使う。

```rust
let alice = OrgChart::Employee::get(&graph, &alice_id).unwrap();
let as_subordinate = Boss::of_subordinate(alice); // Option<BossRef<'_>>
let as_superior = Boss::of_superior(alice);       // impl Iterator<Item = BossRef<'_>>

// 同じクエリを NodeRef からも機械的な名前で開始できる。
alice.boss_as_subordinate();
alice.boss_as_superior();
```

戻り値は相手ノードではなく常に `EdgeRef` である。相手端点は
`edge.superior()` / `edge.subordinate()`、積み荷は `edge.payload()` から辿る。
役割ごとの `each` 制約が戻り型を決める。

- `each role: 1`: `EdgeRef`
- `each role: 0..1`: `Option<EdgeRef>`
- その他または制約なし: `impl Iterator<Item = EdgeRef>`

無向辺は役割を捏造せず `incident(NodeRef)` を提供し、`EdgeRef::endpoints()` で
両端を読む。

## 索引・計算量・確保

凍結時に役割ごとの索引と端点対索引を O(V + E) 時間・O(V + E) メモリで構築する。
完成後の役割クエリ (`of_<role>`) は、内部位置をそのまま添字に使う `Vec` の
直接参照なのでハッシュ不要の O(1) である。`between` は端点対ハッシュ索引を
検索するため平均 O(1) である。どちらも結果の走査は O(k)。複数件の役割索引は
辺位置を連続列と範囲で保持し、問い合わせ時に `Vec` を生成せず借用 iterator を
返す。結果順は辺の挿入順を保持する。

`between(a, b)` は両参照が同じ `Graph` の値由来であることを検査する。
非パニック版 `try_between(a, b) -> Result<_, graphite::GraphMismatch>` も提供する。
有向辺の対は順序付き、無向辺の対は順序なしである。
