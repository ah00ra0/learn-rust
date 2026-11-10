//! ابزارهای کوچک برای کار با متن فارسی/عربی و یونیکد.
//! این کد در فصل «رشته‌ها و یونیکد در عمق» کتاب توضیح داده شده است.

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

/// نیم‌فاصله (ZWNJ)
pub const ZWNJ: char = '\u{200c}';

/// تعداد بایت، کاراکتر (scalar value) و grapheme cluster
pub fn counts(s: &str) -> (usize, usize, usize) {
    (s.len(), s.chars().count(), s.graphemes(true).count())
}

/// حروف عربی «ي» و «ك» را به معادل فارسی «ی» و «ک» تبدیل می‌کند.
pub fn unify_persian_letters(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\u{064A}' | '\u{0649}' => '\u{06CC}', // ي ى → ی
            '\u{0643}' => '\u{06A9}',              // ك → ک
            other => other,
        })
        .collect()
}

/// ارقام فارسی (۰-۹) و عربی-هندی (٠-٩) را به ارقام ASCII تبدیل می‌کند.
pub fn to_ascii_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\u{06F0}'..='\u{06F9}' => char::from(b'0' + (c as u32 - 0x06F0) as u8),
            '\u{0660}'..='\u{0669}' => char::from(b'0' + (c as u32 - 0x0660) as u8),
            other => other,
        })
        .collect()
}

/// نیم‌فاصله‌ها را برمی‌دارد.
pub fn strip_zwnj(s: &str) -> String {
    s.chars().filter(|&c| c != ZWNJ).collect()
}

/// نرمال‌سازی برای مقایسه و جستجو: NFC + یکسان‌سازی حروف + ارقام ASCII.
pub fn normalize_for_search(s: &str) -> String {
    let nfc: String = s.nfc().collect();
    to_ascii_digits(&unify_persian_letters(&nfc))
}

/// n grapheme اول رشته (برش امن برای نمایش)
pub fn take_graphemes(s: &str, n: usize) -> &str {
    match s.grapheme_indices(true).nth(n) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_differ_for_persian() {
        // «سلام»: ۴ حرف، هرکدام ۲ بایت
        assert_eq!(counts("سلام"), (8, 4, 4));
        // «e» + accent ترکیبی: ۲ کاراکتر ولی ۱ grapheme
        assert_eq!(counts("e\u{301}"), (3, 2, 1));
        // خانواده: ۵ کاراکتر (با ZWJ)، ۱ grapheme
        assert_eq!(counts("👨‍👩‍👧"), (18, 5, 1));
    }

    #[test]
    fn zwnj_sticks_to_previous_letter() {
        // «می‌خواهم»: ۸ کاراکتر ولی ۷ grapheme؛ نیم‌فاصله به حرف قبل می‌چسبد
        assert_eq!(counts("می\u{200c}خواهم"), (17, 8, 7));
    }

    #[test]
    fn arabic_and_persian_letters_differ_until_unified() {
        let arabic = "علي"; // ي عربی
        let persian = "علی"; // ی فارسی
        assert_ne!(arabic, persian);
        assert_eq!(unify_persian_letters(arabic), persian);
        assert_eq!(unify_persian_letters("كتاب"), "کتاب");
    }

    #[test]
    fn digits() {
        assert_eq!(to_ascii_digits("۱۲۳ و ٤٥٦"), "123 و 456");
        assert!("۱۲۳".parse::<i32>().is_err());
        assert_eq!(to_ascii_digits("۱۲۳").parse::<i32>(), Ok(123));
    }

    #[test]
    fn zwnj_is_not_whitespace() {
        assert!(!ZWNJ.is_whitespace());
        assert_eq!("  x\u{200c} ".trim(), "x\u{200c}");
        assert_eq!(strip_zwnj("می\u{200c}خواهم"), "میخواهم");
    }

    #[test]
    fn normalization() {
        // NFD و NFC یک متن را متفاوت کدگذاری می‌کنند ولی یکسان نمایش می‌دهند
        let composed = "\u{e9}"; // é
        let decomposed = "e\u{301}";
        assert_ne!(composed, decomposed);
        assert_eq!(normalize_for_search(decomposed), composed);
    }

    #[test]
    fn safe_truncate() {
        assert_eq!(take_graphemes("e\u{301}a", 1), "e\u{301}");
        assert_eq!(take_graphemes("abc", 10), "abc");
        assert_eq!(take_graphemes("می\u{200c}خواهم", 2), "می\u{200c}");
    }
}
