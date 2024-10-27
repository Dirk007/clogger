pub fn extract_name(item: &docker_api::models::ContainerSummary) -> String {
    let default_name = "<UNKNOWN CONTAINER>".to_string();
    let name = item
        .names
        .as_ref()
        .map(|n| n.first())
        .flatten()
        .or(Some(&default_name))
        .expect("ERROR getting container");

    name.to_string()
}

pub fn extract_image(item: &docker_api::models::ContainerSummary) -> String {
    let default_name = "<UNKNOWN IMAGE>".to_string();
    let name = item
        .image
        .as_ref()
        .or(Some(&default_name))
        .expect("ERROR getting image");

    name.to_string()
}
