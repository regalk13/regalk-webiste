use leptos::prelude::*;
use leptos_router::hooks::use_location;
use thiserror::Error;
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

fn parse_md_content(content: &str) -> impl IntoView {
    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        static ref IMAGE_REGEX: Regex = Regex::new(r"^[^\s]+\.(png|jpg|jpeg|webp)$").unwrap();
    }
    let lines: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    lines
        .into_iter()
        .map(|line| {
            match line.as_str() {
                // Match on &str instead of String
                l if l.starts_with("### ") => {
                    view! { <h3>{l.strip_prefix("### ").unwrap_or(l).to_string()}</h3> }.into_any()
                }
                l if l.starts_with("## ") => {
                    view! { <h2>{l.strip_prefix("## ").unwrap_or(l).to_string()}</h2> }.into_any()
                }
                l if l.starts_with("# ") => {
                    view! { <h1>{l.strip_prefix("# ").unwrap_or(l).to_string()}</h1> }.into_any()
                }
                l if l.starts_with("!image[") && l.ends_with(']') => {
                    let src = format!("{}", &l[7..l.len() - 1]);
                    view! { <img src=src class="content-image" /> }.into_any()
                }
                l if IMAGE_REGEX.is_match(l) => {
                    let src = format!("/{}", l.to_string()); // Convert to owned String
                    view! { <img src=src class="content-image" /> }.into_any()
                }
                l if l.starts_with('>') => view! {
                    <div class="quote-container">
                        <div class="quote-left-divider"></div>
                        <p class="quote">{l.strip_prefix('>').unwrap_or(l).trim().to_string()}</p>
                    </div>
                }
                .into_any(),
                l => view! { <p>{l.to_string()}</p> }.into_any(),
            }
        })
        .collect::<Vec<_>>()
}

fn get_blog_post(filename: String) -> Result<BlogPost, ServerFnError> {
    use regex::Regex;

    let valid_format = Regex::new(r"^[a-zA-Z0-9-]+-\d{4}-\d{2}-\d{2}\.md$")
        .map_err(|_| BlogError::Io("Invalid regex pattern".to_string()))?;

    if !valid_format.is_match(&filename) {
        return Err(BlogError::Io(
            "Invalid filename format. Expected: {name}-{year}-{month}-{day}.md".to_string(),
        )
        .into());
    }

    let blog_post = BLOG_POSTS.get(&filename.as_str()).unwrap();

    Ok(BlogPost {
        title: blog_post.title.to_string(),
        date: blog_post.date.to_string(),
        content: blog_post.content.to_string(),
        filename: blog_post.filename.to_string(),
    })
}

#[component]
pub fn BlogView() -> impl IntoView {
    let location = use_location();
    let filename = location
        .pathname
        .get()
        .split('/')
        .last()
        .unwrap_or_default()
        .to_string();

    let blog_post = get_blog_post(filename);

    view! {
        <div>
        {match blog_post {
            Ok(post) => {
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
            Err(_e) => {
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
        </div>
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
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
    #[error("Blog post not found: {0}")]
    NotFound(String),
}
