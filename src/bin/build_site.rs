use chrono::Utc;
use serde::Serialize;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
use tera::{Context, Tera};

#[derive(Serialize)]
struct PageData {
    files: Vec<String>,
    latest: Option<String>,
    built_at_utc: String,
    repo: String,
    sha: String,
    commit_message: String,
    author_name: String,
    timestamp: String,
}

fn copy_xlsx_to_site(repo_root: &Path) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let site = repo_root.join("site");
    fs::create_dir_all(&site)?;

    for entry in fs::read_dir(repo_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
            let fname = path.file_name().unwrap().to_string_lossy().to_string();
            let target = site.join(&fname);
            fs::rename(&path, &target).or_else(|_| fs::copy(&path, &target).map(|_| ()))?;
        }
    }

    for entry in fs::read_dir(&site)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xlsx") {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                files.push(name.to_string());
            }
        }
    }

    files.sort();

    if let Some(latest_src) = files.last() {
        let latest_src_path = site.join(latest_src);
        let latest_dst_path = site.join("latest.xlsx");
        fs::copy(&latest_src_path, &latest_dst_path)?;
    }

    Ok(files)
}

fn main() -> anyhow::Result<()> {
    let repo_root = PathBuf::from(env::var("GITHUB_WORKSPACE").unwrap_or_else(|_| ".".into()));
    let files = copy_xlsx_to_site(&repo_root)?;

    let built_at_utc = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let repo = env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "unknown/unknown".into());
    let sha = env::var("GITHUB_SHA").unwrap_or_else(|_| "unknown".into());
    let commit_message = env::var("GITHUB_COMMIT_MESSAGE").unwrap_or_else(|_| "unknown".into());
    let author_name = env::var("GITHUB_AUTHOR_NAME").unwrap_or_else(|_| "unknown".into());
    let timestamp = env::var("GITHUB_TIMESTAMP").unwrap_or_else(|_| "unknown".into());

    let data = PageData {
        latest: files.last().cloned().or(None),
        files,
        built_at_utc,
        repo,
        sha,
        commit_message,
        author_name,
        timestamp,
    };

    let tera = Tera::new("templates/**/*")?;
    let mut ctx = Context::new();
    ctx.try_insert("files", &data.files)?;
    ctx.try_insert("latest", &data.latest)?;
    ctx.try_insert("built_at_utc", &data.built_at_utc)?;
    ctx.try_insert("repo", &data.repo)?;
    ctx.try_insert("sha", &data.sha)?;
    ctx.try_insert("commit_message", &data.commit_message)?;
    ctx.try_insert("author_name", &data.author_name)?;
    ctx.try_insert("timestamp", &data.timestamp)?;

    let html = tera.render("index.html", &ctx)?;
    let site_dir = repo_root.join("site");
    fs::create_dir_all(&site_dir)?;
    fs::write(site_dir.join("index.html"), html)?;

    Ok(())
}
