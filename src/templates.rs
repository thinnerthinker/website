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
    pub icon: String,
    pub disabled: bool,
}

#[derive(Template)]
#[template(path = "navbar.html")]
pub struct Navbar {
    pub links: Vec<NavLink>,
    pub bg_image: String,
    pub route_url: String,
    pub backdrop_css_path: String,
    pub navbar_css_path: String,
}


#[derive(Template)]
#[template(path = "header.html")]
pub struct Header {
    pub bg_image: String,
    pub header_css_path: String,
    pub logo_path: String,
    pub backdrop_css_path: String,
}


#[derive(Template)]
#[template(path = "main.html")]
pub struct Main {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub main_css_path: String,
}

#[derive(Template)]
#[template(path = "projects/sursface.html")]
pub struct ProjectsSursface {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub projects_navbar: Raw<Navbar>,
    pub projects_css_path: String,
    pub main_css_path: String,
}

#[derive(Template)]
#[template(path = "projects/website.html")]
pub struct ProjectsWebsite {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub projects_navbar: Raw<Navbar>,
    pub projects_css_path: String,
    pub main_css_path: String,
}

#[derive(Template)]
#[template(path = "examples/wasm_example.html")]
pub struct WasmExample {
    pub title: String,
    pub example_id: String,
    pub project_id: String,
}

#[derive(Template)]
#[template(path = "examples/sursface.html")]
pub struct ExamplesSursface {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub examples_navbar: Raw<Navbar>,
    pub sursface_examples: Vec<Raw<WasmExample>>,
    pub examples_css_path: String,
    pub main_css_path: String,
    pub show_canvas_js_path: String,
}

#[derive(Template)]
#[template(path = "examples/sayle.html")]
pub struct ExamplesSayle {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub examples_navbar: Raw<Navbar>,
    pub examples_css_path: String,
    pub main_css_path: String,
}