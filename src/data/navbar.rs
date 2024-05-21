use crate::templates::{Navbar, NavLink};
use crate::Asset;

impl Navbar {
    pub fn create_main(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Projects",
                url: "/projects",
                route: "/projects",
                icon: Asset::get_hashed_url("images/projects.png").unwrap_or_default(),
                disabled: false,
            },
            NavLink {
                name: "Examples",
                url: "/examples/sursface",
                route: "/examples",
                icon: Asset::get_hashed_url("images/examples.png").unwrap_or_default(),
                disabled: false,
            },
            NavLink {
                name: "Blog",
                url: "/blog",
                route: "/blog",
                icon: Asset::get_hashed_url("images/blog.png").unwrap_or_default(),
                disabled: true,
            },
            NavLink {
                name: "Docs",
                url: "/docs",
                route: "/docs",
                icon: Asset::get_hashed_url("images/docs.png").unwrap_or_default(),
                disabled: true,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/flower.jpg").unwrap_or_default(),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css").unwrap_or_default(),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css").unwrap_or_default(),
        }
    }

    pub fn create_projects_sursface(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/projects/sursface",
                route: "/projects/sursface",
                icon: Asset::get_hashed_url("images/sursface.png").unwrap_or_default(),
                disabled: false,
            },
            NavLink {
                name: "Website",
                url: "/projects/website",
                route: "/projects/website",
                icon: Asset::get_hashed_url("images/website.png").unwrap_or_default(),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/puddle.jpg").unwrap_or_default(),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css").unwrap_or_default(),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css").unwrap_or_default(),
        }
    }

    pub fn create_projects_website(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/projects/sursface",
                route: "/projects/sursface",
                icon: Asset::get_hashed_url("images/sursface.png").unwrap_or_default(),
                disabled: false,
            },
            NavLink {
                name: "Website",
                url: "/projects/website",
                route: "/projects/website",
                icon: Asset::get_hashed_url("images/website.png").unwrap_or_default(),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/puddle.jpg").unwrap_or_default(),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css").unwrap_or_default(),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css").unwrap_or_default(),
        }
    }

    pub fn create_examples_sursface(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/examples/sursface",
                route: "/examples/sursface",
                icon: Asset::get_hashed_url("images/sursface.png").unwrap_or_default(),
                disabled: false,
            },
            NavLink {
                name: "Sayle",
                url: "/examples/sayle",
                route: "/examples/sayle",
                icon: Asset::get_hashed_url("images/sayle.png").unwrap_or_default(),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/colorado.jpg").unwrap_or_default(),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css").unwrap_or_default(),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css").unwrap_or_default(),
        }
    }

    pub fn create_examples_sayle(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/examples/sursface",
                route: "/examples/sursface",
                icon: Asset::get_hashed_url("images/sursface.png").unwrap_or_default(),
                disabled: false,
            },
            NavLink {
                name: "Sayle",
                url: "/examples/sayle",
                route: "/examples/sayle",
                icon: Asset::get_hashed_url("images/sayle.png").unwrap_or_default(),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/colorado.jpg").unwrap_or_default(),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css").unwrap_or_default(),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css").unwrap_or_default(),
        }
    }
}
