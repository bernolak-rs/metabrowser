const getElement = (id) => document.getElementById(id);

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
                <li class="list-group-item d-flex justify-content-between align-items-center py-3">
                    <a href="results.html?q=${encodeURIComponent(h.query_text)}" class="text-decoration-none fw-bold text-dark">${h.query_text}</a>
                    <small class="text-muted">${h.created_at}</small>
                </li>
            `).join('');
        }
    } catch (err) { }
}
