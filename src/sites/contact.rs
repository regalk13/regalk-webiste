use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView { 

    return view! {
        <header class="title-main--page--container">
            <h1>CONTACT ME</h1>
        </header>
        <section class="contact-me--container">
            <h2>The Digital Way</h2>
            <img src="apu_band.gif" alt="Guys band playing a song" />

            <div class="email--contact-me">
                <br />
                <br />

                <ul>
                    <li>
                        <a href="mailto:contact@regalk.dev">contact@regalk.dev</a>
                    </li>
                    <li>GPG key for encrypting mail:</li>
                    <li>"curl -sL <> | gpg --import"</li>
                    <li>"Fingerprint: <>"</li>
                    <li>"All legitimate emails from me will be signed with my GPG key."</li>
                    <li>"(The email will be aviable ASAP)."</li>
                </ul>
                <br />
                <br />
                <p>
                    "I’m not on Twitter, I’m not on Facebook, I'm not on Instagram, I’m not on Reddit, I don’t post on 4chan. I don't like social media in general you can have contact with me through email, my RSS channel and sometimes IRC."
                </p>
            </div>

        </section>

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
