import { getElement, initSearchControls, renderResults, updateAuthUI, logout } from './common.js';

document.addEventListener('DOMContentLoaded', () => {
    initSearchControls();
    updateAuthUI(); 
});