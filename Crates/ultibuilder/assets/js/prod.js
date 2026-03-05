document.addEventListener('DOMContentLoaded', () => {

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
                link.scrollIntoView({
                    behavior: 'smooth',
                    block: 'center'
                });
                break;
            }
        }
    }

    if (toggleBtn) {
        toggleBtn.addEventListener('click', () => {
            document.body.classList.toggle('sidebar-open');
            if (document.body.classList.contains('sidebar-open')) {
                scrollToCurrentLink();
            }
        });
    }

    scrollToCurrentLink();
});