use leptos::html::{Div, Span};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes },
    StaticSegment,
    path
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::sites::{library::Library, blog_view::BlogView, contact::Contact, blog::Blog};

#[wasm_bindgen(module = "/src/js/animations.js")]
extern "C" {
    #[wasm_bindgen(js_name = "initTypewriter")]
    pub fn init_typewriter(element: Element, words: Box<[JsValue]>);
    #[wasm_bindgen(js_name = "initScrollAnimations")]
    pub fn init_scroll_animations();
    #[wasm_bindgen(js_name = "initFeather")]
    pub fn init_feather();
}

#[component]
pub fn TypewriterComponent() -> impl IntoView {
    let el = NodeRef::<Span>::new();

    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))] {
            if let Some(element) = el.get() {
                let words = vec![
                    "place!".into(),
                    "exp!".into(),
                    "work!".into(),
                    "ideas!".into(),
                    "site!".into(),
                ];

                init_typewriter(element.into(), words.into_boxed_slice());
            }
        }
    });

    view! { <span node_ref=el class="typewriter"></span> }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let page_description = "Computer scientist specializing in systems programming, AI, and low-level development. Explore my projects in Rust, kernel development, and innovative web solutions.";
    let keywords = "Rust programming, systems programming, kernel development, AI research, web development, Three.js, Leptos, Bevy engine, open source projects";
    let author = "Regalk";
    view! {
        <!DOCTYPE html> 
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="description" content=page_description />
                <meta name="keywords" content=keywords />
                <meta name="author" content=author />
                <meta name="profile:username" content="regalk13" />
                <meta name="profile:languages" content="Rust, C, C++, Python, JavaScript" />
                <meta
                    name="profile:technologies"
                    content="Leptos, Axum, Bevy, Three.js, Linux Kernel"
                />
                <meta
                    name="profile:interests"
                    content="AI Hardware, Kernel Development, WebAssembly, Distributed Systems"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/regalk.css" />

        // sets the document title
        <Title text="Regalk | Computer Scientist & Full-Stack Developer" />
        <InitFeather />

        // content for this welcome page
        <Router>
            <NavBar />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/library") view=Library />
                    <Route path=StaticSegment("/contact") view=Contact />
                    <Route path=StaticSegment("/blog") view=Blog />
                    <Route path=path!("/blog/:id") view=BlogView />
                </Routes>
            </main>

        </Router>
    }
}

#[component]
pub fn ScrollAnimations() -> impl IntoView {
    let el = NodeRef::<Div>::new();

    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(_) = el.get() {                
                init_scroll_animations();
            }
        }
    });

    view! { <div node_ref=el></div> }
}


#[component]
pub fn InitFeather() -> impl IntoView {
    let el = NodeRef::<Div>::new();

    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(_) = el.get() {   
                init_feather();             
            }
        }
    });

    view! { <div node_ref=el></div> }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar--container">
            <ul class="navbar--items">
                <li class="navbar--item">
                    <a href="/">"Home"</a>
                </li>
                <li class="navbar--item">
                    <a href="/blog">"Blog"</a>
                </li>
                <li class="navbar--item">
                    <a href="/library">"Library"</a>
                </li>
                <li class="navbar--item">
                    <a href="/contact">"Contact"</a>
                </li>
            </ul>
            <div class="mobile--items">
                <svg
                    id="mobile-menu-button"
                    fill="#000000"
                    width="18px"
                    height="18px"
                    viewBox="0 0 32 32"
                    version="1.1"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <title>bars</title>
                    <path d="M2 8.749h28c0.414 0 0.75-0.336 0.75-0.75s-0.336-0.75-0.75-0.75v0h-28c-0.414 0-0.75 0.336-0.75 0.75s0.336 0.75 0.75 0.75v0zM30 15.25h-28c-0.414 0-0.75 0.336-0.75 0.75s0.336 0.75 0.75 0.75v0h28c0.414 0 0.75-0.336 0.75-0.75s-0.336-0.75-0.75-0.75v0zM30 23.25h-28c-0.414 0-0.75 0.336-0.75 0.75s0.336 0.75 0.75 0.75v0h28c0.414 0 0.75-0.336 0.75-0.75s-0.336-0.75-0.75-0.75v0z"></path>
                </svg>
            </div>
            <div class="mobile--dropdown" id="mobile-menu">
                <a href="/">"Home"</a>
                <a href="/blog">"Blog"</a>
                <a href="/library">"Library"</a>
                <a href="/contact">"Contact"</a>
            </div>
        </div>
    }
}

#[component]
fn AboutMe() -> impl IntoView {
    let contents: Vec<_> = [
        "🎂 Birthday: March 8.",
        "🧮 Passionate about mathematics, physics, and programming.",
        "🤓 Lifelong learner, driven by curiosity and the pursuit of mastery.",
        "🤖 Enthusiastic about low-level programming, including hardware, OS, kernel development, and drivers.",
        "🤖 Excited about AI, exploring new ideas for intelligent agents, and advancing knowledge in deep learning.",
        "🌐 Full-stack web developer with experience in modern frameworks, always seeking efficient and scalable solutions.",
        "🔧 Currently focusing on Rust and exploring alternatives like Leptos, Axum, and other frameworks to build efficient systems.",
        "💻 Enjoys experimenting with cluster building, IoT, RISC-V, and ARM for fun and learning.",
    ]
    .into_iter()
    .map(|content| view! { <li class="list-disc">{content}</li> })
    .collect();

    view! { <ul>{contents}</ul> }
}

