//! FSM が受理するイベントの列挙と、その表示。

use std::fmt;

/// FSM が受理するイベント一覧。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    Submit,
    Pay,
    Ship,
    Deliver,
    Cancel,
    Refund,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Event::Submit => "submit",
            Event::Pay => "pay",
            Event::Ship => "ship",
            Event::Deliver => "deliver",
            Event::Cancel => "cancel",
            Event::Refund => "refund",
        };
        write!(f, "{s}")
    }
}
