use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{env, sync::Mutex};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub slug: String,
    pub visit_count: i32,
    pub last_edited_at: String,
    pub created_at: String,
}

lazy_static! {
    pub static ref DB: Mutex<Connection> = Mutex::new(Connection::open(get_db_path()).unwrap());
}
fn get_db_path() -> String {
    env::var("DB_PATH").unwrap_or_else(|_| "urls.db".to_string())
}
pub fn init_db() -> Result<()> {
    let conn = DB.lock().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            original_url TEXT NOT NULL,
            slug TEXT NOT NULL UNIQUE,
            visit_count INTEGER DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_edited_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_url(conn: &Connection, original_url: &str, slug: &str) -> Result<i32> {
    conn.execute(
        "INSERT INTO urls (original_url, slug, visit_count) VALUES (?, ?, 0)",
        [original_url, slug],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn get_url_by_slug(conn: &Connection, slug: &str) -> Result<Url> {
    let mut stmt = conn.prepare("SELECT * FROM urls WHERE slug = ?")?;
    let url_iter = stmt.query_map([slug], |row| {
        Ok(Url {
            id: row.get(0)?,
            original_url: row.get(1)?,
            slug: row.get(2)?,
            visit_count: row.get(3)?,
            last_edited_at: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    let x = url_iter.flatten().next().ok_or(Error::InvalidQuery);
    x
}

pub fn increment_visit_count(conn: &Connection, slug: &str) -> Result<()> {
    conn.execute(
        "UPDATE urls SET visit_count = visit_count + 1 WHERE slug = ?",
        [slug],
    )?;
    Ok(())
}

pub fn get_all_urls(conn: &Connection) -> Result<Vec<Url>> {
    let mut stmt = conn.prepare("SELECT * FROM urls")?;
    let url_iter = stmt.query_map([], |row| {
        Ok(Url {
            id: row.get(0)?,
            original_url: row.get(1)?,
            slug: row.get(2)?,
            visit_count: row.get(3)?,
            last_edited_at: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    url_iter.collect()
}

pub fn update_url(
    conn: &Connection,
    original_slug: &str,
    new_slug: &str,
    new_url: &str,
) -> rusqlite::Result<usize> {
    let query =
        "UPDATE urls SET slug = ?, original_url = ?, last_edited_at = CURRENT_TIMESTAMP WHERE slug = ?";

    conn.execute(&query, [new_slug, new_url, original_slug])
}
pub fn delete_url(conn: &Connection, slug: &str) -> rusqlite::Result<usize> {
    let query = "DELETE FROM urls WHERE slug = ?";

    conn.execute(&query, [slug])
}

pub fn _slug_exists(conn: &Connection, slug: &str) -> rusqlite::Result<bool> {
    let count: u32 = conn
        .query_row("SELECT COUNT(*) FROM urls WHERE slug = ?", [slug], |row| {
            row.get(0)
        })
        .unwrap_or(0);
    Ok(count > 0)
}
