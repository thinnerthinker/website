use crate::templates::{Navbar,NavLink};

impl Navbar {
    pub fn create_main(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Projects",
                url: "/projects",
                route: "/projects",
                icon: "/assets/images/projects.png",
                disabled: false,
            },
            NavLink {
                name: "Examples",
                url: "/examples/sursface",
                route: "/examples",
                icon: "/assets/images/examples.png",
                disabled: false,
            },
            NavLink {
                name: "Blog",
                url: "/blog",
                route: "/blog",
                icon: "/assets/images/blog.png",
                disabled: true,
            },
            NavLink {
                name: "Docs",
                url: "/docs",
                route: "/docs",
                icon: "/assets/images/docs.png",
                disabled: true,
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
                route: "/examples/sursface",
                icon: "/assets/images/sursface.png",
                disabled: false,
            },
            NavLink {
                name: "Sayle",
                url: "/examples/sayle",
                route: "/examples/sayle",
                icon: "/assets/images/sayle.png",
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: "/assets/images/colorado.jpg".to_string(),
            route_url: route_url.to_string(),
        }
    }

    pub fn create_examples_sayle(route_url: &'static str) -> Self {
        let nav_links = vec![
            NavLink {
                name: "Sursface",
                url: "/examples/sursface",
                route: "/examples/sursface",
                icon: "/assets/images/sursface.png",
                disabled: false,
            },
            NavLink {
                name: "Sayle",
                url: "/examples/sayle",
                route: "/examples/sayle",
                icon: "/assets/images/sayle.png",
                disabled: false,
            },
        ];
        Navbar {
            links: nav_links,
            bg_image: "/assets/images/colorado.jpg".to_string(),
            route_url: route_url.to_string(),
        }
    }
}