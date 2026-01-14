const getElement = (id) => document.getElementById(id);

document.addEventListener('DOMContentLoaded', () => {
    updateAuthUI();

    const searchBtn = getElement('searchBtn');
    const searchInput = getElement('searchInput');
    const resultsList = getElement('resultsList');

    if (searchBtn && searchInput) {
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

    if (resultsList) {
        const urlParams = new URLSearchParams(window.location.search);
        const query = urlParams.get('q');
        if (query) {
            if (searchInput) searchInput.value = query;
            performSearch(query);
        }
    }

    if (getElement('historyList')) {
        fetchHistory();
    }

    const authForm = getElement('authForm');
    const toggleBtn = getElement('toggleAuth');
    const authTitle = getElement('authTitle');

    if (authForm) {
        let isLogin = true;

        if (toggleBtn) {
            toggleBtn.onclick = (e) => {
                e.preventDefault();
                isLogin = !isLogin;
                authTitle.innerText = isLogin ? 'Login' : 'Login';
                toggleBtn.innerText = isLogin ? "Don't have and account? Register" : 'Already have an account? Login';
            }
        }

        authForm.onsubmit = async (e) => {
            e.preventDefault();
            const username = getElement('username').value;
            const password = getElement('password').value;
            const endpoint = isLogin ? 'login' : 'register';

            try {
                const response = await fetch(endpoint, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ username, password }),
                });

                if (response.ok) {
                    if (isLogin) {
                        window.location.href = 'index.html';
                    } else {
                        alert("Registered successfully! You can now login.");
                        isLogin = false;
                        authTitle.innerText = 'Login';
                        toggleBtn.innerText = "Don't have and account? Register";
                        authForm.reset();
                    }
                } else
                    alert("Login failed. Username might be taken.");
            } catch (error) {
                alert("Login failed. Please try again later.");
            }
        };
    }
});

async function performSearch(query) {
    const wikiHero = getElement('wikiHero');
    const resultsList = getElement('resultsList');

    if (!resultsList || !wikiHero) {
        return;
    }

    wikiHero.innerHTML = '';
    resultsList.innerHTML = '<div class="text-center py-5"><div class="spinner-border text-primary"></div></div>';

    try {
        const response = await fetch(`/search/${encodeURIComponent(query)}`);
        const results = await response.json();
        renderResults(results);
    } catch (error) {
        resultsList.innerHTML = '<div class="alert alert-danger">Search failed.</div>';
    }
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

async function fetchHistory() {
    const list = getElement('historyList');
    if (!list) return;

    try {
        const response = await fetch('/history');
        if (response.ok) {
            const data = await response.json();
            list.innerHTML = data.map(h => `
                <li class="list-group-item d-flex justify-content-between align-items-center py-3">
                    <a href="results.html?q=${encodeURIComponent(h.query_text)}" class="text-decoration-none fw-bold text-dark">${h.query_text}</a>
                    <small class="text-muted">${h.created_at}</small>
                </li>
            `).join('');
        }
    } catch (err) { }
}
