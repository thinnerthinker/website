use crate::{templates::{ExamplesSayle, ExamplesSursface, Header, Main, Navbar, ProjectsSursface, ProjectsWebsite, Raw}, Asset};
mod navbar;

impl Header {
    pub fn create_main() -> Self {
        Header { 
            bg_image: Asset::get_hashed_url("images/bg.jpg").unwrap_or_default(),
            header_css_path: Asset::get_hashed_url("styles/header.css").unwrap_or_default(),
            logo_path: Asset::get_hashed_url("images/flake_white.png").unwrap_or_default(),
            backdrop_css_path: Asset::get_hashed_url("images/bg.jpg").unwrap_or_default(),
        }
    }
}

impl Main {
    pub fn create() -> Self {
        Main { 
            main_navbar: Raw::to_raw(Navbar::create_main("/")),
            header: Raw::to_raw(Header::create_main()),
            main_css_path: Asset::get_hashed_url("styles/main.css").unwrap_or_default(),
        }
    }
}

impl ProjectsSursface {
    pub fn create() -> Self {
        ProjectsSursface { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
            projects_navbar: Raw::to_raw(Navbar::create_projects_sursface("/projects/sursface")),
            main_css_path: Asset::get_hashed_url("styles/main.css").unwrap_or_default(),
            projects_css_path: Asset::get_hashed_url("styles/projects.css").unwrap_or_default(),
        }
    }
}

impl ProjectsWebsite {
    pub fn create() -> Self {
        ProjectsWebsite { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
            projects_navbar: Raw::to_raw(Navbar::create_projects_sursface("/projects/website")),
            main_css_path: Asset::get_hashed_url("styles/main.css").unwrap_or_default(),
            projects_css_path: Asset::get_hashed_url("styles/projects.css").unwrap_or_default(),
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
            }),
            Raw::to_raw(WasmExample {
                example_id: "hello_triangle".to_string(),
                project_id: "sursface_examples".to_string(),
                title: "Hello Triangle".to_string(),
            })];

        ExamplesSursface { 
            main_navbar: Raw::to_raw(Navbar::create_main("/examples")),
            header: Raw::to_raw(Header::create_main()),
            examples_navbar: Raw::to_raw(Navbar::create_examples_sursface("/examples/sursface")),
            sursface_examples: wasm_examples,
            main_css_path: Asset::get_hashed_url("styles/main.css").unwrap_or_default(),
            examples_css_path: Asset::get_hashed_url("styles/examples.css").unwrap_or_default(),
            show_canvas_js_path: Asset::get_hashed_url("js/show_canvas.js").unwrap_or_default(),
        }
    }
}

impl ExamplesSayle {
    pub fn create() -> Self {
        ExamplesSayle { 
            main_navbar: Raw::to_raw(Navbar::create_main("/examples")),
            header: Raw::to_raw(Header::create_main()),
            examples_navbar: Raw::to_raw(Navbar::create_examples_sayle("/examples/sayle")),
            main_css_path: Asset::get_hashed_url("styles/main.css").unwrap_or_default(),
            examples_css_path: Asset::get_hashed_url("styles/examples.css").unwrap_or_default(),
        }
    }
}