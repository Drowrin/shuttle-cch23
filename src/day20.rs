use std::{fs::File, io::Write, path::Path, process::Command, str::from_utf8};

use axum::{body::Bytes, routing, Router};
use sqlx::PgPool;
use tar::Archive;
use tempfile::tempdir;
// use tracing::{error, info};

async fn archive_files(body: Bytes) -> String {
    Archive::new(body.as_ref())
        .entries()
        .unwrap()
        .count()
        .to_string()
}

async fn archive_files_size(body: Bytes) -> String {
    Archive::new(body.as_ref())
        .entries()
        .unwrap()
        .map(|e| e.unwrap().size())
        .sum::<u64>()
        .to_string()
}

fn command(path: &Path, program: &str, args: &[&str]) -> (String, String) {
    // info!("> {program} {}", args.join(" "));

    let out = Command::new(program)
        .args(args)
        .current_dir(path)
        .output()
        .unwrap();

    let stdout = from_utf8(&out.stdout).unwrap().trim().to_string();
    let stderr = from_utf8(&out.stderr).unwrap().trim().to_string();

    // if stdout.len() > 0 {
    //     info!("{stdout}");
    // }
    // if stderr.len() > 0 {
    //     error!("{stderr}");
    // }

    (stdout, stderr)
}

async fn cookie(body: Bytes) -> String {
    let repo = tempdir().unwrap();

    let tarpath = repo.path().join("input.tar");
    let mut tarfile = File::create(tarpath.clone()).unwrap();
    tarfile.write(&body).unwrap();

    command(
        repo.path(),
        "tar",
        &[
            "-xf",
            tarpath.to_str().unwrap(),
            "-C",
            repo.path().to_str().unwrap(),
        ],
    );

    command(repo.path(), "git", &["checkout", "christmas"]);

    let (log, _) = command(
        repo.path(),
        "git",
        &["log", "--all", "--full-history", "-p", "--", "**santa.txt"],
    );

    let mut commits = log.split("commit ");
    while let Some(commit) = commits.next() {
        let mut lines = commit.lines();
        if let Some(cookie_line) =
            lines.find(|line| line.contains("COOKIE") && line.starts_with(['-', '+']))
        {
            let cookie_commit = match cookie_line.chars().next().unwrap() {
                '-' => commits.next().unwrap(),
                '+' => commit,
                _ => unreachable!(),
            };

            let mut cookie_commit_lines = cookie_commit.lines();
            let hash = cookie_commit_lines.next().unwrap();
            let author_line = cookie_commit_lines.next().unwrap();
            let author = &author_line.split(" <").next().unwrap()[8..];

            // println!("FOUND: {}", format!("{author} {hash}"));
            return format!("{author} {hash}");
        }
    }

    "".into()
}

pub fn get_routes() -> Router<PgPool> {
    Router::new()
        .route("/20/archive_files", routing::post(archive_files))
        .route("/20/archive_files_size", routing::post(archive_files_size))
        .route("/20/cookie", routing::post(cookie))
}
