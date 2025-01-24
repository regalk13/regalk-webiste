use leptos::prelude::*;


#[component]
pub fn Contact() -> impl IntoView { 

    return view! {
        <header class="title-main--page--container">
            <h1>
                CONTACT ME
            </h1>
        </header>
        <section class="contact-me--container">
            <h2>The Digital Way</h2>
            <img src="apu_band.gif" />

            <div class="email--contact-me">
                    <br />
                    <br />

                    <ul>
                    <li>contact@regalk.dev</li>
                    <li>GPG key for encrypting mail:</li>
                    <li>
                            "curl -sL <> | gpg --import"
                    </li>
                    <li>
                        "Fingerprint: <>"
                    </li>
                    <li>"All legitimate emails from me will be signed with my GPG key."</li>
                </ul>
                <br />
                <br />
                <p>"I’m not on Twitter, I’m not on Facebook, I'm not on Instagram, I’m not on Reddit, I don’t post on 4chan. I don't like social media in general you can have contact with me through email, my RSS channel and sometimes IRC."</p>
            </div>
        

        </section>

        <footer>
        <div class="footer-container">
            <div class="footer-main">
                <div class="footer-section">
                    <h4>"Contact"</h4>
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
                    <h4>"RSS Feed"</h4>
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