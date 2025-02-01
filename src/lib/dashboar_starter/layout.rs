use crate::data::starting_gui_state;
use dashboar::{
    BoolButton, BoolButtonState, ButtonState, DashboarTx, Div, Form, InputField, SelectInputField,
    SelectOption, Tab, TableFromData, Tabs, Td, Text, TextInputField, Ui, Value,
};
use serde_json::json;

const ENABLED_COLOR: &str = "#377D22";
const DISABLED_COLOR: &str = "#2E327D";

fn switch_button(on_text: &str, off_text: &str) -> BoolButtonState {
    BoolButtonState::builder()
        .on(ButtonState::builder()
            .value(Value::Fixed(String::from(on_text)))
            .color(String::from(ENABLED_COLOR))
            .build())
        .off(
            ButtonState::builder()
                .value(Value::Fixed(String::from(off_text)))
                .color(String::from(DISABLED_COLOR))
                .build(),
        )
        .build()
}

fn ptr_text_td(s: &str) -> Td {
    Text::builder()
        .value(Value::Pointer(String::from(s)))
        .build_td()
}

fn status_tab() -> Vec<Ui> {
    let connections = Div::builder()
        .children(vec![
            Text::builder()
                .value(Value::Fixed(String::from("This Demo simulates a Dashboard to control faulty Connections to Servers in different locations")))
                .build_ui(),
            Text::builder()
                .value(Value::Fixed(String::from("You can Enable/Disable a Connection by pressing the Connect Button")))
                .build_ui(),
            Text::builder()
                .value(Value::Fixed(String::from("The Connection Status will randomly go OFFLINE, because the Connections are faulty")))
                .build_ui(),
            Text::builder()
                .value(Value::Fixed(String::from("When a Connection goes OFFLINE, it will retry once, and if it fails again, it will stop connecting")))
                .build_ui(),
            Text::builder()
                .value(Value::Fixed(String::from("If the Connection stops working, you can turn click the Connect button to try again")))
                .build_ui(),
            TableFromData::builder()
                .pointer(String::from("/connections"))
                .header(vec![
                    String::from("Connect"),
                    String::from("Status"),
                    String::from("Name"),
                    String::from("Server Location"),
                    String::from("ID"),
                ])
                .row_template(vec![
                    BoolButton::builder()
                        .pointer(String::from("/flag"))
                        .state(switch_button("YES", "NO"))
                        .on_click(
                            DashboarTx::Msg { template: json!({
                                "action": "connection_flag",
                                "id": "$ref/id",
                                "connect_flag": "!$ref/flag"
                            })})
                        .build_td(),
                    BoolButton::builder()
                        .pointer(String::from("/status"))
                        .state(switch_button("ONLINE", "OFFLINE"))
                        .build_td(),
                    ptr_text_td("/name"),
                    ptr_text_td("/server_location"),
                    ptr_text_td("/id"),
                ])
                .build_ui(),
        ])
        .build_ui();

    vec![connections]
}

fn settings_tab() -> Vec<Ui> {
    let options = starting_gui_state()
        .connections
        .iter()
        .map(|c| c.id.clone())
        .map(|id| SelectOption {
            text: id.clone(),
            value: id.clone(),
        })
        .collect();

    let settings = Div::builder()
        .children(vec![
            Text::builder()
                .value(Value::Fixed(String::from("This demonstrates a Form, used to change the Name of the Connections in the other Tab")))
                .build_ui(),
            Form::builder()
                .on_submit(DashboarTx::Msg {
                    template: json!({
                        "action": "change_name",
                        "id": "$ref/id",
                        "new_name": "$ref/new_name"
                    })
                })
                .fields(vec![
                    InputField::Select(SelectInputField {
                        label: "ID".to_string(),
                        name: "id".to_string(),
                        options,
                    }),
                    InputField::Text(TextInputField {
                        label: "New Name".to_string(),
                        name: "new_name".to_string(),
                    })
                ])
                .build_ui()
        ])
        .build_ui();

    vec![settings]
}

pub fn layout() -> Vec<Ui> {
    vec![Ui::Tabs(Tabs {
        tabs: Some(vec![
            Tab {
                name: "Status".to_string(),
                contents: status_tab(),
            },
            Tab {
                name: "Settings".to_string(),
                contents: settings_tab(),
            },
        ]),
        max_height: None,
    })]
}
