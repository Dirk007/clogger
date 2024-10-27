pub struct FindQuery {
    pub name: Option<String>,
    pub image: Option<String>,
}

impl FindQuery {
    pub fn new(find_text: impl Into<String>) -> Self {
        let text = find_text.into();
        FindQuery {
            name: Some(text.clone()),
            image: Some(text),
        }
    }

    pub fn is_match(&self, container: &docker_api::models::ContainerSummary) -> bool {
        if let Some(name) = &self.name {
            if container.names.iter().any(|n| n.contains(name)) {
                return true;
            }
        }

        if let (Some(image), Some(cimage)) = (&self.image, &container.image) {
            if cimage.contains(image) {
                return true;
            }
        }

        false
    }
}
