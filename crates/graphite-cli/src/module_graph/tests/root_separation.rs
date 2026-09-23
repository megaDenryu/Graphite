// `lib.rs`・`main.rs`・`bin/*.rs`がそれぞれ別のCargo targetになり、互いの
// `mod`木を混ぜないことの単体試験 (`module_graph`本体の`ルート一覧を求める`
// が対象)。

use super::一時srcで試す;

#[test]
fn libとmainは別targetになる() {
    一時srcで試す(
        &[("lib.rs", "pub fn 甲() {}"), ("main.rs", "fn main() {}")],
        |dir, 結果| {
            let 表 = 結果.unwrap();
            let lib_target = 表.get(&dir.join("lib.rs")).unwrap();
            let main_target = 表.get(&dir.join("main.rs")).unwrap();
            assert_ne!(lib_target, main_target);
        },
    );
}

#[test]
fn libが辿るmodはlibのtargetに属する() {
    一時srcで試す(
        &[
            ("lib.rs", "mod domain; fn main() {}"),
            ("domain.rs", "pub struct 社員;"),
            ("main.rs", "fn main() {}"),
        ],
        |dir, 結果| {
            let 表 = 結果.unwrap();
            let lib_target = 表.get(&dir.join("lib.rs")).unwrap();
            let domain_target = 表.get(&dir.join("domain.rs")).unwrap();
            assert_eq!(lib_target, domain_target);
            let main_target = 表.get(&dir.join("main.rs")).unwrap();
            assert_ne!(lib_target, main_target);
        },
    );
}

#[test]
fn mainだけが辿るmodはmainのtargetに属しlibには属さない() {
    一時srcで試す(
        &[
            ("lib.rs", "pub fn 甲() {}"),
            ("main.rs", "mod scenario; fn main() {}"),
            ("scenario.rs", "pub fn 実行() {}"),
        ],
        |dir, 結果| {
            let 表 = 結果.unwrap();
            let main_target = 表.get(&dir.join("main.rs")).unwrap();
            let scenario_target = 表.get(&dir.join("scenario.rs")).unwrap();
            assert_eq!(main_target, scenario_target);
            let lib_target = 表.get(&dir.join("lib.rs")).unwrap();
            assert_ne!(lib_target, scenario_target);
        },
    );
}

#[test]
fn binの各ファイルは別々のtargetになる() {
    一時srcで試す(
        &[
            ("lib.rs", "pub fn 甲() {}"),
            ("bin/a.rs", "fn main() {}"),
            ("bin/b.rs", "fn main() {}"),
        ],
        |dir, 結果| {
            let 表 = 結果.unwrap();
            let a = 表.get(&dir.join("bin/a.rs")).unwrap();
            let b = 表.get(&dir.join("bin/b.rs")).unwrap();
            let lib = 表.get(&dir.join("lib.rs")).unwrap();
            assert_ne!(a, b);
            assert_ne!(a, lib);
        },
    );
}
