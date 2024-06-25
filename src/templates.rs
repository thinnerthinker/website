use askama::Template;
use crate::data::static_paths::StaticPaths;

#[derive(Default)]
pub struct Raw<T> {
    pub html: String,
    _marker: core::marker::PhantomData<T>,
}

pub struct RawAny {
    pub html: String,
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

#[derive(Default)]
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

impl Default for Navbar {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            links: vec![],
            bg_image: sp.bg_image.clone(),
            route_url: "/".to_string(),
            backdrop_css_path: sp.backdrop_css_path.clone(),
            navbar_css_path: sp.navbar_css_path.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "header.html")]
pub struct Header {
    pub bg_image: String,
    pub header_css_path: String,
    pub logo_path: String,
    pub backdrop_css_path: String,
}

impl Default for Header {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            bg_image: sp.bg_image.clone(),
            header_css_path: sp.header_css_path.clone(),
            logo_path: sp.logo_path.clone(),
            backdrop_css_path: sp.backdrop_css_path.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "main.html")]
pub struct Main {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub main_css_path: String,
}

impl Default for Main {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            main_navbar: Raw::to_raw(Navbar::default()),
            header: Raw::to_raw(Header::default()),
            main_css_path: sp.main_css_path.clone(),
        }
    }
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

impl Default for ProjectsSursface {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            main_navbar: Raw::to_raw(Navbar::default()),
            header: Raw::to_raw(Header::default()),
            projects_navbar: Raw::to_raw(Navbar::default()),
            projects_css_path: sp.projects_css_path.clone(),
            main_css_path: sp.main_css_path.clone(),
        }
    }
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

impl Default for ProjectsWebsite {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            main_navbar: Raw::to_raw(Navbar::default()),
            header: Raw::to_raw(Header::default()),
            projects_navbar: Raw::to_raw(Navbar::default()),
            projects_css_path: sp.projects_css_path.clone(),
            main_css_path: sp.main_css_path.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "examples/wasm_example.html")]
pub struct WasmExample {
    pub title: String,
    pub example_id: String,
    pub project_id: String,
    pub notes: String,
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

impl Default for ExamplesSursface {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            main_navbar: Raw::to_raw(Navbar::default()),
            header: Raw::to_raw(Header::default()),
            examples_navbar: Raw::to_raw(Navbar::default()),
            sursface_examples: vec![],
            examples_css_path: sp.examples_css_path.clone(),
            main_css_path: sp.main_css_path.clone(),
            show_canvas_js_path: sp.show_canvas_js_path.clone(),
        }
    }
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

impl Default for ExamplesSayle {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            main_navbar: Raw::to_raw(Navbar::default()),
            header: Raw::to_raw(Header::default()),
            examples_navbar: Raw::to_raw(Navbar::default()),
            examples_css_path: sp.examples_css_path.clone(),
            main_css_path: sp.main_css_path.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "blog/blog.html")]
pub struct Blog {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub blog_posts: Vec<BlogPost>,
    pub main_css_path: String,
}

impl Default for Blog {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            main_navbar: Raw::to_raw(Navbar::default()),
            header: Raw::to_raw(Header::default()),
            main_css_path: sp.main_css_path.clone(),
            blog_posts: vec![],
        }
    }
}


pub struct BlogPost {
    pub title: String,
    pub slug: String,
    pub content: RawAny
}


#[derive(Template)]
#[template(path = "blog/blog_base.html")]
pub struct BlogPage {
    pub main_navbar: Raw<Navbar>,
    pub header: Raw<Header>,
    pub main_css_path: String,
    pub blog_post: BlogPost
}


#[derive(Template)]
#[template(path = "blog/crust.html")]
pub struct BlogCrust {}