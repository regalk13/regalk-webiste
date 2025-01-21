let observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('visible');
        }
    });
}, {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
});
console.log(observer);

document.querySelectorAll('.scroll-appear').forEach(element => observer.observe(element));
document.querySelectorAll('.staggered-children').forEach(container => observer.observe(container));


const elements = document.querySelectorAll('.main--image--quote');

elements.forEach((element) => {
    // Add the mousemove event listener
    element.addEventListener('mousemove', (e) => {
        if (!element.matches(':hover')) return;

        const rect = element.getBoundingClientRect();
        const x = e.clientX - rect.left;a
        const y = e.clientY - rect.top;

        const xPercent = x / rect.width;
        const yPercent = y / rect.height;

        const shadowX = (xPercent - 0.5) * 30;
        const shadowY = (yPercent - 0.5) * 30;
        const shadowBlur = Math.max(15, Math.abs(shadowX) + Math.abs(shadowY));

        const distance = Math.sqrt(Math.pow(xPercent - 0.5, 2) + Math.pow(yPercent - 0.5, 2));
        const intensity = Math.min(0.8, 0.3 + distance);

        element.style.boxShadow = `${shadowX}px ${shadowY}px ${shadowBlur}px rgba(255, 255, 255, ${intensity})`;
    });

    element.addEventListener('mouseleave', () => {
        element.style.transition = 'box-shadow 0.5s ease'; // Add the transition duration and easing
        element.style.boxShadow = '0 4px 15px rgba(255, 255, 255, 0.6)';
    });
});




export function initTypewriter(element, words) {
    console.log("Initializing typewriter with element:", element);
    console.log("Words:", words);

    function typeWriter(text, i, fnCallback) {
        if (i < text.length) {
            element.innerHTML = text.substring(0, i + 1) + '<span class="cursor" aria-hidden="true"></span>';
            setTimeout(function () {
                typeWriter(text, i + 1, fnCallback)
            }, 100);
        }
        else if (typeof fnCallback == 'function') {
            setTimeout(fnCallback, 700);
        }
    }

    function startTextAnimation(i) {
        if (!words[i]) {
            setTimeout(function () {
                startTextAnimation(0);
            }, 20000);
        } else if (typeof words[i] == 'undefined') {
            setTimeout(function () {
                startTextAnimation(0);
            }, 20000);
        } else if (i < words[i].length) {
            typeWriter(words[i], 0, function () {
                startTextAnimation(i + 1);
            });
        }
    }

    startTextAnimation(0);
}
