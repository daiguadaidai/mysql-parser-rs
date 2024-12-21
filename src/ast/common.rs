#[derive(Debug)]
pub enum ASTType {
    Terminated,
    Enclosed,
    Escaped,
    DefinedNullBy,
}

pub const FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE: isize = 0;
pub const FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE: isize = 1;
pub const FULLTEXT_SEARCH_MODIFIER_MODE_MASK: isize = 0xF;
pub const FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION: isize = 1 << 4;

#[derive(Debug, Default)]
pub struct FulltextSearchModifier {
    pub v: isize,
}

impl FulltextSearchModifier {
    pub fn new(v: isize) -> FulltextSearchModifier {
        FulltextSearchModifier { v }
    }

    pub fn is_bool_mode(&self) -> bool {
        self.v & FULLTEXT_SEARCH_MODIFIER_MODE_MASK == FULLTEXT_SEARCH_MODIFIER_BOOLEAN_MODE
    }

    pub fn is_natural_language_mode(&self) -> bool {
        self.v & FULLTEXT_SEARCH_MODIFIER_MODE_MASK
            == FULLTEXT_SEARCH_MODIFIER_NATURAL_LANGUAGE_MODE
    }

    pub fn with_query_expansion(&self) -> bool {
        self.v & FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION
            == FULLTEXT_SEARCH_MODIFIER_WITH_QUERY_EXPANSION
    }
}
