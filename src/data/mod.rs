use crate::{templates::{Blog, BlogCrust, BlogPage, BlogPost, ExamplesSayle, ExamplesSursface, Header, Main, Navbar, ProjectsSursface, ProjectsWebsite, Raw, RawAny}, Asset};

use self::static_paths::StaticPaths;

mod navbar;
pub mod static_paths;

impl Header {
    pub fn create_main() -> Self {
        Header::default()
    }
}

impl Main {
    pub fn create() -> Self {
        Main { 
            main_navbar: Raw::to_raw(Navbar::create_main("/")),
            header: Raw::to_raw(Header::create_main()),
            ..Default::default()
        }
    }
}

impl ProjectsSursface {
    pub fn create() -> Self {
        ProjectsSursface { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
            projects_navbar: Raw::to_raw(Navbar::create_projects_sursface("/projects/sursface")),
            ..Default::default()
        }
    }
}

impl ProjectsWebsite {
    pub fn create() -> Self {
        ProjectsWebsite { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
            projects_navbar: Raw::to_raw(Navbar::create_projects_website("/projects/website")),
            ..Default::default()
        }
    }
}

impl ExamplesSursface {
    pub fn create() -> Self {
        use crate::templates::WasmExample;

        let wasm_examples = vec![
            Raw::to_raw(WasmExample {
                example_id: "hello_window".to_string(),
                project_id: "sursface_examples".to_string(),
                title: "Hello Window".to_string(),
                notes: "".to_string(),
                url: "/assets/sursface_examples/hello_window.js".to_string()
            }),
            Raw::to_raw(WasmExample {
                example_id: "hello_triangle".to_string(),
                project_id: "sursface_examples".to_string(),
                title: "Hello Triangle".to_string(),
                notes: "".to_string(),
                url: "/assets/sursface_examples/hello_triangle.js".to_string()
            }),
            Raw::to_raw(WasmExample {
                example_id: "cube_camera".to_string(),
                project_id: "sursface_examples".to_string(),
                title: "Cube Camera".to_string(),
                notes: "Drag your cursor to pan around the die!".to_string(),
                url: "/assets/sursface_examples/cube_camera.js".to_string()
            })];

        ExamplesSursface { 
            main_navbar: Raw::to_raw(Navbar::create_main("/examples")),
            header: Raw::to_raw(Header::create_main()),
            examples_navbar: Raw::to_raw(Navbar::create_examples_sursface("/examples/sursface")),
            sursface_examples: wasm_examples,
            ..Default::default()
        }
    }
}

impl ExamplesSayle {
    pub fn create() -> Self {
        ExamplesSayle { 
            main_navbar: Raw::to_raw(Navbar::create_main("/examples")),
            header: Raw::to_raw(Header::create_main()),
            examples_navbar: Raw::to_raw(Navbar::create_examples_sayle("/examples/sayle")),
            ..Default::default()
        }
    }
}

impl BlogPost {
    pub fn create_crust() -> BlogPost {
        BlogPost { 
            title: "Crust - Curiously Frustrated".to_string(), 
            slug: "crust".to_string(),
            content: RawAny { html: BlogCrust {}.to_string() },
            summary: "Presenting a new software development workflow for people (such as me) who like deep work.".to_string(),
        }
    }
}


impl Blog {
    pub fn create() -> Self {
        let posts = vec![
            BlogPost::create_crust()
        ];

        let sp = StaticPaths::new();

        Self {
            main_navbar: Raw::to_raw(Navbar::create_main("/blog")),
            header: Raw::to_raw(Header::create_main()),
            main_css_path: sp.main_css_path.clone(),
            blog_posts: posts,
        }
    }
}

impl BlogPage {
    pub fn create(blog_post: BlogPost) -> Self {
        let sp = StaticPaths::new();
        Self { 
            main_navbar: Raw::to_raw(Navbar::create_main("/blog")),
            header: Raw::to_raw(Header::create_main()),
            main_css_path: sp.main_css_path.clone(),
            blog_post
        }
    }
}
