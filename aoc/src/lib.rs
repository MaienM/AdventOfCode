//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.

use std::{env, fs, sync::Arc};

use puzzle_runner::{
    controller::{Controller, ControllerResult},
    derived::Chapter,
};
use reqwest::{
    Url,
    blocking::{Client, ClientBuilder},
    cookie::Jar,
};

struct AoCController {
    url: Url,
    client: Client,
}
impl AoCController {
    fn new() -> ControllerResult<Self> {
        let url = "https://adventofcode.com".parse::<Url>()?;
        let session = fs::read_to_string(env::var("AOC_SESSION_COOKIE_FILE")?)?;

        let jar = Jar::default();
        jar.add_cookie_str(&format!("session={session}"), &url);

        let client = ClientBuilder::new()
            .use_rustls_tls()
            .cookie_provider(Arc::new(jar))
            .build()?;

        Ok(Self { url, client })
    }

    fn chapter_url(&self, chapter: &Chapter, stem: &str) -> ControllerResult<Url> {
        let (year, day) = chapter.name.split_once('-').ok_or_else(|| {
            format!(
                "failed to parse year/day from chapter name {}",
                chapter.name
            )
        })?;
        let year = 2000 + year.parse::<i32>()?;
        let day = day.parse::<u32>()?;

        Ok(self.url.join(&format!("/{year}/day/{day}/{stem}"))?)
    }
}
impl Controller for AoCController {
    fn get_input(&self, chapter: &Chapter) -> ControllerResult<String> {
        println!("Downloading input...");
        let url = self.chapter_url(chapter, "input")?;
        let text = self.client.get(url).send()?.error_for_status()?.text()?;
        Ok(text.strip_suffix('\n').unwrap_or(&text).to_owned())
    }
}

puzzle_runner::register_series!(
    title = "Advent of Code",
    controller = AoCController::new().unwrap(),
);
