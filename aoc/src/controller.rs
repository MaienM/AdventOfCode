use std::{env, fs};

use puzzle_runner::{
    controller::{Controller, ControllerResult},
    derived::ChapterBuilder,
    http::HTTPRequest,
};

static BASE_URL: &str = "https://adventofcode.com";

pub struct AoCController {
    cookie: Result<String, String>,
}
impl AoCController {
    fn add_auth_header(&self, request: &mut HTTPRequest) -> ControllerResult<()> {
        request
            .headers
            .insert("cookie".to_owned(), self.cookie.clone()?);
        Ok(())
    }

    fn parse_name(name: &str) -> ControllerResult<(u16, u8)> {
        let (year, day) = name
            .split_once('-')
            .ok_or_else(|| format!("failed to parse year/day from chapter name {name}"))?;
        let year = 2000 + year.parse::<u16>()?;
        let day = day.parse::<u8>()?;
        Ok((year, day))
    }

    fn format_chapter_url(chapter: &str, stem: Option<&str>) -> ControllerResult<String> {
        let (year, day) = Self::parse_name(chapter)?;
        let mut url = format!("{BASE_URL}/{year}/day/{day}");
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
        #[cfg(target_arch = "wasm32")]
        let cookie =
            Err("actions requiring authentication are not available in the web version".to_owned());
        #[cfg(not(target_arch = "wasm32"))]
        let cookie = env::var("AOC_SESSION_COOKIE_FILE")
            .map_err(|e| e.to_string())
            .and_then(|p| fs::read_to_string(p).map_err(|e| e.to_string()))
            .map(|session| format!("session={}", session.trim()));

        Ok(Self { cookie })
    }

    fn process_chapter(&self, chapter: &mut ChapterBuilder) -> ControllerResult<()> {
        assert!(chapter.book == Some(None));
        let (year, _) = Self::parse_name(chapter.name.unwrap())?;
        chapter.book(Some(year.to_string()));

        Ok(())
    }

    fn chapter_url(&self, chapter: &str) -> ControllerResult<String> {
        Self::format_chapter_url(chapter, None)
    }

    fn get_input(&self, chapter: &str) -> ControllerResult<String> {
        let mut request =
            HTTPRequest::new("GET", &Self::format_chapter_url(chapter, Some("input"))?);
        self.add_auth_header(&mut request)?;
        let text = request.send()?;
        Ok(text.strip_suffix('\n').unwrap_or(&text).to_owned())
    }

    fn validate_result_impl(
        &self,
        chapter: &str,
        part: u8,
        result: &str,
    ) -> ControllerResult<(bool, String)> {
        let mut request =
            HTTPRequest::new("POST", &Self::format_chapter_url(chapter, Some("answer"))?);
        self.add_auth_header(&mut request)?;
        request.set_body_form([("level", part.to_string()), ("answer", result.to_owned())]);
        let html = request.send()?;

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