#[component]
fn Interests() -> impl IntoView {
    let contents: Vec<_> = [
        "🕊️ Loves FreeSoftware phylosophy.",
        "📚 Enjoys reading books and diving into new ideas.",
        "💻 Loves building software and exploring creative projects in free time.",
        "🚶 Appreciates minimalism.",
        "🔤 Enjoys learning languages : 🇬🇧, 🇪🇸, 🇫🇷 (soon 🇩🇪, 🇵🇹).",
    ]
    .into_iter()
    .map(|content| view! { <li class="list-disc">{content}</li> })
    .collect();

    view! { <ul class="ml-4">{contents}</ul> }
}

#[component]
fn Setup() -> impl IntoView {
    let contents: Vec<_> = [
        "🐧 Distro: NixOS <unstable channel>.",
        "✍️ Editor: Vim or Emacs (motions).",
        "🖥️ WM: Hyprland, RiverWM (Now a full-time Wayland user).",
        "🖤 Terminal: Ghostty, Kitty.",
        "🎨 Theme: Tokyo Night or Rosepine.",
        "🔤 Font: Iosevka NerdFont.",
    ]
    .into_iter()
    .map(|content| view! { <li class="list-disc">{content}</li> })
    .collect();

    view! { <ul class="ml-4">{contents}</ul> }
}

#[component]
fn BlogPosts() -> impl IntoView {
    view! {
        <div class="blogs-container">
            <div class="blog--post content-blog glitch-post">
                <a href="/blog/whylovenix-2025-02-02.md" class="blog--title">Why I love Nix</a>
                <p class="blog--preview">"Nix changed the way I see containers, pkg managers and the communities. Nix is the modern way to understand systems."</p>
            </div>
            <div class="blog--post glitch-post">
                <p>"SOON"</p>
            </div>
            <div class="blog--post glitch-post">
                <p>"SOON"</p>
            </div>
        </div>
    }
}

#[component]
pub fn ScrollAppear(
    children: Children,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] id: &'static str,
) -> impl IntoView {
    let class = format!("scroll-appear {}", class);
    
    return  view! {
        <div id=id class=class>
            {children()}
        </div>
    };
}

