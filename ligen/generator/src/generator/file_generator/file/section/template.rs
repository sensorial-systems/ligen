use ::is_tree::*;

pub struct SectionTemplate {
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

// Tree implementation

impl HasPathSegment for SectionTemplate {
    fn path_segment(&self) -> &String {
        &self.name
    }
}

impl<'a> KnowsBranches<'a> for &'a SectionTemplate {
    type Branches = &'a SectionTemplate;
}

impl<'a> KnowsBranches<'a> for &'a mut SectionTemplate {
    type Branches = &'a mut SectionTemplate;
}

impl<'a> KnowsOwned for SectionTemplate {
    type Owned = SectionTemplate;
}

impl<'a> AddBranch<'a> for &'a mut SectionTemplate
where Self::Branches: KnowsOwned<Owned = SectionTemplate>
{
    fn add_branch(self, branch: impl Into<<Self::Branches as KnowsOwned>::Owned>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
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

impl<'a> HasGet<'a> for &'a SectionTemplate {}

impl<'a> HasGet<'a> for &'a mut SectionTemplate {}

impl<'a> HasGetOrCreate<'a> for &'a mut SectionTemplate
where Self::Branches: KnowsOwned<Owned = SectionTemplate>
{
    fn branch(self, segment: impl Into<String>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned
    {
        let segment = segment.into();
        let self_ = unsafe { &mut *(self as *mut SectionTemplate) }; // FIXME: This is a repetitive safe workaround. is-tree should provide a safe way to do this.
        if let Some(branch) = self.get(&segment) {
            branch
        } else {
            self_.add_branch(SectionTemplate::new(segment, ""))
        }
    }
}