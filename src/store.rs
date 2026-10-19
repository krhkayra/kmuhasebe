//! Qt'den bağımsız veri katmanı: kayıtlar, doğrulama, diske yazma.

use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Income,
    Expense,
}

impl Kind {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "income" => Some(Kind::Income),
            "expense" => Some(Kind::Expense),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub id: i64,
    /// "YYYY-AA" biçiminde, örn. "2026-10"
    pub month: String,
    pub kind: Kind,
    pub description: String,
    /// Kuruş cinsinden (kayan nokta hatalarından kaçınmak için tamsayı).
    pub cents: i64,
}

#[derive(Serialize)]
pub struct Summary<'a> {
    pub income: i64,
    pub expense: i64,
    pub balance: i64,
    pub incomes: Vec<&'a Entry>,
    pub expenses: Vec<&'a Entry>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Book {
    next_id: i64,
    entries: Vec<Entry>,
}

fn data_path() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| {
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share"))
        })
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("accounting").join("data.json")
}

impl Book {
    pub fn load() -> Self {
        let path = data_path();
        let text = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(_) => return Self::default(), // ilk çalıştırma
        };
        match serde_json::from_str(&text) {
            Ok(book) => book,
            Err(e) => {
                // Bozuk dosyanın üzerine yazıp veriyi kaybetmemek için yedekle.
                eprintln!("accounting: veri dosyası okunamadı ({e}); yedekleniyor");
                let _ = fs::copy(&path, path.with_extension("json.bozuk"));
                Self::default()
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let path = data_path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_vec_pretty(self)?)?;
        fs::rename(tmp, path) // atomik değiştirme
    }

    pub fn add(&mut self, month: &str, kind: Kind, description: &str, cents: i64) -> Option<i64> {
        let description = description.trim();
        if !valid_month(month) || description.is_empty() || cents <= 0 {
            return None;
        }
        self.next_id += 1;
        let id = self.next_id;
        self.entries.push(Entry {
            id,
            month: month.to_owned(),
            kind,
            description: description.to_owned(),
            cents,
        });
        Some(id)
    }

    pub fn remove(&mut self, id: i64) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() != before
    }

    pub fn summary(&self, month: &str) -> Summary<'_> {
        let in_month = |k: Kind| -> Vec<&Entry> {
            self.entries
                .iter()
                .filter(|e| e.month == month && e.kind == k)
                .collect()
        };
        let incomes = in_month(Kind::Income);
        let expenses = in_month(Kind::Expense);
        let income: i64 = incomes.iter().map(|e| e.cents).sum();
        let expense: i64 = expenses.iter().map(|e| e.cents).sum();
        Summary { income, expense, balance: income - expense, incomes, expenses }
    }
}

pub fn valid_month(m: &str) -> bool {
    let b = m.as_bytes();
    b.len() == 7
        && b[4] == b'-'
        && b[..4].iter().all(u8::is_ascii_digit)
        && matches!(&m[5..], "01" | "02" | "03" | "04" | "05" | "06" | "07" | "08" | "09" | "10" | "11" | "12")
}

/// "1250", "1250,5", "1250.50" -> kuruş. Binlik ayracı kabul edilmez.
pub fn parse_cents(input: &str) -> Option<i64> {
    let s = input.trim().replace(',', ".");
    let (whole, frac) = s.split_once('.').unwrap_or((s.as_str(), ""));
    if (whole.is_empty() && frac.is_empty())
        || frac.len() > 2
        || !whole.bytes().all(|c| c.is_ascii_digit())
        || !frac.bytes().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let w: i64 = if whole.is_empty() { 0 } else { whole.parse().ok()? };
    let f: i64 = match frac.len() {
        0 => 0,
        1 => frac.parse::<i64>().ok()? * 10,
        _ => frac.parse().ok()?,
    };
    w.checked_mul(100)?.checked_add(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amounts() {
        assert_eq!(parse_cents("1250"), Some(125000));
        assert_eq!(parse_cents("1250,5"), Some(125050));
        assert_eq!(parse_cents("0.99"), Some(99));
        assert_eq!(parse_cents(".5"), Some(50));
        assert_eq!(parse_cents("1,234.50"), None);
        assert_eq!(parse_cents("abc"), None);
        assert_eq!(parse_cents("1.234"), None);
        assert_eq!(parse_cents(""), None);
    }

    #[test]
    fn months() {
        assert!(valid_month("2026-10"));
        assert!(!valid_month("2026-13"));
        assert!(!valid_month("26-10"));
    }

    #[test]
    fn balance() {
        let mut b = Book::default();
        b.add("2026-10", Kind::Income, "Maaş", 100_000).unwrap();
        b.add("2026-10", Kind::Expense, "Kira", 40_000).unwrap();
        b.add("2026-11", Kind::Expense, "Başka ay", 1).unwrap();
        let s = b.summary("2026-10");
        assert_eq!((s.income, s.expense, s.balance), (100_000, 40_000, 60_000));
    }
}
