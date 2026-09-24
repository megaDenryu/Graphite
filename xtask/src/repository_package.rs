use std::path::PathBuf;

use graphite_cli::{GenerationTree, PackageRoot};

// リポジトリの中にある、生成の対象になる1つの cargo パッケージ。
//
// 走査開始点の決め方そのものは外部crate向けの入口と同じ `PackageRoot` へ委ね、
// リポジトリルートからの綴り (表示用) だけをこの型が足す。
//
// 注意: 基準ディレクトリをパッケージルートへ揃えることが、この型を挟む唯一の
// 目的である。基準をリポジトリルートにすると、生成ファイルの2行目へ書く宣言元の
// 綴りが `cargo graphite generate` の書くものと食い違い、同じリポジトリを両方の
// 入口で処理したときに互いを「古い」と判定し合う。
pub struct RepositoryPackage {
    spelling: String,
    root: PackageRoot,
}

impl RepositoryPackage {
    pub(crate) fn new(spelling: String, root: PackageRoot) -> Self {
        Self { spelling, root }
    }

    // リポジトリルートからの綴り。どのパッケージを処理しているかの表示に使う。
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    // 生成の中核へ渡す走査対象。抽出・計画・検査は `graphite-cli` が行う。
    pub fn generation_tree(&self) -> &GenerationTree {
        self.root.generation_tree()
    }

    // このパッケージの Cargo.toml のパス。
    //
    // この関数は `PackageRoot::manifest_path` へ委譲し、綴りの組み立てを1箇所に
    // 保つ。doc コメント検査は、この口を使って各パッケージの Cargo.toml を読む
    // (`PackageManifestFacts::read`)。
    pub fn manifest_path(&self) -> PathBuf {
        self.root.manifest_path()
    }
}
