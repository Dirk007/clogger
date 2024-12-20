use super::helper;

pub struct FindQuery {
    last: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
}

impl FindQuery {
    pub fn new(find_text: impl Into<String>) -> Self {
        let text = find_text.into().to_lowercase();
        FindQuery {
            last: None,
            name: Some(text.clone()),
            image: Some(text),
        }
    }

    pub fn is_match(&self, container: &docker_api::models::ContainerSummary) -> bool {
        if self.last == container.id {
            return false;
        }
        let cname = helper::extract_name(container).to_lowercase();
        if let Some(name) = &self.name {
            if cname.contains(name) {
                return true;
            }
        }

        if let (Some(image), Some(cimage)) = (&self.image, &container.image) {
            if cimage.to_lowercase().contains(image) {
                return true;
            }
        }

        false
    }
}
