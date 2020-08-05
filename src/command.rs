#[derive(PartialEq, Debug)]
pub enum Command {
    MV { source: String, target: String },
    RM(String),
    NoOp,
}
