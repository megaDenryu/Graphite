//! 辺宣言を意味として確定した定義と、その向き・端点・積み荷を持つ。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
//! ファイルは150行を超える。このファイルは辺の向き・有向端点・積み荷を1つの
//! 辺定義へ統合している。責務の量の判定は issue #28 のやること4 が行う。超
//! 過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。
//!
//! 宣言の正規形の綴りを組み立てるメソッドは、辺定義の読み取りとは別の関心なので
//! [`super::edge_declaration_spelling`] にある。

use proc_macro2::Ident;
use syn::Path;

use super::cardinality::{
    each制約が指す端点の側を判定する, EachSide, RoleCardinality, 役割の多重度制約,
};
use super::endpoint_pairing::{端点対のキーの形, 端点対の重複可否};
use super::node_definition::ノード定義番号;
use super::public_id_type::公開ID型;
use crate::naming::generated_id_ident;
use crate::schema::syntax::{EdgeDecl, EdgePayload, EdgeShape};

// スキーマ定義が持つ辺定義の列の中の1件を指すハンドル。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct 辺定義番号(usize);

impl 辺定義番号 {
    pub(super) fn 添字から作る(添字: usize) -> Self {
        Self(添字)
    }

    // 辺定義の列から要素を取り出すための添字へ戻す。
    //
    // 注意: 生の添字へ戻してよいのは、辺定義の列 (またはそれと同じ順で作った列)
    // から取り出す場所だけである。
    pub fn 添字(self) -> usize {
        self.0
    }
}

// 有向辺の端点1つ分。役割名と、その役割が繋がるノード定義を持つ。
pub struct 有向端点 {
    役割名: Ident,
    ノード: ノード定義番号,
}

impl 有向端点 {
    pub(super) fn 役割名とノードから作る(
        役割名: Ident, ノード: ノード定義番号
    ) -> Self {
        Self {
            役割名, ノード
        }
    }

    pub fn 役割名(&self) -> &Ident {
        &self.役割名
    }

    pub fn ノード(&self) -> ノード定義番号 {
        self.ノード
    }
}

// 辺が向きを持つかどうかと、その端点。
//
// 無向辺の両端は同じノード型でなければならない
// (`docs/edge_endpoints_v4_1.md` §2、`schema::validate` が検証済み) ため、
// 端点は1つだけ持つ。
pub enum 辺の向き {
    有向 {
        始点: 有向端点, 終点: 有向端点
    },
    無向 {
        端点のノード: ノード定義番号
    },
}

// 辺が運ぶ利用者定義の値。役割名と、利用者が書いたままの型パスを持つ。
//
// 注意: 積み荷の型パスは公開ID型と違い `self::` → `super::` の読み替えを
// 受けていない。現行の挙動をそのまま保つ。
pub struct 積み荷 {
    役割名: Ident,
    型パス: Path,
}

impl 積み荷 {
    pub fn 役割名(&self) -> &Ident {
        &self.役割名
    }

    pub fn 型パス(&self) -> &Path {
        &self.型パス
    }
}

// 辺種別1つ分の意味。
pub struct 辺定義 {
    辺種別名: Ident,
    公開id型: 公開ID型,
    向き: 辺の向き,
    積み荷: Option<積み荷>,
    端点対の重複可否: 端点対の重複可否,
    記述順の役割の多重度制約: Vec<役割の多重度制約>,
    始点側の役割の多重度制約: Option<役割の多重度制約>,
    終点側の役割の多重度制約: Option<役割の多重度制約>,
}

