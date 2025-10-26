use std::{
    collections::{HashMap, HashSet},
    env, fs,
    sync::LazyLock,
};

use ansi_term::{Colour, Style};
use chrono::{DateTime, NaiveDate};
use puzzle_lib::prelude::*;
use serde::Deserialize;

static USER_ID: LazyLock<usize> = LazyLock::new(|| {
    env::var("USER_ID")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0)
});

const NAME_MAX_LENGTH: usize = 9;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
struct Data<'a> {
    owner_id: usize,
    event: &'a str,
    members: HashMap<usize, Member<'a>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
struct Member<'a> {
    name: Option<&'a str>,
    id: usize,
    stars: u8,
    local_score: u16,
    global_score: u16,
    last_star_ts: usize,
    completion_day_level: HashMap<u8, HashMap<u8, Part>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
struct Part {
    get_star_ts: usize,
    star_index: usize,
}

#[derive(Debug)]
struct Ranking<T> {
    mapping: HashMap<usize, T>,
    order: Vec<usize>,
}
impl<T> Ranking<T> {
    fn value_by_id(&self, id: usize) -> Option<&T> {
        self.mapping.get(&id)
    }

    fn id_by_rank(&self, rank: usize) -> Option<&usize> {
        self.order.get(rank)
    }

    fn iter(&self) -> impl Iterator<Item = (&usize, &T)> {
        self.order.iter().map(|id| (id, &self.mapping[id]))
    }

    fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    fn reversed(mut self) -> Self {
        self.order.reverse();
        self
    }
}
impl<T> FromIterator<(usize, T)> for Ranking<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = (usize, T)>>(iter: I) -> Self {
        let mapping = HashMap::from_iter(iter);
        let order = mapping
            .iter()
            .map(|(id, value)| (value, id))
            .sort_unstable()
            .map(|(_, id)| *id)
            .collect();
        Self { mapping, order }
    }
}

fn create_name_map<T>(data: &Data, ranking: &Ranking<T>) -> HashMap<usize, (Style, String)> {
    // Assign a color to each of the top 5 for the chosen ranking.
    let mut styles = HashMap::new();
    {
        let unassigned_colours = vec![
            Colour::Green,
            Colour::Blue,
            Colour::Red,
            Colour::Yellow,
            Colour::Purple,
        ];
        ranking
            .iter()
            .map(|(id, _)| id)
            .zip(unassigned_colours)
            .for_each(|(id, colour)| {
                styles.insert(*id, colour.normal());
            });
    }

    // Make the current user bold in addition to whatever color they may have.
    {
        let style = styles.remove(&USER_ID).unwrap_or_default();
        styles.insert(*USER_ID, style.bold());
    }

    data.members
        .values()
        .map(|m| {
            let name = m
                .name
                .map_or_else(|| format!("#{}", m.id), ToOwned::to_owned);

            // Shorten all parts but the first (usually the first name) to a single letter.
            let mut parts = name.split(' ').map(ToOwned::to_owned);
            let first = parts.next().unwrap();
            let rest: String = parts.filter_map(|p| p.chars().next()).collect();
            let mut name = format!("{first} {rest}");
            if name.len() > NAME_MAX_LENGTH {
                name = name[0..NAME_MAX_LENGTH].to_owned();
            }

            let style = styles.remove(&m.id).unwrap_or(Style::new());

            (m.id, (style, name))
        })
        .collect()
}

fn rank_speed_per_solution(data: &Data) -> Vec<((u8, u8), Ranking<usize>)> {
    (1..=25u8)
        .flat_map(|day| {
            (1..=2u8)
                .map(|part| {
                    let ranking = data
                        .members
                        .values()
                        .filter_map(|m| {
                            m.completion_day_level
                                .get(&day)
                                .and_then(|d| d.get(&part))
                                .map(|p| (m.id, p.get_star_ts))
                        })
                        .collect::<Ranking<_>>();
                    ((day, part), ranking)
                })
                .collect::<Vec<_>>()
        })
        .filter(|(_, ranked)| !ranked.is_empty())
        .collect()
}

fn rank_local_score(data: &Data) -> Ranking<u16> {
    data.members
        .values()
        .map(|m| (m.id, m.local_score))
        .collect::<Ranking<_>>()
        .reversed()
}

fn rank_most_wins(speed_per_solution: &[((u8, u8), Ranking<usize>)]) -> Ranking<usize> {
    speed_per_solution
        .iter()
        .filter_map(|(_, ranked)| ranked.id_by_rank(0))
        .copied()
        .count_occurences()
        .into_iter()
        .collect::<Ranking<_>>()
        .reversed()
}

