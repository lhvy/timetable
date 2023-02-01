use crate::scraper::scrape_timetable_page;
use generator::gen_timetable_xml;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use std::sync::Arc;

mod generator;
mod scraper;
mod types;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15";
// const DELAY: Duration = Duration::from_millis(85);

fn main() -> anyhow::Result<()> {
    let host = dotenv::var("HOST")?;
    let timetable_uuid = dotenv::var("TIMETABLE_UUID")?;

    let jar = Jar::default();

    let cookie_str = format!(
        "TIMETABLE={}; Domain={}",
        dotenv::var("COOKIE")?,
        host.strip_prefix("https://").unwrap_or(&host)
    );
    jar.add_cookie_str(&cookie_str, &host.parse()?);

    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .user_agent(USER_AGENT)
        .build()?;

    let students = scraper::scrape_student_identities(&client, &host, &timetable_uuid)?;

    let bar = ProgressBar::new(students.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{wide_bar:.red}] [{pos:>3.blue}/{len:>3.blue}] Voting Ends In: {eta:.bold} {msg:.white.bold}")?
            .progress_chars(" ඞ "),
    );

    for student in students {
        let text = client
            .get(format!("{}/{}/{}", host, timetable_uuid, student.id))
            .send()?
            .text()?;

        let timetable = scrape_timetable_page(&text);

        let xml = gen_timetable_xml(&timetable);

        std::fs::write(
            format!(
                "timetables/{}+{}+{}.timetable",
                student.id,
                student.first.to_lowercase(),
                student.last.to_lowercase()
            ),
            xml,
        )?;

        bar.inc(1);
    }
    bar.finish_with_message("Red was the impostor...");

    Ok(())
}
