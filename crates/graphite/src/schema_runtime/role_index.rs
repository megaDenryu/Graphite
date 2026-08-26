//! 役割ごとの辺位置を多重度別に引く索引を、多重度3種ぶんまとめて所有する。
//!
//! 多重度なし・多重度1・多重度0..1 の3型は「バケット列から作り、内部位置で引く」という
//! 同じ契約の3実装であり、多重度の違いが返り値の形 (`&[P]` / `&P` / `Option<&P>`) に
//! そのまま現れる。1画面で見比べられるように1ファイルへ置く。

use std::ops::Range;

/// 多重度制約のない役割索引を、役割ごとの範囲と連続した辺位置列で保持する。
#[doc(hidden)]
pub struct MultipleRoleIndex<P> {
    ranges: Vec<Range<usize>>,
    positions: Vec<P>,
}

impl<P> MultipleRoleIndex<P> {
    #[doc(hidden)]
    pub fn from_buckets(buckets: Vec<Vec<P>>) -> Self {
        let mut positions = Vec::with_capacity(buckets.iter().map(Vec::len).sum());
        let ranges = buckets
            .into_iter()
            .map(|bucket| {
                let start = positions.len();
                positions.extend(bucket);
                start..positions.len()
            })
            .collect();
        Self { ranges, positions }
    }

    #[doc(hidden)]
    pub fn get(&self, position: usize) -> &[P] {
        self.ranges
            .get(position)
            .map(|range| &self.positions[range.clone()])
            .unwrap_or(&[])
    }
}

#[doc(hidden)]
pub struct ExactlyOneRoleIndex<P>(Vec<P>);

impl<P> ExactlyOneRoleIndex<P> {
    #[doc(hidden)]
    pub fn from_buckets(buckets: Vec<Vec<P>>) -> Self {
        Self(
            buckets
                .into_iter()
                .map(|mut bucket| {
                    assert_eq!(
                        bucket.len(),
                        1,
                        "多重度1の役割索引には各ノードの辺位置が1つ必要です"
                    );
                    bucket.pop().expect("長さを検査済みです")
                })
                .collect(),
        )
    }

    #[doc(hidden)]
    pub fn get(&self, position: usize) -> &P {
        &self.0[position]
    }
}

#[doc(hidden)]
pub struct OptionalRoleIndex<P>(Vec<Option<P>>);

impl<P> OptionalRoleIndex<P> {
    #[doc(hidden)]
    pub fn from_buckets(buckets: Vec<Vec<P>>) -> Self {
        Self(
            buckets
                .into_iter()
                .map(|mut bucket| {
                    assert!(
                        bucket.len() <= 1,
                        "多重度0..1の役割索引には辺位置を高々1つだけ格納できます"
                    );
                    bucket.pop()
                })
                .collect(),
        )
    }

    #[doc(hidden)]
    pub fn get(&self, position: usize) -> Option<&P> {
        self.0.get(position).and_then(Option::as_ref)
    }
}
