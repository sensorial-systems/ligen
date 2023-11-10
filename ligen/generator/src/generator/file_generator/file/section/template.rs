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

    pub fn find_child(&self, name: impl AsRef<str>) -> Option<&Self> {
        let name = name.as_ref();
        self.children
            .iter()
            .find(|section| section.name == name)
    }

    pub fn set_child(&mut self, template: impl Into<Self>) {
        let template = template.into();
        self.children.push(template);
    }
}
