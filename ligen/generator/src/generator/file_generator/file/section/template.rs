use ::is_tree::*;

#[derive(IsTree)]
pub struct SectionTemplate {
    #[tree(path_segment)]
    pub name: String,
    pub content: String,
    children: Vec<Self>
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

impl<'a> AddBranch<'a> for SectionTemplate
where Self::Branches: KnowsOwned<Owned = SectionTemplate>
{
    fn add_branch(&'a mut self, branch: impl Into<<Self::Branches as KnowsOwned>::Owned>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
        where Self::Branches: KnowsOwned
    {
        self.children.push(branch.into());
        self
            .children
            .last_mut()
            .unwrap()
    }
}

impl<'a> HasBranches<'a> for &'a mut SectionTemplate {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self.children.iter_mut()
    }
}

impl<'a> HasBranches<'a> for &'a SectionTemplate {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self.children.iter()
    }
}
