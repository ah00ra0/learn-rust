//! کتابخانه‌ی کوچک textkit برای نمایش workspace و feature ها.

/// آمار ساده‌ی یک متن.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Stats {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
}

/// آمار متن را حساب می‌کند.
pub fn stats(text: &str) -> Stats {
    Stats {
        chars: text.chars().count(),
        words: text.split_whitespace().count(),
        lines: text.lines().count(),
    }
}

/// کلمه‌ها را به ترتیب معکوس برمی‌گرداند.
pub fn reverse_words(text: &str) -> String {
    text.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_unicode_chars() {
        let s = stats("سلام دنیا\nRust");
        assert_eq!(s, Stats { chars: 14, words: 3, lines: 2 });
    }

    #[test]
    fn reverses() {
        assert_eq!(reverse_words("a b  c"), "c b a");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn stats_serialize_with_feature() {
        let json = serde_json::to_string(&stats("hi there")).unwrap();
        assert_eq!(json, r#"{"chars":8,"words":2,"lines":1}"#);
    }
}
