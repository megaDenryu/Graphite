//! §3「一覧する」— 種別ごとの全キー・全辺・本数を読む。
//!
//! `{type}_ids` / `{kind}_iter` / `{kind}_len` の3系統が、ノードでもエッジでも、
//! 積み荷の有無によらず同じ形で生えていることを確かめる。

use crate::Org;

// やりたいこと: `g.{type}_ids()` でノード種別ごとの全キーを列挙する。
pub fn person_idsで全ノードキーを列挙する(g: &Org::Graph) {
    for id in g.person_ids() {
        println!("(一覧) person_ids: {id:?}");
    }
}

pub fn team_idsで全ノードキーを列挙する(g: &Org::Graph) {
    for id in g.team_ids() {
        println!("(一覧) team_ids: {id:?}");
    }
}

// やりたいこと: `g.kind_iter()` は `{Kind}Ref` を返す。積み荷なしエッジの例。
pub fn belongs_toのiterで制約ありエッジを列挙する(g: &Org::Graph) {
    for edge in g.belongs_to_iter() {
        println!(
            "(iter) BelongsTo {:?}: {:?} -> {:?}",
            edge.id(),
            edge.member().id(),
            edge.team().id()
        );
    }
}

// やりたいこと: 積み荷ありエッジの `iter()` も同じ形。`edge.payload()` で積み荷を読む。
pub fn bossのiterで積み荷ありエッジを列挙する(g: &Org::Graph) {
    for edge in g.boss_iter() {
        println!(
            "(iter) Boss {:?}: {:?} -> {:?} (since={})",
            edge.id(),
            edge.subordinate().id(),
            edge.superior().id(),
            edge.payload().since
        );
    }
}

// やりたいこと: `g.kind_len()` で表の辺の本数を確認する。
pub fn lenで表の辺の本数を確認する(g: &Org::Graph) {
    println!("(len) g.belongs_to_len() = {}", g.belongs_to_len());
    println!(
        "(len) g.reviewed_by_len() = {} (制約なしは平行辺込みの総本数)",
        g.reviewed_by_len()
    );
}
