#[derive(Clone, Debug)]
pub struct FormattingOptions {
    pub sort_attrs: bool,
    pub sort_flake: bool,
    pub keep_self_first: bool,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            sort_attrs: false,
            sort_flake: false,
            keep_self_first: true,
        }
    }
}