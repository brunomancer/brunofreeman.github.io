//! Trunk post_build hook: turns each `(slug "url")` form in redirects.sexp
//! into a static `<slug>/index.html` that bounces to its URL. GitHub Pages
//! can't do server-side redirects, so this is as close as a static host gets.

use std::{collections::HashSet, env, fs, path::Path, process::ExitCode};

use leptos::prelude::*;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/redirects.sexp");
const SOURCE_NAME: &str = "redirects/redirects.sexp";

#[derive(Debug, PartialEq)]
struct Redirect {
    line: usize,
    slug: String,
    url: String,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let out = env::var_os("TRUNK_STAGING_DIR")
        .ok_or("TRUNK_STAGING_DIR is not set; this runs as a Trunk hook")?;
    let text = fs::read_to_string(SOURCE).map_err(|e| format!("{SOURCE_NAME}: {e}"))?;
    let redirects = parse(&text).map_err(|e| format!("{SOURCE_NAME}:{e}"))?;
    for r in &redirects {
        write_page(Path::new(&out), r).map_err(|e| format!("{SOURCE_NAME}:{}: {e}", r.line))?;
    }
    Ok(())
}

// ---- Parsing ----

/// Parses a sequence of `(slug "url")` forms and validates each one.
fn parse(text: &str) -> Result<Vec<Redirect>, String> {
    let mut parser = lexpr::Parser::from_str(text);
    let mut redirects = Vec::new();
    let mut seen = HashSet::new();

    for datum in parser.datum_iter() {
        // Parsing from a string always yields a location; 0 is just a fallback.
        let datum = datum.map_err(|e| format!("{}: {e}", e.location().map_or(0, |l| l.line())))?;
        let line = datum.span().start().line();
        let (slug, url) = datum
            .list_iter()
            .and_then(|mut items| {
                let slug = items.next()?.value().as_symbol()?;
                let url = items.next()?.value().as_str()?;
                items.is_empty().then_some((slug, url))
            })
            .ok_or_else(|| format!("{line}: expected (slug \"url\"), got {}", datum.value()))?;
        validate(slug, url).map_err(|e| format!("{line}: {e}"))?;
        if !seen.insert(slug.to_owned()) {
            return Err(format!("{line}: duplicate slug '{slug}'"));
        }
        redirects.push(Redirect {
            line,
            slug: slug.to_owned(),
            url: url.to_owned(),
        });
    }
    Ok(redirects)
}

fn validate(slug: &str, url: &str) -> Result<(), String> {
    if !slug
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(format!(
            "slug '{slug}' may only use letters, digits, '-' and '_'"
        ));
    }
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err(format!("url '{url}' must start with http:// or https://"));
    }
    // RFC 3986 characters minus `'`, so the URL can't break out of the
    // HTML attributes or JS string below. Anything else should be %-encoded.
    if let Some(c) = url
        .chars()
        .find(|&c| !(c.is_ascii_alphanumeric() || "-._~:/?#[]@!$&()*+,;=%".contains(c)))
    {
        return Err(format!("url '{url}' contains disallowed character '{c}'"));
    }
    Ok(())
}

// ---- Output ----

fn write_page(out: &Path, r: &Redirect) -> Result<(), String> {
    let dir = out.join(&r.slug);
    if dir.exists() {
        return Err(format!("slug '{}' collides with a built file", r.slug));
    }
    fs::create_dir(&dir).map_err(|e| e.to_string())?;
    fs::write(dir.join("index.html"), page(&r.url)).map_err(|e| e.to_string())
}

fn page(url: &str) -> String {
    let url = url.to_owned();
    // `validate` rules out quotes, backslashes and `<`, so the URL can sit
    // in a JS string literal as-is.
    let script = format!("location.replace(\"{url}\");");
    let html = view! {
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <title>"Redirecting…"</title>
                <meta name="robots" content="noindex" />
                <link rel="canonical" href=url.clone() />
                <meta http-equiv="refresh" content=format!("0; url={url}") />
                <script inner_html=script></script>
            </head>
            <body>
                <a href=url.clone()>{format!("Continue to {url}")}</a>
            </body>
        </html>
    };
    format!("<!DOCTYPE html>\n{}\n", html.to_html())
}

#[cfg(test)]
mod tests;