#[component]
fn ProjectsSection() -> impl IntoView {
    #[derive(Clone)]
    struct Project {
        name: &'static str,
        description: &'static str,
        technologies: Vec<&'static str>,
        repo: &'static str,
    }

    let projects = vec![
        Project {
            name: "SpaceWars",
            description: "A modern reimagining of the classic SpaceWar game built with Bevy engine",
            technologies: vec!["Rust", "Bevy", "WebAssembly"],
            repo: "https://github.com/regalk13/spacewars",
        },
        Project {
            name: "Color Mixer",
            description: "Real-world color blending simulation with physics-based mixing algorithms",
            technologies: vec!["Rust", "WGPU", "Color Science"],
            repo: "https://github.com/regalk13/mix-colors",
        },
        Project {
            name: "Personal Website",
            description: "Portfolio and blog with custom CMS built using Rust web stack",
            technologies: vec!["Rust", "Leptos", "Axum", "SSG"],
            repo: "https://github.com/regalk13/regalk-website",
        },
        Project {
            name: "UNO.rs",
            description: "Multiplayer UNO game implementation with server-authoritative architecture",
            technologies: vec!["Rust", "WebSockets", "Leptos"],
            repo: "https://github.com/regalk13/uno-rs",
        },
        Project {
            name: "UNO.py",
            description: "Multiplayer UNO game implementation with server-authoritative architecture",
            technologies: vec!["Python", "WebSockets", "Django"],
            repo: "https://github.com/regalk13/uno-game",
        },
        Project {
            name: "Valence Tools",
            description: "Suite of developer tools for the Valence Minecraft server framework",
            technologies: vec!["Rust", "Valence", "Minecraft"],
            repo: "https://github.com/IT-MC/valence-extra",
        },
    ];

    view! {
        <div id="projects" class="projects-section scroll-appear">
            <div class="project--content">
                <div class="project-information">
                    <h2 class="section-title">"Featured Projects"</h2>
                    <p class="section-subtitle">
                        "A selection of open source initiatives I've contributed to or created. 
                        Passion drives innovation - these projects represent my journey in 
                        software craftsmanship."
                    </p>
                </div>
                <div class="project-grid">
                    {projects
                        .into_iter()
                        .map(|project| {
                            view! {
                                <div class="project-card">
                                    <div class="project-header">
                                        <div class="project--title-link-container">
                                            <a
                                                href=project.repo
                                                target="_blank"
                                                class="github-link"
                                                aria-label=format!(
                                                    "View {} project on GitHub",
                                                    project.name,
                                                )
                                            >
                                                <h3 class="project-title">{project.name}</h3>

                                            </a>
                                        </div>
                                    </div>
                                    <br />

                                    <p class="project-description">{project.description}</p>
                                    <div class="tech-stack">
                                        {project
                                            .technologies
                                            .into_iter()
                                            .map(|tech| view! { <span class="tech-tag">{tech}</span> })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>

                <div class="github-cta">
                    <p>
                        "Explore more on "
                        <a href="https://github.com/regalk13" target="_blank" class="github-button">
                            <i class="ri-github-fill"></i>
                            "GitHub"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main>
            <ScrollAnimations />
            <div class="main-information-container scroll-appear">
                <div class="main--right-info">
                    <div class="main--right-info-text">
                        <h1>"Welcome to my "<TypewriterComponent /></h1>
                        <p class="main--info-text">
                            "Hi, I'm (Regalk)! A computer scientist who loves exploring hardware, software, and everything in between. From AI hardware to kernel development and brain interfaces, I love building and learning. Oh, and I once competed internationally in web development!"
                        </p>
                    </div>
                    <figure class="main--image-container">
                        <img
                            alt="Main image and logo of the website regalk computer scientist"
                            class="main--image animated-image"
                            src="regalk-main.webp"
                        />
                        <figcaption class="img--quote">
                            (Prompt to stable Diffusion 3: Cubism art image <square 1:1>)
                        </figcaption>
                    </figure>
                </div>
            </div>
            <section id="about-me" class="about-section scroll-appear">
                <div class="about--content">
                    <h2>"About Me"</h2>
                    <p>
                        "I'm a tech guy with a deep curiosity for both hardware and software. Throughout my journey as a computer scientist, I've delved into mutliple areas where software is needed (I like beign a generalist!). My goal is to always learn, build, and explore innovative technologies that shape the future and improve human life. "
                    </p>
                    <br />
                    <AboutMe />
                    <br />
                    <p>
                        "I have experience in some progamming languages like C, C++, Python, Rust, Zig, JS, TS, Go, Haskell and some more..."
                    </p>
                    <br />
                    <h3>"Interests"</h3>

                    <Interests />

                    <br />

                    <h3>Setup</h3>

                    <p style="margin-bottom: 15px">Linux Setup</p>

                    <Setup />
                    <div class="quote--container">
                        <figure class="main--image--quote-container">
                            <img
                                alt="Image of Richard Feynman giving a lecture"
                                class="main--image--quote animated-image"
                                src="feynman.webp"
                            />
                            <figcaption class="img--quote--q">
                                "
                                \"Fall in love with some activity, and do it! Nobody 
                                ever figures out what life is all about, and it doesn't matter. 
                                Explore the world. Nearly everything is really interesting if you go into it deeply enough. Work as hard and as much as you want to on the things you like to do the best. Don't think about what you want to be, but what you want to do. 
                                Keep up some kind of a minimum with other things so that society doesn't stop you from doing anything at all.\""
                                <p>"Richard Feynman"</p>
                            </figcaption>

                        </figure>
                    </div>
                </div>
            </section>

            <ProjectsSection />

            <div id="blog" class="blog-section scroll-appear">
                <div class="blog--content">

                    <h2>"Blog"</h2>
                    <p>
                        "A blog were I develop some vague ideas I usually think off. Add it to your RSS feed and feel free to reach out—let's discuss fascinating topics together!"
                    </p>
                    <BlogPosts />

                    <figure class="main--image--quote-container blog--quote">
                        <img
                            alt="Image of Dennis Ritchie"
                            class="main--image--quote animated-image"
                            src="dennis.webp"
                        />
                        <figcaption class="img--quote--q">
                            "
                            \"The only way to learn a new programming language is by writing programs in it.\""
                            <p>"Dennis Ritchie"</p>
                        </figcaption>
                    </figure>
                </div>
            </div>
        </main>
        <footer>
            <div class="footer-container">
                <div class="footer-main">
                    <div class="footer-section">
                        <h3>"Contact"</h3>
                        <ul>
                            <li>
                                <a href="mailto:contact@regalk.dev">"Email"</a>
                            </li>
                            <li>
                                <a href="https://github.com/regalk13">"GitHub"</a>
                            </li>
                        </ul>
                    </div>
                    <div class="footer-section">
                        <h3>"Quick Links"</h3>
                        <ul>
                            <li>
                                <a href="#about-me">"About"</a>
                            </li>
                            <li>
                                <a href="#projects">"Projects"</a>
                            </li>
                            <li>
                                <a href="#blog">"Blog"</a>
                            </li>
                        </ul>
                    </div>
                    <div class="footer-section">
                        <h3>"RSS Feed"</h3>
                        <p>"Subscribe to my "<a href="/rss.xml">"RSS feed"</a></p>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>"© 2025 Regalk - Built with Rust & ❤️"</p>
                    <p>
                        "This site is open source - "
                        <a href="https://github.com/regalk13/regalk-website">"view source"</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}
