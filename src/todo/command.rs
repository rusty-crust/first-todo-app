pub enum Command {
    Add(String),
    List,
    Complete(usize),
    Delete(usize),
}
