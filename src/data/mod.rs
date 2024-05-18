use crate::templates::{ExamplesSayle, ExamplesSursface, Header, Main, Navbar, ProjectsSursface, ProjectsWebsite, Raw};
mod navbar;

impl Header {
    pub fn create_main() -> Self {
        Header { bg_image: "/assets/images/bg.jpg".to_string() }
    }
}

impl Main {
    pub fn create() -> Self {
        Main { 
            main_navbar: Raw::to_raw(Navbar::create_main("/")),
            header: Raw::to_raw(Header::create_main()) 
        }
    }
}

impl ProjectsSursface {
    pub fn create() -> Self {
        ProjectsSursface { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
            projects_navbar: Raw::to_raw(Navbar::create_projects_sursface("/projects/sursface")),
        }
    }
}

impl ProjectsWebsite {
    pub fn create() -> Self {
        ProjectsWebsite { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
            projects_navbar: Raw::to_raw(Navbar::create_projects_sursface("/projects/website")),
        }
    }
}

impl ExamplesSursface {
    pub fn create() -> Self {
        ExamplesSursface { 
            main_navbar: Raw::to_raw(Navbar::create_main("/examples")),
            header: Raw::to_raw(Header::create_main()),
            examples_navbar: Raw::to_raw(Navbar::create_examples_sursface("/examples/sursface")),
        }
    }
}

impl ExamplesSayle {
    pub fn create() -> Self {
        ExamplesSayle { 
            main_navbar: Raw::to_raw(Navbar::create_main("/examples")),
            header: Raw::to_raw(Header::create_main()),
            examples_navbar: Raw::to_raw(Navbar::create_examples_sayle("/examples/sayle")),
        }
    }
}