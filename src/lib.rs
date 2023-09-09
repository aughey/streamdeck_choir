use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct Config {
    version: u32,
    r#type: String,
    pages: HashMap<String,Page>,
    instances: HashMap<String,Instance>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Instance {
    instance_type: String,
    label: String,
    lastUpgradeIndex: u32,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)] 
#[allow(non_snake_case)]
pub struct Page {
    name: String,
    controls: HashMap<String,HashMap<String,Control>>,
    gridSize: Gridsize
}

#[derive(Debug,Serialize,Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type")]
enum Control {
    button(ButtonControl),
    pageup(PageChange),
    pagedown(PageChange),
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)] 
pub struct PageChange {
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)] 
#[allow(non_snake_case)]
pub struct Gridsize {
    minColumn: u32,
    maxColumn: u32,
    minRow: u32,
    maxRow: u32
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct ButtonControl {
    style: Style,
    options: Options,
    feedbacks: Vec<()>,
    steps: HashMap<String,Step>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Style {
    text: String,
    size: String,
    png: Option<()>,
    alignment: String,
    pngalignment: String,
    color: u32,
    bgcolor: u32,
    show_topbar: String
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Options {
    relativeDelay: bool,
    stepAutoProgress: bool,
    rotaryActions: Option<bool>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct Step {
    action_sets: ActionSet,
    options: StepOptions
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct StepOptions {
    runWhileHeld: Vec<()>
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct ActionSet {
    down: Vec<Action>,
    up: Vec<Action>,
    rotate_left: Option<Vec<Action>>,
    rotate_right: Option<Vec<Action>>
}

#[derive(Debug,Serialize,Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "action")]
enum Action {
    set_page_byindex(SetPageByIndexAction),
    fad(FadAction),
    fader_delta(FaderDeltaAction),
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FaderDeltaAction {
    id: String,
    instance: String,
    options: FadDeltaOptions,
    delay: u32
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FadDeltaOptions {
    target: String,
    delta: f32,
    fadeDuration: u32,
    fadeAlgorithm: String,
    fadeType: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct SetPageByIndexAction {
    id: String,
    instance: String,
    options: PageOptions,
    delay: u32
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct PageOptions {
    controller: u32,
    page_from_variable: bool,
    page: u32,
    page_variable: String,
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FadAction {
    id: String,
    instance: String,
    options: FadOptions,
    delay: u32
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FadOptions {
    target: String,
    fad: u32,
    fadeDuration: u32,
    fadeAlgorithm: String,
    fadeType: String,
}