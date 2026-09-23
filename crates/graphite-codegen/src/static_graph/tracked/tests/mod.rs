// issue #41 段階2の完了条件の回帰検査。フィクスチャ (schema入力・
// instance入力) を共有する2つの独立した検査に分ける (1ファイル100行の
// 原則。それぞれ別の完了条件を確かめる責務であり、この module本体は
// フィクスチャの組み立てだけを共有する):
// - `byte_equality`: 同じ入力から本文がバイト単位で一致すること。値の式の
//   種類 (struct式か関数呼び出しか) が本文を変えないこと (issue #41 是正3)。
// - `internal_leak`: C分類 (`__` で始まる内部生成識別子) が公開の追跡経路へ
//   漏れないこと (設計書 §6 の4、issue #41 是正6)。

mod byte_equality;
mod internal_leak;

use quote::quote;

fn schema入力() -> proc_macro2::TokenStream {
    quote! {
        generated = "generated/組織.rs";
        schema 組織 {
            node 社員;
            node 部署;
            edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
            edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
        }
    }
}

// 積み荷を持つ辺 (`上司`) の使用を含む (issue #41 是正6: C分類漏れ試験の
// 対象を広げる入力に積み荷ありの辺を足す)。
fn instance入力() -> proc_macro2::TokenStream {
    quote! {
        generated = "generated/開発チーム.rs";
        graph 開発チーム;
        node 太郎 = 社員 { 名前: "太郎".into() };
        node 次郎 = 社員 { 名前: "次郎".into() };
        node 開発部: 部署;
        edge 太郎の所属 = 所属(太郎 -> 開発部);
        edge 次郎の所属 = 所属(次郎 -> 開発部);
        edge 太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 2020 }]-> 次郎);
    }
}
