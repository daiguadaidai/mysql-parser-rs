// LinesClause represents lines references clause in load data statement.
#[derive(Debug)]
pub struct LinesClause {
    pub starting: Option<String>,
    pub terminated: Option<String>,
}