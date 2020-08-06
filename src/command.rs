#[derive(PartialEq, Debug)]
pub enum Command {
    MV { source: String, target: String },
    RM(String),
    NoOp,
}

#[derive(PartialEq, Debug)]
pub struct Commands {
    pub elements: Vec<Command>,
}
