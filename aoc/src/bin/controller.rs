use std::{env, fs};

use ehttp::{Request, fetch_async};
use pollster::FutureExt;
use puzzle_runner::controller::{Controller, ControllerResult};

#[puzzle_runner::register_controller]
pub struct AoCController {
    url: &'static str,
    cookie: Result<String, String>,
}
impl AoCController {
    fn format_chapter_url(&self, chapter: &str, stem: Option<&str>) -> ControllerResult<String> {
        let (year, day) = chapter
            .split_once('-')
            .ok_or_else(|| format!("failed to parse year/day from chapter name {chapter}"))?;
        let year = 2000 + year.parse::<i32>()?;
        let day = day.parse::<u32>()?;

        let mut url = format!("{}/{year}/day/{day}", self.url);
        if let Some(stem) = stem {
            url = format!("{url}/{stem}");
        }
        Ok(url)
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

        #[cfg(target_arch = "wasm32")]
        let cookie =
            Err("actions requiring authentication are not available in the web version".to_owned());
        #[cfg(not(target_arch = "wasm32"))]
        let cookie = env::var("AOC_SESSION_COOKIE_FILE")
            .map_err(|e| e.to_string())
            .and_then(|p| fs::read_to_string(p).map_err(|e| e.to_string()))
            .map(|session| format!("session={}", session.trim()));

        Ok(Self { url, cookie })
    }

    fn chapter_url(&self, chapter: &str) -> ControllerResult<String> {
        self.format_chapter_url(chapter, None)
    }

    fn get_input(&self, chapter: &str) -> ControllerResult<String> {
        let url = self.format_chapter_url(chapter, Some("input"))?;
        let mut request = Request::get(url);
        request.headers.insert("cookie", self.cookie.clone()?);
        let response = fetch_async(request).block_on()?;
        let text = response.text().ok_or("empty response")?;
        Ok(text.strip_suffix('\n').unwrap_or(text).to_owned())
    }

    fn validate_result_impl(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
    ) -> ControllerResult<(bool, String)> {
        let url = self.format_chapter_url(chapter, Some("answer"))?;
        let mut request = Request::post(url, format!("level={part}&answer={result}").into_bytes());
        request.headers.insert("cookie", self.cookie.clone()?);
        let response = fetch_async(request).block_on()?;
        let html = response.text().ok_or("empty response")?;

        let mut path = env::temp_dir();
        path.push("aoc-reponse.html");
        let tmp_write = fs::write(&path, html);

        let text = AoCController::extract_text(html).map_err(|e1| match tmp_write {
            Ok(()) => format!("Failed to parse response ({}), wrote full response to {}.", String::from(e1), path.display()),
            Err(e2) => format!("Failed to parse response ({}), and failed to write response to temporary file {} ({e2}), so including full response here: {html}", String::from(e1), path.display())
        })?;

        Ok((text.contains("That's the right answer!"), text))
    }
}
