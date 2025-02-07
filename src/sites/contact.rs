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
    }
}
