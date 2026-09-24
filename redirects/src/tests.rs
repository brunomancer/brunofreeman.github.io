use super::*;

#[test]
fn parses_forms_and_comments() {
    let text =
        "; comment\n\n(improv \"https://x.com/p?q=1&r=2#f\") ; trailing\n(b \"http://y.com\")";
    let parsed = parse(text).unwrap();
    assert_eq!(
        parsed,
        [
            Redirect {
                line: 3,
                slug: "improv".into(),
                url: "https://x.com/p?q=1&r=2#f".into()
            },
            Redirect {
                line: 4,
                slug: "b".into(),
                url: "http://y.com".into()
            },
        ]
    );
}

#[test]
fn escapes_ampersands_in_attributes_only() {
    let html = page("https://x.com/?a=1&b=2");
    assert!(html.contains(r#"href="https://x.com/?a=1&amp;b=2""#));
    assert!(html.contains(r#"location.replace("https://x.com/?a=1&b=2")"#));
}

fn err(text: &str) -> String {
    parse(text).unwrap_err()
}

#[test]
fn rejects_bad_input() {
    assert!(err("(bad/slug \"https://x.com\")").contains("slug 'bad/slug'"));
    assert!(err("(a \"ftp://x.com\")").contains("must start with"));
    assert!(err("(a \"https://x.com/\\\"><script>\")").contains("disallowed character '\"'"));
    assert!(err("(a \"https://x.com/'\")").contains("disallowed character '''"));
    assert!(err("(a \"https://x.com\")\n(a \"https://y.com\")").starts_with("2: duplicate slug"));
    assert!(err("(a https://x.com)").contains("expected (slug \"url\")"));
    assert!(err("(a \"https://x.com\" extra)").contains("expected"));
    assert!(err("(\"a\" \"https://x.com\")").contains("expected"));
    assert!(err("(a \"https://x.com)").contains("EOF while parsing a string"));
    assert!(err("(a \"https://x.com\"").contains("EOF while parsing a list"));
    assert!(err("(a \"https://x.com\"))").starts_with("1: "));
}
