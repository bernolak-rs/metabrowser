import { getElement, initSearchControls, renderResults, updateAuthUI } from './common.js';

document.addEventListener('DOMContentLoaded', () => {
    updateAuthUI();
    initSearchControls();
    initResultsFromQuery();
});

function initResultsFromQuery() {
    const resultsList = getElement('resultsList');
    if (!resultsList) return;

    const urlParams = new URLSearchParams(window.location.search);
    const query = urlParams.get('q');
    console.log('Query from URL:', query);
    if (query) {
        const searchInput = getElement('searchInput');
        if (searchInput) searchInput.value = query;
        performSearch(query);
    }
}

async function performSearch(query) {
    const wikiHero = getElement('wikiHero');
    const resultsList = getElement('resultsList');

    if (!resultsList || !wikiHero) {
        return;
    }

    console.log('Performing search for:', query);
    wikiHero.innerHTML = '';
    resultsList.innerHTML = '<div class="text-center py-5"><div class="spinner-border text-primary"></div></div>';

    try {
        const response = await fetch(`/search/${encodeURIComponent(query)}`);
        console.log('Search response status:', response.status);
        const results = await response.json();
        console.log('Search results:', results);
        renderResults(results);
    } catch (error) {
        console.log('Search error:', error);
        resultsList.innerHTML = '<div class="alert alert-danger">Search failed.</div>';
    }
}