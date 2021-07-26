//! Structures related to Object Identifier

#[derive(Clone)]
pub struct OIDComponent {
    name: Option<String>,
    number: u32,
}

impl OIDComponent {
    pub fn new(name: Option<String>, number: u32) -> Self {
        Self { name, number }
    }
}

impl std::fmt::Display for OIDComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_some() {
            write!(f, "{}({})", self.name.as_ref().unwrap(), self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

impl std::fmt::Debug for OIDComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
#[derive(Default, Clone)]
pub struct ObjectIdentifier {
    components: Vec<OIDComponent>,
}

impl ObjectIdentifier {
    pub fn new(components: Vec<OIDComponent>) -> Self {
        Self { components }
    }

    pub fn len(self) -> usize {
        self.components.len()
    }
}

impl std::fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((first, rest)) = self.components.split_first() {
            write!(f, "{}", first)?;
            for c in rest {
                write!(f, ".{}", c)?;
            }
        } else {
            write!(f, "EMPTY")?;
        }
        write!(f, "")
    }
}

impl std::fmt::Debug for ObjectIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
