use leptos::prelude::*;


#[component]
pub fn Library() -> impl IntoView { 

    return view! {
        <header class="title-main--page--container library">
            <h1>
                LIBRARY
            </h1>
        </header>
        <section class="main--library-container">
            <div class="main--library-text">
            <p>
"This is my curated collection of both physical and digital resources - a library of books (fiction and non-fiction, and materials I find interesting across various disciplines. Currently organized into: computer science, mathematics, political science, linguistics/languages, and extra topics. This ever-growing collection reflects my diverse approach to learning but no all my opinions."        </p>                
            </div>

            <div class="collection-books"> 
            <section class="book-category">
                <div class="category-content">
                <aside class="featured-book">
                    <img src="featured-cs-book.webp" alt="Cover the C Programming Language - (K&R)" class="book-cover" />
                </aside>
                <br />
                    <ul class="book-list">
                        <li>"The Art of Computer Programming - Knuth"</li>
                        <li>"Clean Code - Martin"</li>
                        <li>"Design Patterns: Elements of Reusable Object-Oriented Software"</li>
                        <li>"Introduction to Algorithms - CLRS"</li>
                        <li>"Code: The Hidden Language of Computer Hardware and Software - Petzold"</li>
                        <li>"The C Programming Language - (K&R)"</li>
                        <li>"The Pragmatic Programmer - Andrew Hunt, David Thomas"</li>
                        <li>"Computer Networking: A Top-Down Approach - Kurose, Ross"</li>
                        <li>"Designing Data-Intensive Applications, 2nd Edition -  Martin Kleppmann, Chris Riccomini"</li>
                        <li>"Quantum Computation and Quantum Information - Michael Nielsen, Isaac Chuang"</li>
                        <li>"Computer Systems: A Programmer’s Perspective - Randal Bryant, David O'Hallaron"</li>
                        <li>"The Mythical Man-Month - Fred Brooks"</li>
                        <li>" What Every Computer Scientist Should Know About Floating-Point Arithmetic"</li>
                        <li>"Linear Algebra Done Right - Axl"</li>
                        <li>"Hurley, Patrick J. – A Concise Introduction to Logic"</li>
                        <li>"Calculus Made Easy - Thompson"</li>
                        <li>"Discrete Mathematical Structures - Bernard Kolman, Robert Busby, Sharon Ross"</li>
                        <li>"How to Prove It: A Structured Approach - Daniel J. Velleman"</li>
                        <li>"Elementary Statistic - Neil Weiss"</li>
                        <li>"Graph Theory - Ronald Gould"</li>
                    </ul>
                    <br />

                    <aside class="featured-book">
                    <img src="featured-sc-book.webp" alt="Cover meditations Marco Aurelio" class="book-cover" />
                    </aside>
                    <br />

                    <ul class="book-list">
                    <li>"The Republic - Plato"</li>
                    <li>"Leviathan - Thomas Hobbes"</li>
                    <li>"The Social Contract - Jean-Jacques Rousseau"</li>
                    <li>"1984 - George Orwell"</li>
                    <li>"Animal Farm - George Orwell"</li>
                    <li>"The End of History and the Last Man"</li>
                    <li>"The Age of Surveillance Capitalism"</li>
                    <li>"A Brief History of Intelligence: Evolution, AI, and the Five Breakthroughs That Made Our Brains"</li>
                    <li>"The Age of Em: Work, Love and Life when Robots Rule the Earth"</li>
                    <li>"Asimov, Isaac – Before the Golden Age: A Science Fiction Anthology of the 1930's "</li>
                    <li>"Kurzweil, Ray – The Singularity is Near "</li>
                    <li>"Kurzweil, Ray – The Singularity is Nearer "</li>
                    <li>"Nexus: A Brief History of Information Networks from the Stone Age to AI"</li>
                    <li>"Sapiens: A Brief History of Humankind"</li>
                    <li>"The Last Question - Isaac Asimov"</li>
                    <li>"Notes from Underground - Fyodor Dostoevsky"</li>
                    <li>"Crime and Punishment - Fyodor Dostoevsky"</li>
                    <li>"The Idiot - Fyodor Dostoevsky"</li>
                    <li>"The Brothers Karamazov - Fyodor Dostoevsky"</li>
                    <li>"MEDITATIONS, Marcus Aurelius"</li>
                    <br />


                    <aside class="featured-book">
                    <img src="featured-cc-book.webp" alt="Cover One Hundred Years of Solitud" class="book-cover" />
                    </aside>
                    </ul>
                    <br />

                    <ul>
                        <li>"Dalby, Andrew – Dictionary of Languages, the Definitive Reference to More than 400 Languages "</li>
                        <li>"Watkins, Calvert – The American Heritage Dictionary of Indo-European Roots "</li>
                        <li>"Wheelock, Frederic M. – Latin, an Introductory Course Based on Ancient Authors "</li>
                        <li>"Euripides – Three Plays of Euripides: Alcestis, Medea, the Bacchae"</li>
                        <li>"Lucretius – On the Nature of Things"</li>
                        <li>"García Márquez, Gabriel – One Hundred Years of Solitude"</li>
                        <li>"Tolkien, J. R. R. – The Lord of the Rings, One-Volume Edition"</li>
                        <li>"Lovecraft, H. P. – Great Tales of Horror"</li>
                        <li>"Lyon, Pamela – French Short Stories; Nouvelles françaises"</li>
                    </ul>
                
                </div>

            </section>
            

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
