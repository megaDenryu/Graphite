//! `chain` サブコマンドの表示と、社員キーが見つからないときの案内。

use crate::analysis::ChainResult;
use crate::schema::EmployeeId;

pub fn print_chain(result: &ChainResult) {
    println!("=== 管理チェーン ===");
    for entry in &result.entries {
        let indent = "  ".repeat(entry.depth);
        match entry.since {
            Some(since) => println!(
                "{}└─ {} ({} / {}) [在任 {}年〜, 深さ{}]",
                indent, entry.name, entry.title, entry.employee.0, since, entry.depth
            ),
            None => println!(
                "{}{} ({} / {}) [起点, 深さ{}]",
                indent, entry.name, entry.title, entry.employee.0, entry.depth
            ),
        }
    }
    if let Some(back_to) = &result.cycle_back_to {
        println!(
            "\n[警告] 循環を検出したため打ち切りました (社員 {} まで戻っています)",
            back_to.0
        );
    } else {
        println!("\nトップ層に到達しました (これ以上の上司なし)");
    }
}

/// `main.rs` から使う小ヘルパー: 社員キーが存在するかどうかの案内。
pub fn print_unknown_employee(key: &EmployeeId) {
    eprintln!("エラー: 社員キー '{}' は存在しません", key.0);
}
