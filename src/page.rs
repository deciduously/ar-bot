// page.rs handles the askama templates
use askama::Template;
use brain::Context;

#[derive(Template)]
#[template(path = "digest.html")]
struct DigestTemplate<'a> {
    entries: Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate<'a> {
    date: &'a str,
}

// write_digest writes the digest to hx/
pub fn write_digest(c: &Context) {
    let hx_path = c.hx_path();
}
