use anyhow::Result;
use std::collections::HashMap;
use streamdeck_choir::{ButtonControl, Config, Control, Instance, MyConfig, Page};

fn main() -> Result<()> {
    {
        const FILE: &str = "config.json";
        let filedata = std::fs::read_to_string(FILE).expect("Unable to read file");

        let _parsed = serde_json::from_str::<Config>(&filedata).expect("Unable to parse JSON");
    }

    let myconfig = {
        const CONFIG_FILE: &str = "config.yml";
        let filedata = std::fs::read_to_string(CONFIG_FILE).expect("Unable to read file");
        serde_yaml::from_str::<MyConfig>(&filedata).expect("Unable to parse YAML")
    };

    let mut config = Config::default();

    let x32_id = uuid::Uuid::new_v4().to_string();

    for p in 1..100 {
        let mut page = Page::default();
        {
            let mut row = HashMap::new();

            for (index, group) in myconfig.groups.iter().enumerate() {
                let button = ButtonControl::new_page_select(&group.name, (index + 1).try_into()?);
                row.insert(index.to_string(), Control::button(button));
            }
            // row.insert("0".to_string(), Control::pageup);
            // row.insert("1".to_string(), Control::pagedown);
            page.controls.insert("0".to_string(), row);
        }

        if let Some(this_controls) = myconfig.groups.get(p - 1) {
            let mut viewrow = HashMap::new();
            let mut controlrow = HashMap::new();
            for (index, (channel_name, channel_num)) in this_controls.channels.iter().enumerate() {
                let button = ButtonControl::new_channel_view(&channel_name, *channel_num);
                viewrow.insert(index.to_string(), Control::button(button));

                let button = ButtonControl::new_channel_rotary(&x32_id, *channel_num, 0.3);
                controlrow.insert(index.to_string(), Control::button(button));
            }
            page.controls.insert("2".to_string(), viewrow);
            page.controls.insert("3".to_string(), controlrow);
        }

        config.pages.insert(p.to_string(), page);
    }

    let x32 = Instance::new("behringer-x32", "x32", 2);
    config.instances.insert(x32_id, x32);

    // write config to a file in json format
    let json = serde_json::to_string_pretty(&config).expect("Unable to serialize config");
    println!("{}", json);

    Ok(())
}
