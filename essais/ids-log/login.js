document.addEventListener("DOMContentLoaded", () => {
    const tableNameInput = document.getElementById("tableName");
    const passwordInput = document.getElementById("password");
    const loginBtn = document.getElementById("loginBtn");
    const createTableBtn = document.getElementById("createTableBtn");
    const message = document.getElementById("message");

    const handleLogin = async () => {
        const tableName = tableNameInput.value;
        const password = passwordInput.value;

        const response = await fetch("/rserver/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ table_name: tableName, password }),
        });
        const result = await response.text();

        if (response.ok) {
                // Store tableName and password in the window object
                window.tableName = tableName;
                window.password = password;
            document.getElementById("App").style.display = "table";
            document.getElementById("login").style.display = "block";
            message.textContent = "login succesfull";


            fetchData();        
        } else {
            message.textContent = result;
        }
    };

    const handleCreateTable = async () => {
        const tableName = tableNameInput.value;
        const password = passwordInput.value;

        const response = await fetch("/rserver/create_table", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ table_name: tableName, password }),
        });
        const result = await response.text();

        if (response.ok) {
            document.getElementById("App").style.display = "table";
            document.getElementById("login").style.display = "block";
            message.textContent = "table created";

        } else {
            message.textContent = result;
        }
    };

    loginBtn.addEventListener("click", handleLogin);
    createTableBtn.addEventListener("click", handleCreateTable);
});