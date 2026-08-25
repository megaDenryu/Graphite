// `generated = "../evil.rs";` のように `generated/` の外・上位ディレクトリへ
// 逸脱する指定を拒否する診断を固定する。宣言元ディレクトリの外へ書き込める
// 経路を残さないための検査 (`graphite_codegen::validate_generated_relative_path`)。

struct Person;

graphite::graph_schema! {
    generated = "../evil.rs";
    schema Evil {
        node Person;
    }
}

fn main() {}
