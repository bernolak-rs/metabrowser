import { getElement, initSearchControls, renderResults, updateAuthUI, logout } from './common.js';

document.addEventListener('DOMContentLoaded', () => {
    console.log("index listeners")
    initSearchControls();
    updateAuthUI(); 
});