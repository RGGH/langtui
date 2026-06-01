use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CellStats {
    pub ok:  u32,
    pub err: u32,
}

impl CellStats {
    pub fn total(&self) -> u32 { self.ok + self.err }
    pub fn error_pct(&self) -> f64 {
        if self.total() == 0 { 0.0 } else { self.err as f64 / self.total() as f64 }
    }
}

/// verb → tense → person → CellStats
pub type DrillStats = HashMap<String, HashMap<String, HashMap<String, CellStats>>>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub drill: DrillStats,
    /// date-string (YYYY-MM-DD) → answer count
    pub streak: HashMap<String, u32>,
}

impl Stats {
    pub fn record(&mut self, verb: &str, tense: &str, person: &str, correct: bool) {
        let cell = self
            .drill
            .entry(verb.to_string()).or_default()
            .entry(tense.to_string()).or_default()
            .entry(person.to_string()).or_default();
        if correct { cell.ok += 1; } else { cell.err += 1; }

        let today = chrono_today();
        *self.streak.entry(today).or_default() += 1;
    }

    pub fn cell(&self, verb: &str, tense: &str) -> CellStats {
        use crate::data::PERSONS;
        let mut total = CellStats::default();
        for p in PERSONS {
            if let Some(c) = self.drill.get(verb)
                .and_then(|t| t.get(tense))
                .and_then(|p2| p2.get(*p))
            {
                total.ok  += c.ok;
                total.err += c.err;
            }
        }
        total
    }
}

pub fn stats_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".verbdrill_stats.json")
}

pub fn load() -> Stats {
    let path = stats_path();
    if path.exists() {
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(s) = serde_json::from_str(&text) {
                return s;
            }
        }
    }
    Stats::default()
}

pub fn save(stats: &Stats) {
    let path = stats_path();
    if let Ok(text) = serde_json::to_string_pretty(stats) {
        let _ = std::fs::write(path, text);
    }
}

fn chrono_today() -> String {
    // Use std time — no chrono dep needed
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days_since_epoch = secs / 86400;
    // days since 1970-01-01
    let (y, m, d) = days_to_ymd(days_since_epoch as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

fn days_to_ymd(mut z: i64) -> (i64, i64, i64) {
    z += 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y   = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp  = (5 * doy + 2) / 153;
    let d   = doy - (153 * mp + 2) / 5 + 1;
    let m   = if mp < 10 { mp + 3 } else { mp - 9 };
    let y   = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

pub fn today_str() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days = secs / 86400;
    let (y, m, d) = days_to_ymd(days as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Returns the last `n` date strings (oldest first)
pub fn last_n_days(n: u32) -> Vec<String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let today_days = (secs / 86400) as i64;
    (0..n as i64).rev().map(|i| {
        let (y, m, d) = days_to_ymd(today_days - i);
        format!("{y:04}-{m:02}-{d:02}")
    }).collect()
}

/// day-of-week: 0=Mon … 6=Sun
pub fn dow(date_str: &str) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let today_secs = (secs / 86400) as i64;
    // count days from today to the target date
    let today = today_str();
    let diff = date_diff_days(&today, date_str);
    let today_dow = (today_secs + 3) % 7; // 1970-01-01 was Thursday=3
    ((today_dow - diff).rem_euclid(7)) as usize
}

fn date_diff_days(a: &str, b: &str) -> i64 {
    // a - b in days (positive if a is later)
    fn parse(s: &str) -> i64 {
        let y: i64 = s[0..4].parse().unwrap_or(0);
        let m: i64 = s[5..7].parse().unwrap_or(0);
        let d: i64 = s[8..10].parse().unwrap_or(0);
        ymd_to_days(y, m, d)
    }
    parse(a) - parse(b)
}

fn ymd_to_days(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}