fn format_solve_time(data: &Data, day: u8, time: usize) -> String {
    let start = NaiveDate::from_ymd_opt(data.event.parse().unwrap(), 12, u32::from(day))
        .unwrap()
        .and_hms_opt(5, 0, 0)
        .unwrap()
        .and_utc();
    let solve = DateTime::from_timestamp(time as i64, 0).unwrap();

    let duration = solve - start;
    if duration.num_days() >= 1 {
        ">1 day".to_owned()
    } else {
        format!(
            "{:0>2}:{:0>2}:{:0>2}",
            duration.num_hours(),
            duration.num_minutes() % 60,
            duration.num_seconds() % 60,
        )
    }
}

pub fn main() {
    let path = env::args().nth(1).unwrap();
    let text = fs::read_to_string(path).unwrap();
    let data: Data = serde_json::from_str(&text).unwrap();

    let by_speed_per_solution = rank_speed_per_solution(&data);
    let by_local_score = rank_local_score(&data);
    let by_most_wins = rank_most_wins(&by_speed_per_solution);
    let names = create_name_map(&data, &by_local_score);

    let mut always_show: HashSet<_> = by_local_score.iter().take(5).map(|(id, _)| *id).collect();
    always_show.insert(*USER_ID);

    println!("Top competitors per solution:");
    for ((day, part), ranked) in &by_speed_per_solution {
        print!(" {day:0>2}-{part}:");
        for (rank, (id, solve_ts)) in ranked.iter().enumerate() {
            let rank = if rank < 3 {
                (rank + 1).to_string()
            } else if always_show.contains(id) {
                format!("{:>2}", rank + 1)
            } else {
                continue;
            };

            let (style, name) = &names[&id];
            let solve_time = format!("[{}]", format_solve_time(&data, *day, *solve_ts));
            print!(
                "{}",
                style.paint(format!("  {rank}) {name:NAME_MAX_LENGTH$} {solve_time:10}"))
            );
        }
        println!();
    }

    println!();
    println!("Ranked by score:");
    for (id, score) in by_local_score.iter().take(5) {
        let (style, name) = &names[&id];
        println!("{}", style.paint(format!(" {score:>4} {name}")));
    }

    println!();
    println!("Ranked by first place counts:");
    for (id, wins) in by_most_wins.iter() {
        let (style, name) = &names[&id];
        println!("{}", style.paint(format!(" {wins:>2} {name}")));
    }

    println!();
    println!("Tiebreakers:");
    for (left_rank, right_rank) in
        (0..4).flat_map(|l| ((l + 1)..5).map(|r| (l, r)).collect::<Vec<_>>())
    {
        let Some(left_id) = by_local_score.id_by_rank(left_rank) else {
            break;
        };
        let Some(right_id) = by_local_score.id_by_rank(right_rank) else {
            break;
        };

        let mut left_wins = by_speed_per_solution
            .iter()
            .filter(|(_, ranking)| {
                ranking.value_by_id(*left_id).unwrap_or(&usize::MAX)
                    < ranking.value_by_id(*right_id).unwrap_or(&usize::MAX)
            })
            .count();
        let mut right_wins = by_speed_per_solution
            .iter()
            .filter(|(_, ranking)| {
                ranking.value_by_id(*left_id).unwrap_or(&usize::MAX)
                    > ranking.value_by_id(*right_id).unwrap_or(&usize::MAX)
            })
            .count();

        let (mut left_style, mut left_name) = names[&left_id].clone();
        let (mut right_style, mut right_name) = names[&right_id].clone();

        if left_wins < right_wins {
            (left_style, right_style) = (right_style, left_style);
            (left_name, right_name) = (right_name, left_name);
            (left_wins, right_wins) = (right_wins, left_wins);
        }

        if left_wins > right_wins {
            println!(
                " {} beat {} ({} vs {})",
                left_style.paint(format!("{left_name:>NAME_MAX_LENGTH$}")),
                right_style.paint(format!("{right_name:<NAME_MAX_LENGTH$}")),
                left_style.paint(left_wins.to_string()),
                right_style.paint(right_wins.to_string()),
            );
        } else {
            println!(
                " {} ties {} ({} each)",
                left_style.paint(format!("{left_name:>NAME_MAX_LENGTH$}")),
                right_style.paint(format!("{right_name:<NAME_MAX_LENGTH$}")),
                left_wins,
            );
        }
    }
}
