use askama::Template;


pub struct Raw<T> {
    pub html: String,
    _marker: core::marker::PhantomData<T>,
}

impl<T> Raw<T> {
    pub fn new(html: String) -> Self {
        Raw {
            html,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T: Template> Raw<T> {
    pub fn to_raw(template: T) -> Raw<T> {
        let html = template.render().unwrap();
        Raw::new(html)
    }
}


pub struct NavLink {
    pub name: &'static str,
    pub url: &'static str,
    pub route: &'static str,
}

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct Navbar {
    pub links: Vec<NavLink>,
    pub bg_image: String,
    pub route_url: String,
}


#[derive(Template)]
#[template(path = "header.html")]
pub struct Header {
    pub bg_image: String,
}


#[derive(Template)]
#[template(path = "main.html")]
pub struct Main {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct Projects {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
}

#[derive(Template)]
#[template(path = "examples/sursface.html")]
pub struct ExamplesSursface {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub examples_navbar: Raw<Navbar>,
}