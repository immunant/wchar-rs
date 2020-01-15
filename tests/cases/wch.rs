use wchar::wch;

macro_rules! test_wch {
    ($s:expr) => {
        assert_eq!(wch!($s), &*$s.encode_utf16().collect::<Vec<u16>>());
        assert_eq!(wch!(u16, $s), &*$s.encode_utf16().collect::<Vec<u16>>());
        assert_eq!(wch!(u32, $s), &*$s.chars().map(|x| x as u32).collect::<Vec<u32>>());
    };
}

fn main() {
    test_wch!("foo");
    test_wch!("bar");

    test_wch!("foo bar");

    test_wch!("foo\nbar");
    test_wch!("foo\r\nbar");

    test_wch!("foo\0 bar");
    test_wch!("foo bar\0");

    test_wch!(r#"foo\bar\"#);
    test_wch!(r#"foo "bar" baz"#);
}
