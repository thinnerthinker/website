use crate::Asset;

pub struct StaticPaths {
    pub bg_image: String,
    pub logo_path: String,     
    pub header_css_path: String,
    pub backdrop_css_path: String,
    pub main_css_path: String,
    pub navbar_css_path: String,
    pub projects_css_path: String,
    pub examples_css_path: String,
    pub show_canvas_js_path: String
}

impl StaticPaths {
    pub fn new() -> Self {
        Self {
            bg_image: Asset::get_hashed_url("images/bg.jpg"),
            logo_path: Asset::get_hashed_url("images/flake_white.png"),
            header_css_path: Asset::get_hashed_url("styles/header.css"),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css"),
            main_css_path: Asset::get_hashed_url("styles/main.css"),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css"),
            projects_css_path: Asset::get_hashed_url("styles/projects.css"),
            examples_css_path: Asset::get_hashed_url("styles/examples.css"),
            show_canvas_js_path: Asset::get_hashed_url("js/show_canvas.js"),
        }
    }
}
