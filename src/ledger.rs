use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

use crate::store::{parse_cents, Book, Kind};

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Ledger = super::LedgerRust;

        /// Verilen ay ("YYYY-AA") için özet + kayıtlar, JSON olarak.
        #[qinvokable]
        #[cxx_name = "monthJson"]
        fn month_json(self: &Ledger, month: &QString) -> QString;

        /// Başarılıysa true döner. `kind`: "income" | "expense".
        #[qinvokable]
        #[cxx_name = "addEntry"]
        fn add_entry(
            self: Pin<&mut Ledger>,
            month: &QString,
            kind: &QString,
            description: &QString,
            amount: &QString,
        ) -> bool;

        /// Başarılıysa true döner.
        #[qinvokable]
        #[cxx_name = "removeEntry"]
        fn remove_entry(self: Pin<&mut Ledger>, id: i64) -> bool;
    }
}

pub struct LedgerRust {
    book: Book,
}

impl Default for LedgerRust {
    fn default() -> Self {
        Self { book: Book::load() }
    }
}

impl qobject::Ledger {
    pub fn month_json(&self, month: &QString) -> QString {
        let month = month.to_string();
        let json = serde_json::to_string(&self.book.summary(&month)).unwrap_or_else(|_| "{}".into());
        QString::from(json.as_str())
    }

    pub fn add_entry(
        mut self: Pin<&mut Self>,
        month: &QString,
        kind: &QString,
        description: &QString,
        amount: &QString,
    ) -> bool {
        let (Some(kind), Some(cents)) = (
            Kind::parse(&kind.to_string()),
            parse_cents(&amount.to_string()),
        ) else {
            return false;
        };

        let mut rust = self.as_mut().rust_mut();
        match rust
            .book
            .add(&month.to_string(), kind, &description.to_string(), cents)
        {
            None => false,
            Some(id) => match rust.book.save() {
                Ok(()) => true,
                Err(e) => {
                    eprintln!("accounting: kaydedilemedi: {e}");
                    rust.book.remove(id); // diske yazılamadıysa belleği de geri al
                    false
                }
            },
        }
    }

    pub fn remove_entry(mut self: Pin<&mut Self>, id: i64) -> bool {
        let mut rust = self.as_mut().rust_mut();
        if !rust.book.remove(id) {
            return false;
        }
        match rust.book.save() {
            Ok(()) => true,
            Err(e) => {
                eprintln!("accounting: kaydedilemedi: {e}");
                false
            }
        }
    }
}
