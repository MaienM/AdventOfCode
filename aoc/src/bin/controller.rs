use std::{env, fs};

use puzzle_runner::controller::{Controller, ControllerResult};

#[puzzle_runner::register_controller]
struct AoCController {
    url: &'static str,
    cookie: String,
}
impl AoCController {
    fn chapter_url(&self, chapter: &str, stem: &str) -> ControllerResult<String> {
        let (year, day) = chapter
            .split_once('-')
            .ok_or_else(|| format!("failed to parse year/day from chapter name {chapter}"))?;
        let year = 2000 + year.parse::<i32>()?;
        let day = day.parse::<u32>()?;

        Ok(format!("{}/{year}/day/{day}/{stem}", self.url))
    }
}
impl Controller for AoCController {
    fn new() -> ControllerResult<Self> {
        let url = "https://adventofcode.com";

        let session = fs::read_to_string(env::var("AOC_SESSION_COOKIE_FILE")?)?;
        let cookie = format!("session={}", session.trim());

        Ok(Self { url, cookie })
    }

    fn get_input(&self, chapter: &str) -> ControllerResult<String> {
        let url = self.chapter_url(chapter, "input")?;
        let text = ureq::get(url)
            .header("cookie", self.cookie.clone())
            .call()?
            .body_mut()
            .read_to_string()?;
        Ok(text.strip_suffix('\n').unwrap_or(&text).to_owned())
    }
}
