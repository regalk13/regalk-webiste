use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/regalk.css"/>

        // sets the document title
        <Title text="Welcome to Regalk's website"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div class="navbar--container">
            <ul class="navbar--items">
                <li class="navbar--item"><a href="/">"Home"</a></li>
                <li class="navbar--item"><a href="/blog">"Blog"</a></li>
                <li class="navbar--item"><a href="/library">"Library"</a></li>
                <li class="navbar--item"><a href="/contact">"Contact"</a></li>
            </ul>
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

    view! {
        <ul>{contents}</ul>
    }
}

#[component]
fn Interests() -> impl IntoView {
    let contents: Vec<_> = [
        "🕊️ Loves FreeSoftware phylosophy.",
        "📚 Enjoys reading books and diving into new ideas.",
        "💻 Loves building software and exploring creative projects in free time.",
        "🚶 Appreciates minimalism.",
        "🔤 Enjoys learning languages currently : 🇬🇧, 🇪🇸, 🇫🇷 (soon 🇩🇪, 🇵🇹).",
    ]
    .into_iter()
    .map(|content| view! { <li class="list-disc">{content}</li> })
    .collect();

    view! {
        <ul class="ml-4">{contents}</ul>
    }
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

    view! {
        <ul class="ml-4">{contents}</ul>
    }
}

#[component]
fn BlogPosts() -> impl IntoView {
    view! {
        <div class="blogs-container">
            <div class="blog--post"><p>"SOON"</p></div>
            <div class="blog--post"><p>"SOON"</p></div>
            <div class="blog--post"><p>"SOON"</p></div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <NavBar />
        <main>
            <div class="main-information-container">
                <div class="main--left-info">
                <video autoplay loop muted playsinline>
                    <source src="trimmed.mp4" type="video/mp4" />
                </video>
                </div>
                <div class="main--right-info">
                    <h1>"Welcome to my "<span class="mark-text">"site!"</span></h1>
                    <p class="main--info-text">"Hi, I'm (Regalk)! A computer scientist who loves exploring hardware, software, and everything in between. From AI hardware to kernel development and brain interfaces, I love building and learning. Oh, and I once competed internationally in web development!"</p>
                    <figure class="main--image-container">
                        <img class="main--image" src="regalk-main.jpg" />
                        <figcaption class="img--quote">(Prompt to stable Diffusion 3: Cubism art image <square 1:1>)</figcaption>
                    </figure>
                </div>
            </div>
            <div id="about-me" class="about-section">
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
                        <img class="main--image--quote" src="feyman.jpg" />
                        <figcaption class="img--quote--q">"
                        \"Fall in love with some activity, and do it! Nobody 
                        ever figures out what life is all about, and it doesn't matter. 
                        Explore the world. Nearly everything is really interesting if you go into it deeply enough. Work as hard and as much as you want to on the things you like to do the best. Don't think about what you want to be, but what you want to do. 
                        Keep up some kind of a minimum with other things so that society doesn't stop you from doing anything at all.\""
                        <p>"Richard Feynman"</p>
                        </figcaption>

                    </figure>
                    </div>
                </div>
            </div>


            <div id="projects" class="projects-section">
                <div class="project--content">
                    <h2>"Projects"</h2>
                    <p>
                    "I have been actively working on multiple opensource projects. I love to build on my work and my free time!"</p>
                    <br />
                    <ul class="project--list">
                        <li>SpaceWars - Classic SpaceWar game on Bevy</li>
                        <li>Color Mixer - Mixing colors on rust with results like real life</li>
                        <li>UNO rs - online uno game on rust!</li>
                        <li>"UNO py - online uno game on python(django)!"</li>
                        <li>"Tools for valence.rs framework for Minecraft server"</li>
                        <li>"..."</li>
                        <li>Advent of code journey</li>
                    </ul>
                    <br />

                    <p>"You can go and read more about my projects on my " <a href=" https://github.com/regalk13">"github account"</a></p>
                </div>
            </div>

            <div id="blog" class="blog-section">
                <div class="blog--content">
                <figure class="main--image--quote-container blog--quote">
                    <img class="main--image--quote" src="dennis.jpg" />
                    <figcaption class="img--quote--q">"
                    \"The only way to learn a new programming language is by writing programs in it.\""
                    <p>"Dennis Ritchie"</p>
                    </figcaption>
                </figure>


                <h2>"Blog"</h2>
                <p>
                "A blog were I develop some vague ideas I usually think off. Add it to your RSS feed and feel free to reach out—let's discuss fascinating topics together!"
                </p>
                <BlogPosts />
                </div>
            </div>
            <div id="secret" class="secret-section">
                <div class="secret--content">
                    <h2 class="glitch" data-text="3dnotionestuntcreate">"notiones sunt creata"</h2>
                    <div class="viewer" hint="its a viewr">
                    </div>
                </div>
            </div>
        </main>
        <footer>
        <div class="footer-container">
            <div class="footer-main">
                <div class="footer-section">
                    <h4>"Contact"</h4>
                    <ul>
                        <li><a href="mailto:contact@regalk.dev">"Email"</a></li>
                        <li><a href="https://github.com/regalk13">"GitHub"</a></li>
                    </ul>
                </div>
                <div class="footer-section">
                    <h4>"Quick Links"</h4>
                    <ul>
                        <li><a href="#about-me">"About"</a></li>
                        <li><a href="#projects">"Projects"</a></li>
                        <li><a href="#blog">"Blog"</a></li>
                    </ul>
                </div>
                <div class="footer-section">
                    <h4>"RSS Feed"</h4>
                    <p>"Subscribe to my "<a href="/rss.xml">"RSS feed"</a></p>
                </div>
            </div>
            <div class="footer-bottom">
                <p>"© 2025 Regalk - Built with Rust & ❤️"</p>
                <p>"This site is open source - "<a href="https://github.com/regalk13/website">"view source"</a></p>
            </div>
        </div>
    </footer>
    }
}
