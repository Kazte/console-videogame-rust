use csv::{ReaderBuilder, StringRecord};
use std::{collections::HashMap, fs, io::stdin};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct HistoryNode {
    node_type: String,
    tag: String,
    text: String,
    health: i32,
    options: Vec<HistoryNode>,
}

impl HistoryNode {
    fn new(row: StringRecord) -> HistoryNode {
        let node_type = row.get(0).unwrap().trim().to_string();
        let tag = row.get(1).unwrap().trim().to_string();
        let text = row.get(2).unwrap().trim().to_string();
        let health: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);

        let node = HistoryNode {
            node_type: node_type,
            tag: tag,
            text: text,
            health: health,
            options: vec![],
        };

        return node;
    }
}

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut last_record: String = "".to_string();
    let mut history: HashMap<String, HistoryNode> = HashMap::new();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let node = HistoryNode::new(result);

        if node.node_type == "SITUACION" {
            let nodetag = node.tag.clone();
            history.insert(node.tag.clone(), node);
            last_record = nodetag;
        } else if node.node_type == "OPCION" {
            if let Some(data) = history.get_mut(&last_record) {
                (*data).options.push(node);
            }
        }
    }

    let mut health = 100;
    let mut current_tag = FIRST_TAG;

    // GAME LOOP
    loop {
        print!("{}[2J", 27 as char);
        println!("Your current health is {}", health);

        if let Some(data) = history.get(current_tag) {
            println!("");
            println!("{:?}", data.text);

            for (index, option) in data.options.iter().enumerate() {
                println!("[{}]: {}", index, option.text);
            }

            let mut selection = String::new();

            std::io::stdin().read_line(&mut selection).unwrap();

            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selection) = &data.options.get(selection) {
                current_tag = &selection.tag;
            } else {
                println!("Not valid Option.");
            }

            health += data.health;
            println!("------------------------------------------------------------------");
        } else {
            break;
        }

        if health <= 0 {
            println!("You died!");
            break;
        }
    }
}
