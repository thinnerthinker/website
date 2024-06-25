use crate::{templates::Navbar, Asset};
use crate::templates::NavLink;

impl Navbar {
    pub fn create_main(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Projects",
                url: "/projects",
                route: "/projects",
                icon: Asset::get_hashed_url("images/projects.png"),
                disabled: false,
            },
            NavLink {
                name: "Examples",
                url: "/examples/sursface",
                route: "/examples",
                icon: Asset::get_hashed_url("images/examples.png"),
                disabled: false,
            },
            NavLink {
                name: "Blog",
                url: "/blog",
                route: "/blog",
                icon: Asset::get_hashed_url("images/blog.png"),
                disabled: false,
            },
            NavLink {
                name: "Docs",
                url: "/docs",
                route: "/docs",
                icon: Asset::get_hashed_url("images/docs.png"),
                disabled: true,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/flower.jpg"),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css"),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css"),
        }
    }

    pub fn create_projects_sursface(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/projects/sursface",
                route: "/projects/sursface",
                icon: Asset::get_hashed_url("images/sursface.png"),
                disabled: false,
            },
            NavLink {
                name: "Website",
                url: "/projects/website",
                route: "/projects/website",
                icon: Asset::get_hashed_url("images/website.png"),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/puddle.jpg"),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css"),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css"),
        }
    }

    pub fn create_projects_website(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/projects/sursface",
                route: "/projects/sursface",
                icon: Asset::get_hashed_url("images/sursface.png"),
                disabled: false,
            },
            NavLink {
                name: "Website",
                url: "/projects/website",
                route: "/projects/website",
                icon: Asset::get_hashed_url("images/website.png"),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/puddle.jpg"),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css"),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css"),
        }
    }

    pub fn create_examples_sursface(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/examples/sursface",
                route: "/examples/sursface",
                icon: Asset::get_hashed_url("images/sursface.png"),
                disabled: false,
            },
            NavLink {
                name: "Sayle",
                url: "/examples/sayle",
                route: "/examples/sayle",
                icon: Asset::get_hashed_url("images/sayle.png"),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/colorado.jpg"),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css"),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css"),
        }
    }

    pub fn create_examples_sayle(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/examples/sursface",
                route: "/examples/sursface",
                icon: Asset::get_hashed_url("images/sursface.png"),
                disabled: false,
            },
            NavLink {
                name: "Sayle",
                url: "/examples/sayle",
                route: "/examples/sayle",
                icon: Asset::get_hashed_url("images/sayle.png"),
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: Asset::get_hashed_url("images/colorado.jpg"),
            route_url: route_url.to_string(),
            backdrop_css_path: Asset::get_hashed_url("styles/backdrop.css"),
            navbar_css_path: Asset::get_hashed_url("styles/navbar.css"),
        }
    }
}
