//! 線形合同法による擬似乱数生成器。

// Numerical Recipes 系の定数を使った線形合同法 (LCG)。
// `state_{n+1} = state_n * A + C (mod 2^64)`。外部乱数クレート禁止という
// 制約のもとで「同じ seed なら同じ組織になる」再現性だけを目的にした最小実装
// であり、暗号用途などの品質は求めていない。
pub(super) struct Lcg {
    state: u64,
}

impl Lcg {
    pub(super) fn new(seed: u64) -> Self {
        // seed=0 だと初期状態が単調になりやすいので撹拌しておく。
        Self {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    pub(super) fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    // `[0, n)` の一様乱数。上位ビットを使うことで LCG 下位ビットの周期性の
    // 影響を避ける。
    pub(super) fn next_range(&mut self, n: usize) -> usize {
        debug_assert!(n > 0);
        ((self.next_u64() >> 33) % n as u64) as usize
    }

    // `[lo, hi]` (両端含む) の一様乱数。
    pub(super) fn next_range_inclusive(&mut self, lo: i64, hi: i64) -> i64 {
        lo + self.next_range((hi - lo + 1) as usize) as i64
    }

    // `numerator / denominator` の確率で `true`。
    pub(super) fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.next_range(denominator) < numerator
    }
}
