const searchBtn = document.getElementById('searchBtn');
const searchInput = document.getElementById('searchInput');
const resultsList = document.getElementById('resultsList');
const wikiHero = document.getElementById('wikiHero');

document.addEventListener('DOMContentLoaded', () => {
    const urlParams = new URLSearchParams(window.location.search);
    const query = urlParams.get('q');
    if (query) {
        searchInput.value = query;
        performSearch(query);
    }
});

async function performSearch(query) {
    if (!query) return;
    
    searchBtn.disabled = true;
    wikiHero.innerHTML = '';
    resultsList.innerHTML = '<div class="text-center py-5"><div class="spinner-border text-primary"></div></div>';

    try {
        const response = await fetch(`/search/${encodeURIComponent(query)}`);
        if (!response.ok) throw new Error(`Server responded with ${response.status}`);
        const data = await response.json();
        renderResults(data);
    } catch (error) {
        resultsList.innerHTML = `<div class="alert alert-danger">Error: ${error.message}</div>`;
    } finally {
        searchBtn.disabled = false;
    }
}

function renderResults(results) {
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
            <div class="wiki-hero-container mb-5 p-5 bg-white rounded-4 shadow-sm">
                <div class="d-flex align-items-center mb-3">
                    <div class="wiki-icon me-2">W</div>
                    <span class="text-uppercase tracking-widest fw-bold text-muted small">Summary from Wikipedia</span>
                </div>
                <h1 class="display-4 fw-bold text-dark mb-3">${wiki.title}</h1>
                <p class="lead text-secondary mb-4" style="line-height: 1.8;">
                    ${wiki.snippet}
                </p>
                <div class="d-flex align-items-center gap-3">
                    <a href="${wiki.url}" target="_blank" rel="noopener" class="btn btn-primary btn-lg rounded-pill px-4">Read Full Article</a>
                    <small class="text-muted border-start ps-3">${new URL(wiki.url).hostname}</small>
                </div>
            </div>
            <h4 class="mb-4 text-dark fw-bold opacity-75 ms-2">Web Search Results</h4>
        `;
    }

    web.forEach(item => {
        resultsList.insertAdjacentHTML('beforeend', `
            <div class="card mb-3 border-0 shadow-sm result-card">
                <div class="card-body p-4">
                    <h5 class="card-title mb-1">
                        <a href="${item.url}" target="_blank" rel="noopener" class="text-decoration-none">${item.title}</a>
                    </h5>
                    <small class="text-success d-block mb-2 text-truncate">${item.url}</small>
                    <p class="card-text text-dark small mb-3">${item.snippet}</p>
                    <span class="badge bg-light text-secondary border fw-normal">${item.source}</span>
                </div>
            </div>
        `);
    });
}

searchBtn.addEventListener('click', () => {
    const newQuery = searchInput.value.trim();
    if (newQuery) {
        window.location.href = `results.html?q=${encodeURIComponent(newQuery)}`;
    }
});

searchInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        searchBtn.click();
    }
});