impl 辺定義 {
    pub(super) fn 宣言と向きから作る(宣言: &EdgeDecl, 向き: 辺の向き) -> Self {
        let 記述順の役割の多重度制約: Vec<役割の多重度制約> = 宣言
            .constraints
            .each
            .iter()
            .map(|制約| {
                let 側 = each制約が指す端点の側を判定する(宣言, &制約.role)
                    .expect("validate_each_reference() を通過していれば必ず解決できるはず");
                役割の多重度制約::宣言と側から作る(制約, 側)
            })
            .collect();
        let 側の制約を探す = |側: EachSide| {
            記述順の役割の多重度制約
                .iter()
                .find(|制約| 制約.側() == 側)
                .cloned()
        };
        Self {
            辺種別名: 宣言.kind.clone(),
            公開id型: 公開ID型::宣言から作る(
                generated_id_ident(&宣言.kind),
                宣言.id_ty.clone(),
            ),
            積み荷: 宣言の積み荷を写す(&宣言.shape),
            端点対の重複可否: 端点対の重複可否::unique_pair指定から作る(
                宣言.constraints.unique_pair,
            ),
            始点側の役割の多重度制約: 側の制約を探す(EachSide::Source),
            終点側の役割の多重度制約: 側の制約を探す(EachSide::Target),
            記述順の役割の多重度制約,
            向き,
        }
    }

    pub fn 辺種別名(&self) -> &Ident {
        &self.辺種別名
    }

    pub fn 公開id型(&self) -> &公開ID型 {
        &self.公開id型
    }

    pub fn 向き(&self) -> &辺の向き {
        &self.向き
    }

    pub fn 有向か(&self) -> bool {
        matches!(self.向き, 辺の向き::有向 { .. })
    }

    pub fn 積み荷(&self) -> Option<&積み荷> {
        self.積み荷.as_ref()
    }

    pub fn 端点対の重複可否(&self) -> 端点対の重複可否 {
        self.端点対の重複可否
    }

    // 端点対索引のキーの形。向きから決まる。
    pub fn 端点対のキーの形(&self) -> 端点対のキーの形 {
        if self.有向か() {
            端点対のキーの形::順序付きの対
        } else {
            端点対のキーの形::順序なしの対
        }
    }

    // 位置0側 (有向辺の始点、無向辺の唯一の端点型) のノード定義。
    pub fn 始点のノード定義番号(&self) -> ノード定義番号 {
        match &self.向き {
            辺の向き::有向 { 始点, .. } => 始点.ノード(),
            辺の向き::無向 { 端点のノード } => *端点のノード,
        }
    }

    // 位置1側 (有向辺の終点、無向辺の唯一の端点型) のノード定義。
    pub fn 終点のノード定義番号(&self) -> ノード定義番号 {
        match &self.向き {
            辺の向き::有向 { 終点, .. } => 終点.ノード(),
            辺の向き::無向 { 端点のノード } => *端点のノード,
        }
    }

    // DSL の `where` 節に書かれた順の多重度制約。違反 variant の並び順が
    // この順に従うため、側ごとの取り出しとは別に保持する。
    pub fn 記述順の役割の多重度制約(&self) -> &[役割の多重度制約] {
        &self.記述順の役割の多重度制約
    }

    pub fn 側の役割の多重度制約(
        &self,
        側: EachSide,
    ) -> Option<&役割の多重度制約> {
        match 側 {
            EachSide::Source => self.始点側の役割の多重度制約.as_ref(),
            EachSide::Target => self.終点側の役割の多重度制約.as_ref(),
        }
    }

    // 指定した側の多重度。制約が書かれていない側は `Multiple` になる。
    pub fn 側の多重度(&self, 側: EachSide) -> RoleCardinality {
        self.側の役割の多重度制約(側)
            .map_or(RoleCardinality::Multiple, 役割の多重度制約::多重度)
    }
}

// 辺宣言の柄に書かれた積み荷を意味モデルへ写す。有向・無向で同じ形なので
// 向きを確定する前に取り出せる。
fn 宣言の積み荷を写す(shape: &EdgeShape) -> Option<積み荷> {
    let payload: &Option<EdgePayload> = match shape {
        EdgeShape::Directed { payload, .. } | EdgeShape::Undirected { payload, .. } => payload,
    };
    payload.as_ref().map(|payload| 積み荷 {
        役割名: payload.role.clone(),
        型パス: payload.ty.clone(),
    })
}
