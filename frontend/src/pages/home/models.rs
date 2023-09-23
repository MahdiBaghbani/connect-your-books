// `Model` describes our app state.
pub struct Model {
    pub team: Vec<TeamMember>,
}

#[derive(Clone, Debug)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub description: String,
    pub image_url: String,
    pub links: Option<Vec<TeamMemberLink>>,
}

#[derive(Clone, Debug)]
pub struct TeamMemberLink {
    pub logo: Option<String>,
    pub url: Option<String>,
}
