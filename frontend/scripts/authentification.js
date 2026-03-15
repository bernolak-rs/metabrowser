import { getElement } from './common.js';

document.addEventListener('DOMContentLoaded', () => {
    initAuthForm();
});

function initAuthForm() {
    const authForm = getElement('authForm');
    const toggleBtn = getElement('toggleAuth');
    const authTitle = getElement('authTitle');

    if (!authForm) return;

    let isLogin = true;

    if (toggleBtn) {
        toggleBtn.addEventListener('click', (e) => {
            e.preventDefault();
            isLogin = !isLogin;
            if (authTitle) authTitle.innerText = isLogin ? 'Login' : 'Register';
            toggleBtn.innerText = isLogin ? "Don't have an account? Register" : 'Already have an account? Login';
        });
    }

    authForm.addEventListener('submit', async (e) => {
        console.log("Hello");
        e.preventDefault();
        const usernameEl = getElement('username');
        const passwordEl = getElement('password');
        const username = usernameEl ? usernameEl.value : '';
        const password = passwordEl ? passwordEl.value : '';
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
                    isLogin = true;
                    if (authTitle) authTitle.innerText = 'Login';
                    if (toggleBtn) toggleBtn.innerText = "Don't have an account? Register";
                    authForm.reset();
                }
            } else {
                alert(isLogin ? "Login failed. Check credentials." : "Registration failed. Username might be taken.");
            }
        } catch (error) {
            alert(isLogin ? "Login failed. Please try again later." : "Registration failed. Please try again later.");
        }
    });
}