//! My solutions to the [Advent of Code](https://adventofcode.com) challenges.
#![allow(unused)]

use std::{env, fs, sync::Arc};

use common_macros::hash_map;
use puzzle_runner::{
    controller::{Controller, ControllerError, ControllerResult},
    derived::Chapter,
};
use regex::Regex;
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

    fn validate_result(
        &self,
        chapter: &Chapter,
        part: &puzzle_runner::derived::Part,
        result: &str,
    ) -> ControllerResult<Result<(), String>> {
        let regex_main = Regex::new(r"(?s)<main>.+?</main>")?;
        let regex_tag = Regex::new(r"<[^>]+?>")?;

        // let url = self.chapter_url(chapter, "answer")?;
        // let params = hash_map![
        //     "level" => part.num.to_string(),
        //     "answer" => result.to_owned(),
        // ];
        // let html = self
        //     .client
        //     .post(url)
        //     .form(&params)
        //     .send()?
        //     .error_for_status()?
        //     .text()?;

        // let mut path = env::temp_dir();
        // path.push("aoc-reponse.html");
        // fs::write(&path, &html);

        let html = fs::read_to_string("/home/maienm/Downloads/incorrect.html")?;
        let main = regex_main
            .find(&html)
            .ok_or_else(|| {
                let mut path = env::temp_dir();
                path.push("aoc-reponse.html");
                match fs::write(&path, &html) {
                    Ok(()) => format!(
                        "unable to find main section in response, see {}",
                        path.display()
                    ),
                    // this'll be messy, but it beats not having the response available at all.
                    Err(_) => format!("unable to find main section in response:\n\n{html}"),
                }
            })?
            .as_str();
        let text = regex_tag.replace_all(main, "");

        println!("{text}");
        if text.contains("That's the right answer!") {
            Ok(Ok(()))
        } else {
            Ok(Err("incorrect answer".to_owned()))
        }
    }
}

puzzle_runner::register_series!(
    title = "Advent of Code",
    controller = AoCController::new().unwrap(),
);
