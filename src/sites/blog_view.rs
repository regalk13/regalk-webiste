use leptos::prelude::*;
use leptos_router::hooks::use_location;
use thiserror::Error;

fn parse_md_content(content: &str) -> impl IntoView {
    use regex::Regex;   
    use lazy_static::lazy_static;
    lazy_static! {
        static ref IMAGE_REGEX: Regex = Regex::new(r"^[^\s]+\.(png|jpg|jpeg|webp)$").unwrap();
    }
     let lines: Vec<String> = content.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

        lines.into_iter()
        .map(|line| {
            match line.as_str() { // Match on &str instead of String
                l if l.starts_with("### ") => view! { <h3>{l.strip_prefix("### ").unwrap_or(l).to_string()}</h3> }.into_any(),
                l if l.starts_with("## ") => view! { <h2>{l.strip_prefix("## ").unwrap_or(l).to_string()}</h2> }.into_any(),
                l if l.starts_with("# ") => view! { <h1>{l.strip_prefix("# ").unwrap_or(l).to_string()}</h1> }.into_any(),
                l if l.starts_with("!image[") && l.ends_with(']') => {
                    let src = format!("{}", &l[7..l.len()-1]);
                    view! { <img src=src class="content-image" /> }.into_any()
                },
                l if IMAGE_REGEX.is_match(l) => {
                    let src = format!("/{}", l.to_string()); // Convert to owned String
                    view! { <img src=src class="content-image" /> }.into_any()
                },
                l if l.starts_with('>') => view! {
                    <div class="quote-container">
                        <div class="quote-left-divider"></div>
                        <p class="quote">{l.strip_prefix('>').unwrap_or(l).trim().to_string()}</p>
                    </div>
                }.into_any(),
                l => view! { <p>{l.to_string()}</p> }.into_any()
            }
        })
        .collect::<Vec<_>>()
}


#[server]
pub async fn get_blog_post(filename: String) -> Result<BlogPost, ServerFnError> {
    use tokio::fs;
    use std::collections::HashMap;
    use regex::Regex;   
    
    let valid_format = Regex::new(r"^[a-zA-Z0-9-]+-\d{4}-\d{2}-\d{2}\.md$")
        .map_err(|_| BlogError::Io("Invalid regex pattern".to_string()))?;
    
    if !valid_format.is_match(&filename) {
        return Err(BlogError::Io(
            "Invalid filename format. Expected: {name}-{year}-{month}-{day}.md".to_string()
        ).into());
    }

    let path = format!("./blogs/{}", filename);
    let content = fs::read_to_string(&path)
        .await
        .map_err(|e| BlogError::Io(format!("Failed to read file {}: {}", filename, e)))?;

    let mut metadata = HashMap::new();
    let mut lines = content.lines();
    let mut body = String::new();
    
    while let Some(line) = lines.next() {
        if line.starts_with("--") {
            let line = line.trim_start_matches("--").trim();
            if let Some((key, value)) = line.split_once(':') {
                metadata.insert(key.trim().to_string(), value.trim().to_string());
            }
        } else {
            body = lines.collect::<Vec<_>>().join("\n");
            break;
        }
    }

    Ok(BlogPost {
        title: metadata.get("title").cloned().unwrap_or_default(),
        date: metadata.get("date").cloned().unwrap_or_default(),
        content: body,
        filename,
    })
}

#[component]
pub fn BlogView() -> impl IntoView {
    let location = use_location();
    let filename = move || {
        location.pathname.get()
            .split('/')
            .last()
            .unwrap_or_default()
            .to_string()
    };

    let blog_post = Resource::new(
        filename,
        |f| async move { get_blog_post(f).await }
    );

    view! {
        <Suspense fallback=move || {
            view! { <div>"Loading post..."</div> }
        }>
            {move || match blog_post.get() {
                None => {
                    view! {
                        <header class="title-main--page--container">
                            <h1>{"NO BLOG POST FOUND".to_string()}</h1>
                        </header>
                        <div class="blog--content-view">
                            <h2>{"The singularity is nearer".to_string()}</h2>
                            <span>{"2099-99-99".to_string()}</span>
                            <div></div>
                        </div>
                    }
                        .into_any()
                }
                Some(Ok(post)) => {
                    view! {
                        <header class="title-main--page--container">
                            <h1>{post.title.to_string()}</h1>
                        </header>
                        <div class="blog--content-view">
                            <br />
                            <br />
                            <span>{post.date.to_string()}</span>
                            <div class="md-content">
                                <div>{parse_md_content(&post.content.to_string())}</div>
                            </div>
                        </div>
                    }
                        .into_any()
                }
                Some(Err(_e)) => {
                    view! {
                        <header class="title-main--page--container">
                            <h1>{"NO BLOG POST FOUND".to_string()}</h1>
                        </header>
                        <div class="blog--content-view">
                            <h2>{"The singularity is nearer".to_string()}</h2>
                            <span>{"2099-99-99".to_string()}</span>
                            <div inner_html="<p></p>".to_string()></div>
                        </div>
                    }
                        .into_any()
                }
            }}
        </Suspense>
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
                        <h3>"RSS Feed"</h3>
                        <p>
                            "Subscribe to my "<a target="_blank" href="/rss.xml">
                                "RSS feed"
                            </a>
                        </p>
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct BlogPost {
    title: String,
    date: String,
    content: String,
    filename: String,
}

#[derive(Error, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BlogError {
    #[error("IO error: {0}")]
    Io(String),
}

