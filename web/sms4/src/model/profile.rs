use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: i64,
    pub birthday: Option<String>,
    pub gender: Option<String>,
    pub city: Option<String>,
    pub occupation: Option<String>,
    pub education: Option<String>,
    pub height: Option<i64>,
    pub looking_for: Option<String>,
    pub about_me: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileWithUser {
    pub user_id: i64,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
    pub city: Option<String>,
    pub occupation: Option<String>,
    pub education: Option<String>,
    pub height: Option<i64>,
    pub looking_for: Option<String>,
    pub about_me: Option<String>,
    pub tags: Vec<String>,
    pub age: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileSearch {
    pub gender: Option<String>,
    pub age_min: Option<i64>,
    pub age_max: Option<i64>,
    pub city: Option<String>,
    pub occupation: Option<String>,
    pub education: Option<String>,
    pub height_min: Option<i64>,
    pub height_max: Option<i64>,
    pub looking_for: Option<String>,
    pub tags: Option<String>,
    pub q: Option<String>,
}

pub fn upsert_profile(conn: &Connection, user_id: i64, p: &Profile) -> Result<()> {
    conn.execute(
        "INSERT INTO profiles (user_id, birthday, gender, city, occupation, education, height, looking_for, about_me, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))
         ON CONFLICT(user_id) DO UPDATE SET
            birthday=excluded.birthday, gender=excluded.gender, city=excluded.city,
            occupation=excluded.occupation, education=excluded.education,
            height=excluded.height, looking_for=excluded.looking_for,
            about_me=excluded.about_me, updated_at=excluded.updated_at",
        params![
            user_id, p.birthday, p.gender, p.city, p.occupation,
            p.education, p.height, p.looking_for, p.about_me
        ],
    )?;
    Ok(())
}

pub fn get_profile(conn: &Connection, user_id: i64) -> Result<Option<Profile>> {
    let mut stmt = conn.prepare(
        "SELECT user_id, birthday, gender, city, occupation, education, height, looking_for, about_me, updated_at
         FROM profiles WHERE user_id = ?1"
    )?;
    let mut rows = stmt.query_map(params![user_id], |row| {
        Ok(Profile {
            user_id: row.get(0)?,
            birthday: row.get(1)?,
            gender: row.get(2)?,
            city: row.get(3)?,
            occupation: row.get(4)?,
            education: row.get(5)?,
            height: row.get(6)?,
            looking_for: row.get(7)?,
            about_me: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

fn get_age(birthday: &str) -> Option<i64> {
    let parts: Vec<&str> = birthday.split('-').collect();
    if parts.len() != 3 { return None; }
    let year: i64 = parts[0].parse().ok()?;
    let this_year = 2026i64;
    Some(this_year - year)
}

pub fn search_profiles(conn: &Connection, s: &ProfileSearch) -> Result<Vec<ProfileWithUser>> {
    let mut sql = String::from(
        "SELECT u.id, u.username, u.display_name, u.bio,
                p.birthday, p.gender, p.city, p.occupation, p.education,
                p.height, p.looking_for, p.about_me
         FROM users u
         LEFT JOIN profiles p ON p.user_id = u.id
         WHERE 1=1"
    );
    let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref gender) = s.gender {
        params_vec.push(Box::new(gender.clone()));
        sql.push_str(&format!(" AND p.gender = ?{}", params_vec.len()));
    }
    if let Some(ref city) = s.city {
        params_vec.push(Box::new(format!("%{}%", city)));
        sql.push_str(&format!(" AND p.city LIKE ?{}", params_vec.len()));
    }
    if let Some(ref occupation) = s.occupation {
        params_vec.push(Box::new(format!("%{}%", occupation)));
        sql.push_str(&format!(" AND p.occupation LIKE ?{}", params_vec.len()));
    }
    if let Some(ref education) = s.education {
        params_vec.push(Box::new(education.clone()));
        sql.push_str(&format!(" AND p.education = ?{}", params_vec.len()));
    }
    if let Some(ref looking_for) = s.looking_for {
        params_vec.push(Box::new(looking_for.clone()));
        sql.push_str(&format!(" AND p.looking_for = ?{}", params_vec.len()));
    }
    if s.height_min.is_some() || s.height_max.is_some() {
        if let Some(h) = s.height_min {
            params_vec.push(Box::new(h));
            sql.push_str(&format!(" AND p.height >= ?{}", params_vec.len()));
        }
        if let Some(h) = s.height_max {
            params_vec.push(Box::new(h));
            sql.push_str(&format!(" AND p.height <= ?{}", params_vec.len()));
        }
    }
    if s.age_min.is_some() || s.age_max.is_some() {
        let this_year = 2026i64;
        if let Some(age) = s.age_max {
            let y = this_year - age;
            params_vec.push(Box::new(y));
            sql.push_str(&format!(" AND CAST(SUBSTR(p.birthday, 1, 4) AS INTEGER) >= ?{}", params_vec.len()));
        }
        if let Some(age) = s.age_min {
            let y = this_year - age;
            params_vec.push(Box::new(y));
            sql.push_str(&format!(" AND CAST(SUBSTR(p.birthday, 1, 4) AS INTEGER) <= ?{}", params_vec.len()));
        }
    }
    if let Some(ref q) = s.q {
        params_vec.push(Box::new(format!("%{}%", q)));
        sql.push_str(&format!(" AND (p.about_me LIKE ?{} OR p.occupation LIKE ?{} OR u.display_name LIKE ?{})", params_vec.len(), params_vec.len(), params_vec.len()));
    }

    let tags: Vec<String> = s.tags.as_ref().map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()).unwrap_or_default();

    if !tags.is_empty() {
        for tag in &tags {
            params_vec.push(Box::new(tag.clone()));
        }
        let placeholders: Vec<String> = (0..tags.len()).map(|i| format!("?{}", params_vec.len() - tags.len() + i + 1)).collect();
        sql.push_str(&format!(
            " AND u.id IN (SELECT user_id FROM interests WHERE tag IN ({}))",
            placeholders.join(",")
        ));
    }

    sql.push_str(" ORDER BY u.id");

    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    let rows = stmt.query_map(param_refs.as_slice(), |row| {
        Ok(ProfileWithUser {
            user_id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            bio: row.get(3)?,
            birthday: row.get(4)?,
            gender: row.get(5)?,
            city: row.get(6)?,
            occupation: row.get(7)?,
            education: row.get(8)?,
            height: row.get(9)?,
            looking_for: row.get(10)?,
            about_me: row.get(11)?,
            tags: Vec::new(),
            age: row.get::<_, Option<String>>(4)?.as_deref().and_then(get_age),
        })
    })?;

    let mut results: Vec<ProfileWithUser> = Vec::new();
    for row in rows {
        results.push(row?);
    }

    if !tags.is_empty() {
        for r in &mut results {
            let mut stmt2 = conn.prepare("SELECT tag FROM interests WHERE user_id = ?1")?;
            let tag_rows = stmt2.query_map(params![r.user_id], |row| row.get::<_, String>(0))?;
            for t in tag_rows {
                if let Ok(tag) = t {
                    r.tags.push(tag);
                }
            }
        }
    }

    Ok(results)
}
