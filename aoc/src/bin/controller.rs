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

    fn extract_text(html: &str) -> ControllerResult<String> {
        let (_, main) = html
            .split_once("<main>")
            .ok_or("failed to find start of main section")?;
        let (main, _) = main
            .split_once("</main>")
            .ok_or("failed to find end of main section")?;
        let text: String = main
            .split('>')
            .map(|part| part.split_once('<').map_or(part, |p| p.0))
            .collect();
        let text = text.replace("  ", " ");
        Ok(text)
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

    fn validate_result_impl(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
    ) -> ControllerResult<(bool, String)> {
        let url = self.chapter_url(chapter, "answer")?;
        let html = ureq::post(url)
            .header("cookie", self.cookie.clone())
            .send_form([("level", part.to_string()), ("answer", result.to_owned())])?
            .body_mut()
            .read_to_string()?;

        let mut path = env::temp_dir();
        path.push("aoc-reponse.html");
        let tmp_write = fs::write(&path, &html);

        let text = AoCController::extract_text(&html).map_err(|e1| match tmp_write {
            Ok(()) => format!("Failed to parse response ({}), wrote full response to {}.", String::from(e1), path.display()),
            Err(e2) => format!("Failed to parse response ({}), and failed to write response to temporary file {} ({e2}), so including full response here: {html}", String::from(e1), path.display())
        })?;

        Ok((text.contains("That's the right answer!"), text))
    }
}
