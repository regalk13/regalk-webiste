use leptos::prelude::*;
use thiserror::Error; // Add thiserror to dependencies

#[derive(Error, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BlogError {
    #[error("IO error: {0}")]
    Io(String),
}


#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct File {
    title: String,
    desc: String,
    date: String,
    filename: String,
    image: String
}

#[server]
pub async fn get_blog_posts() -> Result<Vec<File>, ServerFnError> {
    use tokio::fs;
    use std::collections::HashMap;
    let mut entries = fs::read_dir("./blogs")
        .await
        .map_err(|e| BlogError::Io(format!("Failed to read directory: {}", e)))?;

    let mut blog_files = Vec::new();

    while let Some(entry) = entries.next_entry()
        .await
        .map_err(|e| BlogError::Io(format!("Failed to read entry: {}", e)))?
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {

                let content = fs::read_to_string(&path)
                    .await
                    .map_err(|e| BlogError::Io(format!("Failed to read file {}: {}", name, e)))?;

                let mut metadata = HashMap::new();
                let mut lines = content.lines();
                
                while let Some(line) = lines.next() {
                    if line.starts_with("--") {
                        let line = line.trim_start_matches("--").trim();
                        if let Some((key, value)) = line.split_once(':') {
                            metadata.insert(key.trim().to_string(), value.trim().to_string());
                        }
                    } else {
                        break;
                    }
                }

                let title = metadata.get("title").cloned().unwrap_or_default();
                let desc = metadata.get("desc").cloned().unwrap_or_default();
                let image = metadata.get("image").cloned().unwrap_or_default();
                let date = metadata.get("date").cloned().unwrap_or_default();

                blog_files.push(File {
                    title,
                    desc,
                    date,
                    filename: name.to_string(),
                    image,
                });

            }
        }
    }

    Ok(blog_files)
}

#[component]
pub fn Blog() -> impl IntoView {
    let blog_posts = Resource::new(|| (), |_| async move {
        get_blog_posts().await
    });
    view! {
        <header class="title-main--page--container">
            <h1>BLOG</h1>
        </header>

        <div class="blog-posts">
            <p class="blog-posts-desc">
                "A blog were I develop some vague ideas I usually think off. Add it to your RSS feed and feel free to reach out—let's discuss fascinating topics together!"
            </p>
            <Suspense fallback=move || {
                view! { <div>"Loading posts..."</div> }
            }>
                {move || {
                    blog_posts
                        .read()
                        .clone()
                        .map(|result| match result {
                            Ok(files) => {
                                view! {
                                    <ul class="blog-list">
                                        {if !files.is_empty() {
                                            files
                                                .iter()
                                                .map(|file| {
                                                    let href: String = format!("/blog/{}", file.filename);
                                                    let text = file.title.to_string();
                                                    view! {
                                                        <li class="blog-post">
                                                            <div class="blog--preview-content">
                                                                <div>
                                                                    <a href=href>{text}</a>
                                                                    <span class="date-blog-release">{file.date.to_string()}</span>
                                                                    <p>{file.desc.to_string()}</p>
                                                                </div>
                                                                <div class="image-container"> 
                                                                    <img src={file.image.to_string()} loading="lazy" />
                                                                </div>
                                                            </div>
                                                        </li>
                                                    }
                                                })
                                                .collect::<Vec<_>>()
                                        } else {
                                            let fallback_href: String = "#".to_string();
                                            vec![
                                                view! {
                                                    <li class="blog-post">
                                                        <div class="blog--preview-content">
                                                            <div>
                                                                <a href=fallback_href>{"No posts found".to_string()}</a>
                                                                <span class="date-blog-release">{"2025-02-01".to_string()}</span>
                                                                <p>{"".to_string()}</p>
                                                            </div>
                                                            <div class="image-container"> 
                                                                <img src={"image.png".to_string()} loading="lazy" />
                                                            </div>
                                                        </div>
                                                    </li>
                                                },
                                            ]
                                        }}
                                    </ul>
                                }
                                    .into_view()
                            }
                            Err(e) => {
                                let fallback_href: String = "#".to_string();
                                view! {
                                    <ul class="blog-list">
                                        {vec![
                                            view! {
                                                <li class="blog-post">

                                                    <div class="blog--preview-content">
                                                        <div>
                                                            <a href=fallback_href>
                                                                {"Error loading posts: ".to_string()}
                                                            </a>
                                                            <span class="date-blog-release">{"2999-99-99".to_string()}</span>
                                                            <p>{e.to_string()}</p>
                                                        </div>

                                                            <div class="image-container"> 
                                                                <img src={"image.png".to_string()} loading="lazy" />
                                                            </div>
                                                    </div>
                                                </li>
                                            },
                                        ]}
                                    </ul>
                                }
                                    .into_view()
                            }
                        })
                }}
            </Suspense>
        </div>
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
