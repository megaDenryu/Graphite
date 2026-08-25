# 役割ベースの逆引きクエリ (Fudaba #5)

有向辺は始点・終点という固定語彙ではなく、スキーマに書いた役割名を両側で対称に使う。
生成される探索メソッドの実際の形と索引の型は `docs/desugaring_reference.md` §5・§18 にある。

```rust
let alice = graph.employee_by_id(&alice_id).unwrap();
let as_subordinate = alice.boss_as_subordinate(); // Option<BossRef<'_>>
let as_superior = alice.boss_as_superior();       // impl Iterator<Item = BossRef<'_>>
```

戻り値は相手ノードではなく常に `EdgeRef` である。相手端点は
`edge.superior()` / `edge.subordinate()`、積み荷は `edge.payload()` から辿る。
役割ごとの `each` 制約が戻り型を決める。

- `each role: 1`: `EdgeRef`
- `each role: 0..1`: `Option<EdgeRef>`
- その他または制約なし: `impl Iterator<Item = EdgeRef>`

無向辺は役割を捏造せず `NodeRef` のメソッド `{kind}_incident()` を提供し、
`EdgeRef::endpoints()` で両端を読む。

## 索引・計算量・確保

凍結時に役割ごとの索引と端点対索引を O(V + E) 時間・O(V + E) メモリで構築する。
完成後の役割クエリ (`{kind}_as_<role>`) は、内部位置をそのまま添字に使う `Vec` の
直接参照なのでハッシュ不要の O(1) である。`{kind}_between` は端点対ハッシュ索引を
検索するため平均 O(1) である。どちらも結果の走査は O(k)。複数件の役割索引は
辺位置を連続列と範囲で保持し、問い合わせ時に `Vec` を生成せず借用 iterator を
返す。結果順は辺の挿入順を保持する。

`a.{kind}_between(b)` は両参照が同じ `Graph` の値由来であることを検査する。
非パニック版 `a.{kind}_try_between(b) -> Result<_, graphite::GraphMismatch>` も提供する。
有向辺の対は順序付き、無向辺の対は順序なしである。
