use crate::{templates::{ExamplesSayle, ExamplesSursface, Header, Main, Navbar, ProjectsSursface, ProjectsWebsite, Raw}, Asset};

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
            }),
            Raw::to_raw(WasmExample {
                example_id: "hello_triangle".to_string(),
                project_id: "sursface_examples".to_string(),
                title: "Hello Triangle".to_string(),
            }),
            Raw::to_raw(WasmExample {
                example_id: "spinning_cube".to_string(),
                project_id: "sursface_examples".to_string(),
                title: "Spinning Cube".to_string(),
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