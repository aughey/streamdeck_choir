use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub channels: Vec<(String, u32)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    version: u32,
    r#type: String,
    pub pages: HashMap<String, Page>,
    pub instances: HashMap<String, Instance>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            version: 4,
            r#type: "full".to_string(),
            pages: (1..100)
                .into_iter()
                .map(|i| (i.to_string(), Page::default()))
                .collect(),
            instances: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Instance {
    instance_type: String,
    label: String,
    lastUpgradeIndex: u32,
}
impl Instance {
    pub fn new(instance_type: &str, label: &str, upindex: u32) -> Self {
        Self {
            instance_type: instance_type.to_string(),
            label: label.to_string(),
            lastUpgradeIndex: upindex,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Page {
    name: String,
    pub controls: HashMap<String, HashMap<String, Control>>,
    gridSize: Gridsize,
}
impl Default for Page {
    fn default() -> Self {
        Self {
            name: "PAGE".to_string(),
            controls: Default::default(),
            gridSize: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type")]
pub enum Control {
    button(ButtonControl),
    pageup,
    pagedown,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// pub struct PageChange {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Gridsize {
    minColumn: u32,
    maxColumn: u32,
    minRow: u32,
    maxRow: u32,
}
impl Default for Gridsize {
    fn default() -> Self {
        Self {
            minColumn: 0,
            maxColumn: 3,
            minRow: 0,
            maxRow: 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct ButtonControl {
    style: Style,
    options: Options,
    feedbacks: Vec<()>,
    steps: HashMap<String, Step>,
}
impl ButtonControl {
   pub fn new_page_select(text: impl AsRef<str>, page: u32) -> Self {
        let mut steps = HashMap::new();
        steps.insert("0".to_string(), Step::new_page_select(page));
        Self {
            style: Style::new(text.as_ref()),
            options: Default::default(),
            feedbacks: Default::default(),
            steps: steps
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Style {
    text: String,
    size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    png: Option<()>,
    alignment: String,
    pngalignment: String,
    color: u32,
    bgcolor: u32,
    show_topbar: String,
}
impl Style {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            size: "auto".to_string(),
            png: None,
            alignment: "center:center".to_string(),
            pngalignment: "center:center".to_string(),
            color: 16777215,
            bgcolor: 0,
            show_topbar: "default".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Options {
    relativeDelay: bool,
    stepAutoProgress: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotaryActions: Option<bool>,
}
impl Default for Options {
    fn default() -> Self {
        Self { relativeDelay: false, stepAutoProgress: true, rotaryActions: None }
    }
}

fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Step {
    action_sets: ActionSet,
    options: StepOptions,
}
impl Step {
    fn new_page_select(page: u32) -> Self {
        Self {
            action_sets: ActionSet {
                down: vec![Action::set_page_byindex(SetPageByIndexAction {
                    id: new_id(),
                    instance: "internal".to_string(),
                    options: PageOptions {
                        controller: 0,
                        page_from_variable: false,
                        page,
                        page_variable: page.to_string(),
                    },
                    delay: 0,
                })],
                up: vec![],
                rotate_left: None,
                rotate_right: None,
            },
            options: StepOptions { runWhileHeld: vec![] },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct StepOptions {
    runWhileHeld: Vec<()>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct ActionSet {
    down: Vec<Action>,
    up: Vec<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotate_left: Option<Vec<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rotate_right: Option<Vec<Action>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "action")]
enum Action {
    set_page_byindex(SetPageByIndexAction),
    fad(FadAction),
    fader_delta(FaderDeltaAction),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FaderDeltaAction {
    id: String,
    instance: String,
    options: FadDeltaOptions,
    delay: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FadDeltaOptions {
    target: String,
    delta: f32,
    fadeDuration: u32,
    fadeAlgorithm: String,
    fadeType: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SetPageByIndexAction {
    id: String,
    instance: String,
    options: PageOptions,
    delay: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct PageOptions {
    controller: u32,
    page_from_variable: bool,
    page: u32,
    page_variable: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FadAction {
    id: String,
    instance: String,
    options: FadOptions,
    delay: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FadOptions {
    target: String,
    fad: u32,
    fadeDuration: u32,
    fadeAlgorithm: String,
    fadeType: String,
}
