// page.rs handles the askama templates
use askama::Template;
use chrono::prelude::*;
use crate::{batch::{Batch, BatchEntry}, brain::Context, errors::*};
use std::{fs::File, io::prelude::*};

#[derive(Template)]
#[template(path = "digest.html")]
struct DigestTemplate {
    entries: Vec<BatchEntry>,
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate<'a> {
    date: &'a str,
}

#[derive(Template)]
#[template(path = "skel.html")]
struct SkelTemplate {}

// write_digest writes the digest to hx/
pub fn write_digest(ctx: &Context) -> Result<()> {
    let batch = Batch::from_brain(&ctx.brain)?;
    let mut digest_path = ctx.hx_path();
    digest_path.push(format!("digest-{}.html", Local::now().timestamp()));
    let mut digest_file = File::create(digest_path).chain_err(|| "Could not create digest file")?;
    let mut entries = Vec::new();

    for entry in batch.entries.values() {
        entries.push(entry.clone());
    }
    let digest = DigestTemplate { entries };
    digest_file
        .write_all(
            digest
                .render()
                .expect("Could not render digest template")
                .as_bytes(),
        )
        .chain_err(|| "Could not write digest")?;
    Ok(())
}
