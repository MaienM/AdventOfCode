//! The multi-day CLI entrypoints.

use std::{collections::HashSet, sync::OnceLock, time::Duration};

use ansi_term::Colour::{Cyan, Purple};
use clap::{
    Parser,
    builder::{PossibleValue, PossibleValuesParser, TypedValueParser},
};
use rayon::ThreadPoolBuilder;

use super::source::source_path_fill_tokens;
use crate::{
    derived::{Chapter, Part, Series},
    runner::{DurationThresholds, InstantTimer, PartResult},
    source::{ChapterSources, ChapterSourcesValueParser, Source},
};

pub static SERIES: OnceLock<Series> = OnceLock::new();

/// Create parser for --only/--skip.
fn create_target_value_parser() -> impl TypedValueParser {
    fn create_value(name: &str, suffix: &str) -> PossibleValue {
        PossibleValue::new(format!("{name}{suffix}"))
    }

    let mut options = Vec::new();
    for chapter in &SERIES.get().unwrap().chapters {
        options.push(create_value(chapter.name, ""));
        for part in &chapter.parts {
            options.push(create_value(chapter.name, &format!("-{}", part.num)));
        }
    }
    let parser = PossibleValuesParser::new(options);

    parser.map(|item| {
        let parts: Vec<u8> = item
            .split('-')
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        match parts.len() {
            3 => vec![(parts[0], parts[1], parts[2])],
            2 => vec![(parts[0], parts[1], 1), (parts[0], parts[1], 2)],
            1 => (1..=25)
                .flat_map(|d| vec![(parts[0], d, 1), (parts[0], d, 2)])
                .collect(),
            _ => panic!("Invalid filter item {item:?}."),
        }
    })
}

#[derive(Parser, Debug)]
pub(super) struct TargetArgs {
    /// Only run the listed items.
    ///
    /// This can be either {chapter} for a single chapter, or {chapter}-{part} a single part of a single chapter.
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "22-01,22-08-1",
        value_parser = create_target_value_parser(),
        group = "targets",
    )]
    only: Option<Vec<Vec<(String, u8)>>>,

    /// Skip the listed items.
    ///
    /// This can be either {chapter} for a single chapter, or {chapter}-{part} a single part of a single chapter.
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "22-01,22-08-1",
        value_parser = create_target_value_parser(),
        group = "targets",
    )]
    skip: Option<Vec<Vec<(String, u8)>>>,

    /// Pattern for paths to the folders containing the inputs (`input.txt`) and expected outputs (`partN.txt`).
    ///
    /// The following tokens will be replaced:
    /// - `{series}`: the name of the crate for the series (e.g., `aoc`).
    /// - `{chapter}`: the name of the chapter (e.g., `21-01`).
    #[arg(
        long,
        default_value = "inputs/{series}/{chapter}",
        value_parser = ChapterSourcesValueParser,
        verbatim_doc_comment,
        conflicts_with = "use_examples",
    )]
    folder_pattern: ChapterSources,

    /// Run using examples instead of real inputs/results.
    #[arg(long)]
    use_examples: bool,
}
impl TargetArgs {
    pub(super) fn filtered_chapters(&self) -> Vec<Chapter> {
        let mut chapters = SERIES.get().unwrap().chapters.clone();
        if let Some(only) = &self.only {
            let only: HashSet<_> = only.iter().flatten().collect();
            for chapter in &mut chapters {
                chapter
                    .parts
                    .retain_mut(|part| only.contains(&(chapter.name.to_owned(), part.num)));
            }
        } else if let Some(skip) = &self.skip {
            let skip: HashSet<_> = skip.iter().flatten().collect();
            for chapter in &mut chapters {
                chapter
                    .parts
                    .retain_mut(|part| !skip.contains(&(chapter.name.to_owned(), part.num)));
            }
        }
        chapters.retain_mut(|chapter| !chapter.parts.is_empty());
        chapters
    }

