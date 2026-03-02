const getElement = (id) => document.getElementById(id);

function initSearchControls() {
    const searchBtn = getElement('searchBtn');
    const searchInput = getElement('searchInput');

    if (!searchBtn || !searchInput) return;

    searchBtn.addEventListener('click', () => {
        const newQuery = searchInput.value.trim();
        if (newQuery) {
            window.location.href = `results.html?q=${encodeURIComponent(newQuery)}`;
        }
    });

    searchInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') searchBtn.click();
    });
}

function renderResults(results) {
    const wikiHero = getElement('wikiHero');
    const resultsList = getElement('resultsList');

    if (!resultsList || !wikiHero) return;

    wikiHero.innerHTML = '';
    resultsList.innerHTML = '';

    if (!results || results.length === 0) {
        resultsList.innerHTML = '<div class="text-center py-5 text-muted">No results found.</div>';
        return;
    }

    const wiki = results.find(item => item.source === "Wikipedia");
    const web = results.filter(item => item.source !== "Wikipedia");

    if (wiki) {
        wikiHero.innerHTML = `
            <div class="wiki-hero-container mb-5 p-5 bg-white rounded-4 shadow-sm border">
                <div class="d-flex align-items-center mb-3">
                    <span class="badge bg-primary me-2">W</span>
                    <span class="text-uppercase fw-bold text-muted small">Wikipedia Summary</span>
                </div>
                <h1 class="display-5 fw-bold text-dark mb-3">${wiki.title}</h1>
                <p class="lead text-secondary mb-4" style="line-height: 1.8;">${wiki.description}</p>
                <a href="${wiki.url}" target="_blank" class="btn btn-primary rounded-pill px-4">Read Full Article</a>
            </div>
            <h4 class="mb-4 text-dark fw-bold opacity-75 ms-2">Web Results</h4>
        `;
    }

    web.forEach(item => {
        resultsList.insertAdjacentHTML('beforeend', `
            <div class="card mb-3 border-0 shadow-sm result-card">
                <div class="card-body p-4">
                    <h5 class="card-title mb-1">
                        <a href="${item.url}" target="_blank" class="text-decoration-none">${item.title}</a>
                    </h5>
                    <small class="text-success d-block mb-2 text-truncate">${item.url}</small>
                    <p class="card-text text-dark small mb-3">${item.description}</p>
                    <span class="badge bg-light text-secondary border fw-normal">${item.source}</span>
                </div>
            </div>
        `);
    });
}

async function updateAuthUI() {
    const userNav = getElement('userNav');
    if (!userNav) return;

    try {
        const res = await fetch('/history');
        if (res.ok) {
            userNav.innerHTML = `
                <div class="dropdown">
                    <button class="btn btn-light dropdown-toggle rounded-pill shadow-sm" data-bs-toggle="dropdown">
                        User Account
                    </button>
                    <ul class="dropdown-menu dropdown-menu-end shadow border-0">
                        <li><a class="dropdown-item" href="history.html">Search History</a></li>
                        <li><hr class="dropdown-divider"></li>
                        <li><button class="dropdown-item text-danger" onclick="logout()">Logout</button></li>
                    </ul>
                </div>`;
        }
    } catch (e) { }
}

async function logout() {
    await fetch('/logout', { method: 'POST' });
    window.location.href = 'index.html';
}

document.addEventListener('DOMContentLoaded', () => {
    initSearchControls();
    updateAuthUI(); 
});