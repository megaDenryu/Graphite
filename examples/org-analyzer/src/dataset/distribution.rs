//! 擬似乱数から、現実の組織らしい偏りを持つ値を引く分布。

use super::lcg::Lcg;

/// grade 分布 (1〜5)。現場の人数が多いピラミッド型組織を模す。
pub(super) fn weighted_grade(rng: &mut Lcg) -> u8 {
    let roll = rng.next_range(100);
    match roll {
        0..=39 => 1,
        40..=64 => 2,
        65..=84 => 3,
        85..=94 => 4,
        _ => 5,
    }
}

/// 1人あたりの兼務プロジェクト数 (0〜3)。
pub(super) fn weighted_assignment_count(rng: &mut Lcg) -> usize {
    let roll = rng.next_range(100);
    match roll {
        0..=29 => 0,
        30..=69 => 1,
        70..=89 => 2,
        _ => 3,
    }
}
