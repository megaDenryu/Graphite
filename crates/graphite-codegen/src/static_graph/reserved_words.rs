// `static_graph`が定義する固定語彙の生の綴り。`literal`(instanceの構文検証)
// と`trace`(生成名・意味カード) の両方がこの綴りを読むが、`literal`は層の
// 向き (`static_graph`のdoc参照) により`trace`/`naming`/`semantic`へ依存
// できないため、両者より下位のこのファイルへ綴りを1箇所へ集める。

// 個体の具象参照が実体を取り出す固定語彙のメソッド名 (`{個体}Ref::entity()`)。
// instanceが同名の具体辺を宣言すると、脱糖後に同じ名前の生成物と衝突する
// (`literal::validate`が検出する)。
pub(crate) const 実体アクセサ名: &str = "entity";
