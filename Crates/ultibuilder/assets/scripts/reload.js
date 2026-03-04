if (window.EventSource) {
    const source = new EventSource("/reload");
    source.onmessage = (e) => {
        if (e.data === "reload") location.reload();
    };
}

document.addEventListener('DOMContentLoaded', async () => {
    const container = document.querySelector('#sidebar-container');

    // 1️⃣ Load sidebar first
    if (container) {
        const res = await fetch('/sidebar.html');
        const html = await res.text();
        container.innerHTML = html;
    }

    // 2️⃣ Now query sidebar AFTER it exists
    const sidebar = document.querySelector('.sidebar');
    const toggleBtn = document.querySelector('#sidebar-toggle');

    if (!sidebar) return;

    function expandParents(element) {
        let parent = element.closest('details');
        while (parent) {
            parent.open = true;
            parent = parent.parentElement.closest('details');
        }
    }

    function scrollToCurrentLink() {
        const currentPath = window.location.pathname;
        const allLinks = sidebar.querySelectorAll('a[href]');
        for (const link of allLinks) {
            const linkPath = new URL(link.href).pathname;
            if (linkPath === currentPath) {
                link.classList.add('current');
                expandParents(link);
                link.scrollIntoView({ behavior: 'smooth', block: 'center' });
                break;
            }
        }
    }

    // Toggle sidebar
    if (toggleBtn) {
        toggleBtn.addEventListener('click', () => {
            document.body.classList.toggle('sidebar-open');

            if (document.body.classList.contains('sidebar-open')) {
                scrollToCurrentLink();
            }
        });
    }

    // Initial highlight
    scrollToCurrentLink();
});