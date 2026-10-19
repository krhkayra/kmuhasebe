import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

ColumnLayout {
    id: section

    property string title
    property var entries: []
    property real total: 0          // kuruş
    property color accent: Kirigami.Theme.textColor

    signal addRequested(string description, string amount)
    signal removeRequested(int id)

    function money(cents) {
        return Number(cents / 100).toLocaleCurrencyString(Qt.locale())
    }
    function clear() {
        descField.clear()
        amountField.clear()
        descField.forceActiveFocus()
    }
    function submit() {
        section.addRequested(descField.text, amountField.text)
    }

    spacing: Kirigami.Units.smallSpacing

    RowLayout {
        Layout.fillWidth: true
        Layout.margins: Kirigami.Units.largeSpacing
        Layout.bottomMargin: 0

        Kirigami.Heading {
            level: 2
            text: section.title
            Layout.fillWidth: true
        }
        Kirigami.Heading {
            level: 2
            text: section.money(section.total)
            color: section.accent
        }
    }

    RowLayout {
        Layout.fillWidth: true
        Layout.leftMargin: Kirigami.Units.largeSpacing
        Layout.rightMargin: Kirigami.Units.largeSpacing

        QQC2.TextField {
            id: descField
            Layout.fillWidth: true
            placeholderText: "Açıklama"
            onAccepted: amountField.forceActiveFocus()
        }
        QQC2.TextField {
            id: amountField
            Layout.preferredWidth: Kirigami.Units.gridUnit * 8
            placeholderText: "Tutar (örn. 1250,50)"
            inputMethodHints: Qt.ImhFormattedNumbersOnly
            onAccepted: section.submit()
        }
        QQC2.Button {
            text: "Ekle"
            icon.name: "list-add"
            onClicked: section.submit()
        }
    }

    ListView {
        id: list
        Layout.fillWidth: true
        Layout.fillHeight: true
        clip: true
        model: section.entries

        QQC2.ScrollBar.vertical: QQC2.ScrollBar {}

        delegate: QQC2.ItemDelegate {
            id: row
            required property var modelData
            width: ListView.view.width

            contentItem: RowLayout {
                QQC2.Label {
                    text: row.modelData.description
                    elide: Text.ElideRight
                    Layout.fillWidth: true
                }
                QQC2.Label {
                    text: section.money(row.modelData.cents)
                    color: section.accent
                }
                QQC2.ToolButton {
                    icon.name: "edit-delete"
                    onClicked: section.removeRequested(row.modelData.id)
                    QQC2.ToolTip.text: "Sil"
                    QQC2.ToolTip.visible: hovered
                }
            }
        }

        Kirigami.PlaceholderMessage {
            anchors.centerIn: parent
            width: parent.width - Kirigami.Units.gridUnit * 4
            visible: list.count === 0
            text: "Bu ay için kayıt yok"
        }
    }
}
