import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.kayra.accounting

Kirigami.ApplicationWindow {
    id: root

    title: "kmuhasebe"
    width: Kirigami.Units.gridUnit * 50
    height: Kirigami.Units.gridUnit * 36
    minimumWidth: Kirigami.Units.gridUnit * 36
    minimumHeight: Kirigami.Units.gridUnit * 26

    property int year: new Date().getFullYear()
    property int month: new Date().getMonth() + 1
    readonly property string monthKey: year + "-" + String(month).padStart(2, "0")
    property var summary: ({ income: 0, expense: 0, balance: 0, incomes: [], expenses: [] })

    function money(cents) {
        return Number(cents / 100).toLocaleCurrencyString(Qt.locale())
    }
    function reload() {
        summary = JSON.parse(ledger.monthJson(monthKey))
    }

    onMonthKeyChanged: reload()
    Component.onCompleted: reload()

    Ledger {
        id: ledger
    }

    function add(kind, section, description, amount) {
        if (ledger.addEntry(monthKey, kind, description, amount)) {
            section.clear()
            reload()
        } else {
            showPassiveNotification("Açıklama boş olamaz; tutar 0'dan büyük bir sayı olmalı (örn. 1250,50)")
        }
    }

    pageStack.initialPage: Kirigami.Page {
        title: Qt.locale().monthName(root.month - 1, Locale.LongFormat) + " " + root.year
        padding: 0

        RowLayout {
            anchors.fill: parent
            spacing: 0

            // ---- Sol kenar çubuğu: aylar ----
            ColumnLayout {
                Layout.preferredWidth: Kirigami.Units.gridUnit * 11
                Layout.fillHeight: true
                spacing: 0

                QQC2.SpinBox {
                    Layout.fillWidth: true
                    Layout.margins: Kirigami.Units.smallSpacing
                    from: 2000
                    to: 2100
                    value: root.year
                    textFromValue: function (v) { return String(v) }
                    onValueModified: root.year = value
                }

                ListView {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    model: 12
                    currentIndex: root.month - 1

                    delegate: QQC2.ItemDelegate {
                        required property int index
                        width: ListView.view.width
                        text: Qt.locale().monthName(index, Locale.LongFormat)
                        highlighted: index === root.month - 1
                        onClicked: root.month = index + 1
                    }
                }
            }

            Kirigami.Separator { Layout.fillHeight: true }

            // ---- Ana ekran: gelir / gider / kalan ----
            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 0

                EntrySection {
                    id: incomeSection
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    title: "Gelir"
                    entries: root.summary.incomes
                    total: root.summary.income
                    accent: Kirigami.Theme.positiveTextColor
                    onAddRequested: (d, a) => root.add("income", incomeSection, d, a)
                    onRemoveRequested: (id) => { ledger.removeEntry(id); root.reload() }
                }

                Kirigami.Separator { Layout.fillWidth: true }

                EntrySection {
                    id: expenseSection
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    title: "Gider"
                    entries: root.summary.expenses
                    total: root.summary.expense
                    accent: Kirigami.Theme.negativeTextColor
                    onAddRequested: (d, a) => root.add("expense", expenseSection, d, a)
                    onRemoveRequested: (id) => { ledger.removeEntry(id); root.reload() }
                }

                Kirigami.Separator { Layout.fillWidth: true }

                // ---- Kalan para ----
                RowLayout {
                    Layout.fillWidth: true
                    Layout.margins: Kirigami.Units.largeSpacing

                    Kirigami.Heading {
                        level: 1
                        text: "Kalan"
                        Layout.fillWidth: true
                    }
                    Kirigami.Heading {
                        level: 1
                        text: root.money(root.summary.balance)
                        color: root.summary.balance < 0
                               ? Kirigami.Theme.negativeTextColor
                               : Kirigami.Theme.positiveTextColor
                    }
                }
            }
        }
    }
}
