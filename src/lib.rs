use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub channels: Vec<(String, String)>,
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
            pages: (1..100).map(|i| (i.to_string(), Page::default())).collect(),
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
    sortOrder: u32,
    isFirstInit: bool,
    config: InstanceConfig,
    enabled: bool,
    lastUpgradeIndex: u32,
}
impl Instance {
    pub fn new(instance_type: &str, label: &str, upindex: u32) -> Self {
        Self {
            instance_type: instance_type.to_string(),
            label: label.to_string(),
            lastUpgradeIndex: upindex,
            sortOrder: 1,
            isFirstInit: false,
            config: InstanceConfig {
                host: "10.0.0.50".to_string(),
                fadeFps: 10,
            },
            enabled: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct InstanceConfig {
    host: String,
    fadeFps: u32,
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
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type")]
pub enum Control {
    button(ButtonControl),
    pageup,
    pagedown,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type")]
pub enum Feedback {
    mute(FeedbackMute),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FeedbackMute {
    id: String,
    instance_id: String,
    options: FeedbackMuteOptions,
    style: ColorStyle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
struct FeedbackMuteOptions {
    target: String,
    state: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct ColorStyle {
    color: u32,
    bgcolor: u32,
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

pub struct ChannelPath<'a> {
    channel_str: &'a str,
}
impl<'a> ChannelPath<'a> {
    pub fn new(c: &'a str) -> Self {
        Self { channel_str: c }
    }
    pub fn view_string(&self) -> Result<String> {
        let channel_as_number = self.channel_str.parse::<u32>();
        if let Ok(channel) = channel_as_number {
            return Ok(format!("$(x32:fader_ch_{:02})", channel));
        }

        let slash_to_underscore = self.channel_str.replace('/', "_");
        Ok(format!("$(x32:fader{slash_to_underscore})"))
    }

    fn set_string(&self) -> String {
        let channel_as_number = self.channel_str.parse::<u32>();
        match channel_as_number {
            Ok(num) => format!("/ch/{:02}", num),
            Err(_) => self.channel_str.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct ButtonControl {
    style: Style,
    options: Options,
    feedbacks: Vec<Feedback>,
    steps: HashMap<String, Step>,
}
impl ButtonControl {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self {
            style: Style::new(text.as_ref()),
            options: Default::default(),
            feedbacks: Default::default(),
            steps: Default::default(),
        }
    }
    pub fn new_page_select(text: impl AsRef<str>, page: u32) -> Self {
        let mut steps = HashMap::new();
        steps.insert("0".to_string(), Step::new_page_select(page));
        Self {
            steps,
            ..Self::new(text)
        }
    }
    pub fn new_channel_view(text: &str, channel: &ChannelPath) -> Result<Self> {
        Ok(Self::new(format!("{text}\n{}", channel.view_string()?)))
    }
    pub fn new_channel_rotary(text: &str, x32_id: &str, channel: &ChannelPath, step: f32) -> Self {
        let mut steps = HashMap::new();
        steps.insert("0".to_string(), Step::fade_channel(x32_id, channel, step));
        Self {
            style: Style::new(text),
            options: Options {
                rotaryActions: Some(true),
                ..Default::default()
            },
            feedbacks: Default::default(),
            steps,
        }
    }
    pub fn add_down_action(mut self, action: Action) -> Self {
        let action_set = self
            .steps
            .entry("0".to_string())
            .or_insert_with(Step::default);
        action_set.action_sets.down.push(action);
        self
    }

    pub fn background_color(mut self, color: u32) -> Self {
        self.style.bgcolor = color;
        self
    }

    pub fn add_mute_feedback(self, x32_id: &str, channel: &ChannelPath<'_>) -> Self {
        self.add_feedback(Feedback::mute(FeedbackMute {
            id: new_id(),
            instance_id: x32_id.to_string(),
            options: {
                FeedbackMuteOptions {
                    target: channel.set_string(),
                    state: true,
                }
            },
            style: ColorStyle {
                color: 0,
                bgcolor: 0xff0000,
            },
        }))
    }

    pub fn add_feedback(mut self, feedback: Feedback) -> Self {
        self.feedbacks.push(feedback);
        self
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
        Self {
            relativeDelay: false,
            stepAutoProgress: true,
            rotaryActions: None,
        }
    }
}

fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
            options: StepOptions {
                runWhileHeld: vec![],
            },
        }
    }

    fn fade_channel(x32_id: &str, channel: &ChannelPath, step: f32) -> Self {
        Self {
            action_sets: ActionSet {
                down: vec![],
                up: vec![],
                rotate_left: Some(vec![Action::fade_channel(x32_id, channel, -step)]),
                rotate_right: Some(vec![Action::fade_channel(x32_id, channel, step)]),
            },
            options: StepOptions {
                runWhileHeld: vec![],
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct StepOptions {
    runWhileHeld: Vec<()>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
pub enum Action {
    set_page_byindex(SetPageByIndexAction),
    fad(FadAction),
    fader_delta(FaderDeltaAction),
    go_scene(GoSceneAction),
    mute(MuteAction),
}
impl Action {
    pub fn fade_channel(x32_id: &str, channel: &ChannelPath, step: f32) -> Self {
        Self::fader_delta(FaderDeltaAction::fade_channel(x32_id, channel, step))
    }
    pub fn set_fader(x32_id: &str, channel: &ChannelPath, value: f32) -> Self {
        Self::fad(FadAction::set_fader(x32_id, channel, value))
    }
    pub fn go_to_scene(x32_id: &str, scene: u32) -> Self {
        Self::go_scene(GoSceneAction {
            id: new_id(),
            instance: x32_id.to_string(),
            options: GoSceneOptions { scene },
            delay: 0,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct MuteAction {
    id: String,
    instance: String,
    options: MuteOptions,
    delay: u32,
}
impl MuteAction {
    pub fn new(x32_id: &str, channel: &ChannelPath) -> Self {
        Self {
            id: new_id(),
            instance: x32_id.to_string(),
            options: MuteOptions {
                target: channel.set_string(),
                mute: 2,
            },
            delay: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct MuteOptions {
    target: String,
    mute: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct GoSceneAction {
    id: String,
    instance: String,
    options: GoSceneOptions,
    delay: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct GoSceneOptions {
    scene: u32,
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
impl FaderDeltaAction {
    fn fade_channel(x32_id: &str, channel: &ChannelPath, step: f32) -> Self {
        Self {
            id: new_id(),
            instance: x32_id.to_string(),
            options: FadDeltaOptions {
                target: channel.set_string(),
                delta: step,
                fadeDuration: 0,
                fadeAlgorithm: "linear".to_string(),
                fadeType: "ease_in".to_string(),
            },
            delay: 0,
        }
    }
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
impl FadAction {
    fn set_fader(x32_id: &str, channel: &ChannelPath, value: f32) -> Self {
        Self {
            id: new_id(),
            instance: x32_id.to_string(),
            options: FadOptions {
                target: channel.set_string(),
                fad: value,
                fadeDuration: 0,
                fadeAlgorithm: "linear".to_string(),
                fadeType: "ease_in".to_string(),
            },
            delay: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(non_snake_case)]
pub struct FadOptions {
    target: String,
    fad: f32,
    fadeDuration: u32,
    fadeAlgorithm: String,
    fadeType: String,
}