    pub(super) fn get_targets(&self, chapters: &[Chapter]) -> Vec<Target> {
        let mut targets = Vec::new();
        if self.use_examples {
            for chapter in chapters {
                for example in &chapter.examples {
                    for part in &chapter.parts {
                        let Some(solution) = example.parts.get(&part.num) else {
                            continue;
                        };
                        targets.push(Target {
                            chapter: chapter.name.to_owned(),
                            part: part.clone(),
                            source_name: Some(example.name.to_owned()),
                            input: Source::Inline {
                                source: example.name.to_owned(),
                                contents: example.input.to_owned(),
                            },
                            solution: Source::Inline {
                                source: example.name.to_owned(),
                                contents: (*solution).to_owned(),
                            },
                        });
                    }
                }
            }
        } else {
            for chapter in chapters {
                let folder = source_path_fill_tokens!(
                    self.folder_pattern,
                    series = SERIES.get().unwrap().name,
                    chapter = chapter.name,
                );
                for part in &chapter.parts {
                    targets.push(Target {
                        chapter: chapter.name.to_owned(),
                        part: part.clone(),
                        source_name: None,
                        input: folder.input().unwrap(),
                        solution: folder.part(part.num).unwrap(),
                    });
                }
            }
        }
        targets
    }
}

pub(super) struct Target {
    pub(super) chapter: String,
    pub(super) part: Part,
    pub(super) source_name: Option<String>,
    pub(super) input: Source,
    pub(super) solution: Source,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MainArgs {
    #[command(flatten)]
    targets: TargetArgs,

    /// Show the results in addition to the pass/fail (which is always shown).
    #[arg(short = 'r', long)]
    show_results: bool,
}

#[doc(hidden)]
pub fn main(series: &Series) {
    SERIES.get_or_init(|| series.clone());
    let args = MainArgs::parse();

    let chapters = args.targets.filtered_chapters();
    let targets = args.targets.get_targets(&chapters);
    println!(
        "Running {} solves across {} parts across {} chapters...",
        Cyan.paint(targets.len().to_string()),
        Cyan.paint(
            chapters
                .iter()
                .map(|c| c.parts.len())
                .sum::<usize>()
                .to_string()
        ),
        Cyan.paint(chapters.len().to_string()),
    );

    // Initialize the thread pool now. This will happen automatically when it's first needed, but if this is inside a solution this will add to the runtime of that solution, unfairly penalizing it for being the first to use rayon while the other solutions that also do so get a free pass.
    ThreadPoolBuilder::new().build_global().unwrap();

    let runs: Vec<(String, PartResult)> = targets
        .into_iter()
        .map(|target| {
            let mut name = format!("{} part {}", target.chapter, target.part.num);
            if let Some(source) = target.source_name {
                name = format!("{name} {source}");
            }

            let input = match target.input.read() {
                Ok(input) => input,
                Err(err) => {
                    return (name, PartResult::Error(err));
                }
            };

            (
                name,
                match target.solution.read_maybe() {
                    Ok(solution) => target.part.run::<InstantTimer>(&input, solution),
                    Err(err) => PartResult::Error(err),
                },
            )
        })
        .collect();

    let durations = runs
        .iter()
        .filter_map(|(_, r)| match r {
            PartResult::Success { duration, .. } => Some(*duration),
            PartResult::Error(_) => None,
        })
        .collect::<Vec<_>>();
    let duration_total = durations.iter().sum::<Duration>();
    let duration_avg = if durations.is_empty() {
        Duration::from_secs(0)
    } else {
        duration_total / durations.len() as u32
    };
    let thresholds = DurationThresholds {
        good: duration_avg / 3,
        acceptable: duration_avg * 2 / 3,
    };
    for (name, result) in runs {
        result.print(&name, &thresholds, args.show_results);
    }
    if !durations.is_empty() {
        println!(
            "Finished {} runs in {}, averaging {} per run.",
            Cyan.paint(durations.len().to_string()),
            Purple.paint(format!("{duration_total:?}")),
            Purple.paint(format!("{duration_avg:?}",)),
        );
    }
}
