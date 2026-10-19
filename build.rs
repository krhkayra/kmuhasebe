use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "io.github.kayra.accounting",
            rust_files: &["src/ledger.rs"],
            qml_files: &["qml/Main.qml", "qml/EntrySection.qml"],
            ..Default::default()
        })
        .build();
}
