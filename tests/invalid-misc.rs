extern crate toml;

#[test]
fn bad() {
    fn bad(s: &str) {
        assert!(s.parse::<toml::Value>().is_err());
    }

    bad("a = 01");
    bad("a = 1__1");
    bad("a = 1_");
    bad("''");
}
