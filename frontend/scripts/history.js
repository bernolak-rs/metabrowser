import { getElement } from './common.js';

document.addEventListener('DOMContentLoaded', () => {
    fetchHistory();
});

async function fetchHistory() {
    const list = getElement('historyList');
    if (!list) return;

    try {
        const response = await fetch('/history');
        if (response.ok) {
            const data = await response.json();
            list.innerHTML = data.map(h => `
                <li class="list-group-item">
                    <a href="results.html?q=${encodeURIComponent(h.query_text)}" onclick="console.log('History item clicked:', '${h.query_text}')" class="text-decoration-none d-flex justify-content-between align-items-center py-3">
                        <span class="fw-bold text-dark">${h.query_text}</span>
                        <small class="text-muted">${h.created_at}</small>
                    </a>
                </li>
            `).join('');
        } else {
            console.log('History fetch failed:', response.status);
        }
    } catch (err) {
        console.log('History fetch error:', err);
    }
}
