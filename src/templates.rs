use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
pub struct Main {
    pub links: Vec<NavLink>
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct Projects {
    pub links: Vec<NavLink>
}

#[derive(Template)]
#[template(path = "examples.html")]
pub struct Examples {
    pub links: Vec<NavLink>
}


pub struct NavLink {
    pub name: &'static str,
    pub url: &'static str,
}

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct Navbar {
    pub links: Vec<NavLink>,
}
