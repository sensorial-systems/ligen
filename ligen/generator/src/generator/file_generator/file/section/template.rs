use ::is_tree::*;

#[derive(IsTree)]
pub struct SectionTemplate {
    #[tree(path_segment)]
    pub name: String,
    pub content: String,
    #[tree(branch)]
    children: Vec<SectionTemplate>
}

impl SectionTemplate {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        let name = name.into();
        let content = content.into();
        let children = Default::default();
        Self { name, content, children }
    }
}

impl From<String> for SectionTemplate {
    fn from(name: String) -> Self {
        Self::new(name, "")
    }
}

// Tree implementation

impl AddBranch<SectionTemplate> for SectionTemplate
{
    fn add_branch(&mut self, branch: SectionTemplate) -> &mut SectionTemplate {
        self.children.push(branch.into());
        self
            .children
            .last_mut()
            .unwrap()
    }
}
