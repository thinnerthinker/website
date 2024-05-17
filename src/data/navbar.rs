use crate::templates::{Navbar,NavLink};

impl Navbar {
    pub fn create_main(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Home",
                url: "/",
                route: "/",
            },
            NavLink {
                name: "Projects",
                url: "/projects",
                route: "/projects",
            },
            NavLink {
                name: "Examples",
                url: "/examples/sursface",
                route: "/examples",
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: "/assets/images/flower.jpg".to_string(),
            route_url: route_url.to_string(),
        }
    }

    pub fn create_examples_sursface(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/examples/sursface",
                route: "/examples/sursface"
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: "/assets/images/colorado.jpg".to_string(),
            route_url: route_url.to_string(),
        }
    }
}