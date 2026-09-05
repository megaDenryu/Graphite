//! 辺種別1つ分について、生成コードが使う識別子一式をまとめて持つ。
//!
//! このファイルは1ファイル100行の原則の例外である。`EdgeInfo` は辺種別1つ分の
//! 生成識別子をすべて束ねる構造体であり、フィールドごとの命名理由 (有向/無向の
//! 使い分け等) をコメントで残している。構造体・アクセサ・構築関数を分割
//! すると「この構造体が何を保持するか」が読めなくなるため、まとめて置いている。

use syn::Ident;

use crate::naming::{
    accessor_ident, duplicate_edge_key_variant_ident, edge_record_ident,
    incident_index_field_ident, internal_position_ident, named_position_ident, reference_ident,
    source_role_index_field_ident, target_role_index_field_ident,
    unique_pair_violation_variant_ident, unknown_endpoint_variant_ident,
    unknown_source_variant_ident, unknown_target_variant_ident,
};
use crate::schema::codegen::declaration_doc::宣言元への参照;
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::codegen::public_id_type::PublicIdType;
use crate::schema::semantic::{EachSide, RoleCardinality, 積み荷, 辺の向き, 辺定義};

// エッジ種別 1 つ分の、生成コードで使う識別子一式。
//
// 意味の問い合わせ (向き・多重度・端点対の重複可否) は `定義` へ委ね、この型は
// 生成名だけを持つ。`from_node`/`to_node` は `node_infos` (呼び出し元
// `generate_module_body` のローカル変数) への参照であり、両者の借用が同じ関数
// スコープに収まるよう単一のライフタイムパラメータで表現する。無向辺では
// `from_node`/`to_node` は常に同一の `NodeInfo` (両端同型) を指す。
//
// `宣言元への参照` だけは生成名ではないが、この種別の生成物すべてが同じ参照を doc へ
// 書くため、識別子と同じ場所で持つ。`accessor_ident` は `kind` が既に PascalCase
// (型名) であるため、ノードと同じ `to_snake_case` 変換で単数形 snake_case へ導出できる。
// `index_field_ident` と `to_index_field_ident` はどちらも凍結時に構築する
// (`docs/schema_v4.md` §3.2・`docs/reverse_query.md`)。前者は有向辺では始点を表す
// `{accessor}_from_index`、無向辺では方向の意味を持たないため `{accessor}_index` と
// する。後者は有向辺だけが持ち、終点役割クエリと入次数 each 検証の両方に使う
// (無向辺は `index_field_ident` が既に対称なので要らない)。
pub(crate) struct EdgeInfo<'a> {
    pub(crate) 定義: &'a 辺定義,
    pub(crate) kind: &'a Ident,
    pub(crate) 宣言元への参照: 宣言元への参照, // 生成物の doc へ足す `edge` 宣言元への参照
    pub(crate) id_ty: PublicIdType<'a>,        // エッジ種別の newtype キー型名 (`BossId`)
    pub(crate) accessor_ident: Ident, // 内部ストレージのフィールド名 = builder 追加メソッド名 (`boss`)
    pub(crate) index_field_ident: Ident, // 位置0キーからその辺のキー一覧を引く内部フィールド名
    pub(crate) to_index_field_ident: Ident, // 位置1キー (終点) から入る辺のキー一覧を引く内部フィールド名
    pub(crate) from_node: &'a NodeInfo<'a>,
    pub(crate) to_node: &'a NodeInfo<'a>,
}

impl<'a> EdgeInfo<'a> {
    pub(crate) fn shape(&self) -> &'a 辺の向き {
        self.定義.向き()
    }

    pub(crate) fn is_directed(&self) -> bool {
        self.定義.有向か()
    }

    pub(crate) fn payload(&self) -> Option<&'a 積み荷> {
        self.定義.積み荷()
    }

    pub(crate) fn unique_pair(&self) -> bool {
        self.定義.端点対の重複可否().対ごとに1本だけか()
    }

    // 指定した側の多重度。役割クエリの戻り型・索引の実装・凍結時の確定が
    // すべてこの1箇所を通る。
    pub(crate) fn cardinality(&self, side: EachSide) -> RoleCardinality {
        self.定義.側の多重度(side)
    }

    pub(crate) fn duplicate_key_variant(&self) -> Ident {
        duplicate_edge_key_variant_ident(self.kind)
    }
    pub(crate) fn unknown_source_variant(&self) -> Ident {
        unknown_source_variant_ident(self.kind)
    }
    pub(crate) fn unknown_target_variant(&self) -> Ident {
        unknown_target_variant_ident(self.kind)
    }
    // 無向辺用: 位置の区別が無いため未知端点は1種類の variant で足りる。
    pub(crate) fn unknown_endpoint_variant(&self) -> Ident {
        unknown_endpoint_variant_ident(self.kind)
    }

    pub(crate) fn unique_pair_violation_variant(&self) -> Ident {
        unique_pair_violation_variant_ident(self.kind)
    }

    pub(crate) fn internal_position_ident(&self) -> Ident {
        internal_position_ident(self.kind)
    }

    pub(crate) fn reference_ident(&self) -> Ident {
        reference_ident(self.kind)
    }

    pub(crate) fn record_ident(&self) -> Ident {
        edge_record_ident(self.kind)
    }

    pub(crate) fn named_position_ident(&self) -> Ident {
        named_position_ident(self.kind)
    }
}

// 辺定義から、その辺種別の生成に使う識別子一式を導出する。
//
// 端点のノードは意味モデルが確定済みのノード定義番号で持つため、ここでは同じ
// 並びで作った `node_infos` から取り出すだけで、名前の照合はしない。
pub(crate) fn build_edge_info<'a>(
    定義: &'a 辺定義,
    node_infos: &'a [NodeInfo<'a>],
    宣言元への参照: 宣言元への参照,
) -> EdgeInfo<'a> {
    let kind = 定義.辺種別名();
    let accessor = accessor_ident(kind);
    // 有向辺は始点側と終点側の2索引を持つため、位置を名前へ明示する。
    let index_field_ident = if 定義.有向か() {
        source_role_index_field_ident(&accessor)
    } else {
        incident_index_field_ident(&accessor)
    };
    // 無向辺では使わないが、無条件に計算しておいて差し支えない (単なる
    // Ident の合成であり、無向辺では単に参照されないだけ)。
    let to_index_field_ident = target_role_index_field_ident(&accessor);
    EdgeInfo {
        定義,
        kind,
        宣言元への参照,
        id_ty: PublicIdType::new(定義.公開id型()),
        accessor_ident: accessor,
        index_field_ident,
        to_index_field_ident,
        from_node: &node_infos[定義.始点のノード定義番号().添字()],
        to_node: &node_infos[定義.終点のノード定義番号().添字()],
    }
}
