use crate::templates::{Header, Main, Projects, ExamplesSursface, Raw, Navbar};
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

impl Projects {
    pub fn create() -> Self {
        Projects { 
            main_navbar: Raw::to_raw(Navbar::create_main("/projects")),
            header: Raw::to_raw(Header::create_main()),
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