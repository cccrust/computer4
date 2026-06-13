use crate::cli::fmt;
use crate::model::profile::{self, Profile, ProfileSearch};
use crate::model::interest;
use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;

#[derive(Parser)]
pub struct ProfileCommand {
    #[command(subcommand)]
    pub subcommand: ProfileSubcommands,
}

#[derive(Subcommand)]
pub enum ProfileSubcommands {
    Set {
        user_id: i64,
        #[arg(long)]
        birthday: Option<String>,
        #[arg(long)]
        gender: Option<String>,
        #[arg(long)]
        city: Option<String>,
        #[arg(long)]
        occupation: Option<String>,
        #[arg(long)]
        education: Option<String>,
        #[arg(long)]
        height: Option<i64>,
        #[arg(long)]
        looking_for: Option<String>,
        #[arg(long)]
        about_me: Option<String>,
    },
    Show {
        user_id: i64,
    },
    Search {
        #[arg(long)]
        gender: Option<String>,
        #[arg(long)]
        age_min: Option<i64>,
        #[arg(long)]
        age_max: Option<i64>,
        #[arg(long)]
        city: Option<String>,
        #[arg(long)]
        occupation: Option<String>,
        #[arg(long)]
        education: Option<String>,
        #[arg(long)]
        height_min: Option<i64>,
        #[arg(long)]
        height_max: Option<i64>,
        #[arg(long)]
        looking_for: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(short, long)]
        q: Option<String>,
    },
}

pub fn run(conn: &Connection, cmd: &ProfileSubcommands) -> Result<()> {
    match cmd {
        ProfileSubcommands::Set { user_id, birthday, gender, city, occupation, education, height, looking_for, about_me } => {
            let p = Profile {
                user_id: *user_id,
                birthday: birthday.clone(),
                gender: gender.clone(),
                city: city.clone(),
                occupation: occupation.clone(),
                education: education.clone(),
                height: *height,
                looking_for: looking_for.clone(),
                about_me: about_me.clone(),
                updated_at: String::new(),
            };
            match profile::upsert_profile(conn, *user_id, &p) {
                Ok(_) => println!("{}", fmt::success_msg(&format!("使用者 #{} 的交友資料已更新", user_id))),
                Err(e) => println!("{}", fmt::error_msg(&e.to_string())),
            }
        }
        ProfileSubcommands::Show { user_id } => {
            let p = profile::get_profile(conn, *user_id)?;
            let tags = interest::list_interests(conn, *user_id)?;
            match p {
                Some(prof) => {
                    println!("{}", fmt::header("交友資料"));
                    println!("{}: {}", fmt::label("使用者"), user_id);
                    if let Some(v) = &prof.birthday { println!("{}: {}", fmt::label("生日"), v); }
                    if let Some(v) = &prof.gender { println!("{}: {}", fmt::label("性別"), v); }
                    if let Some(v) = &prof.city { println!("{}: {}", fmt::label("城市"), v); }
                    if let Some(v) = &prof.occupation { println!("{}: {}", fmt::label("職業"), v); }
                    if let Some(v) = &prof.education { println!("{}: {}", fmt::label("學歷"), v); }
                    if let Some(v) = &prof.height { println!("{}: {}", fmt::label("身高"), v); }
                    if let Some(v) = &prof.looking_for { println!("{}: {}", fmt::label("交友目的"), v); }
                    if let Some(v) = &prof.about_me { println!("{}: {}", fmt::label("關於我"), v); }
                    if !tags.is_empty() {
                        let tag_str: Vec<String> = tags.iter().map(|t| t.tag.clone()).collect();
                        println!("{}: {}", fmt::label("興趣"), tag_str.join(", "));
                    }
                }
                None => println!("{}", fmt::info_msg("該使用者尚未填寫交友資料。")),
            }
        }
        ProfileSubcommands::Search { gender, age_min, age_max, city, occupation, education, height_min, height_max, looking_for, tags, q } => {
            let search = ProfileSearch {
                gender: gender.clone(),
                age_min: *age_min,
                age_max: *age_max,
                city: city.clone(),
                occupation: occupation.clone(),
                education: education.clone(),
                height_min: *height_min,
                height_max: *height_max,
                looking_for: looking_for.clone(),
                tags: tags.clone(),
                q: q.clone(),
            };
            let results = profile::search_profiles(conn, &search)?;
            if results.is_empty() {
                println!("{}", fmt::info_msg("沒有符合條件的使用者。"));
                return Ok(());
            }
            println!("{}", fmt::header(&format!("配對結果 ({} 人)", results.len())));
            for r in &results {
                let age_str = r.age.map(|a| format!("{} 歲", a)).unwrap_or_default();
                let city_str = r.city.as_deref().unwrap_or("");
                let occ_str = r.occupation.as_deref().unwrap_or("");
                let tags_str = if !r.tags.is_empty() { format!("興趣: {}", r.tags.join(", ")) } else { String::new() };
                println!("#{} {} (@{}) · {} · {} · {}", r.user_id, r.display_name, r.username, age_str, city_str, occ_str);
                if !tags_str.is_empty() { println!("   {}", tags_str); }
                if let Some(about) = &r.about_me { println!("   關於我: {}", about); }
                println!();
            }
        }
    }
    Ok(())
}
