use streamdeck_choir::Config;

fn main() {
    const FILE : &str = "config.json";
    let filedata = std::fs::read_to_string(FILE).expect("Unable to read file");

    let parsed = serde_json::from_str::<Config>(&filedata).expect("Unable to parse JSON");

    println!("{:#?}", parsed);
}
