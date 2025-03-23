document.addEventListener("DOMContentLoaded", () => {
    const tableNameInput = document.getElementById("tableName");
    const passwordInput = document.getElementById("password");
    const loginBtn = document.getElementById("loginBtn");
    const createTableBtn = document.getElementById("createTableBtn");
    const message = document.getElementById("message");
    const masterPasswordInput = document.getElementById("masterpassword");

    const handleLogin = async () => {
        const tableName = tableNameInput.value;
        const password = passwordInput.value;
        const masterpassword = masterPasswordInput.value;

        const response = await fetch("/rserver/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ table_name: tableName, password}),
        });
        const result = await response.text();

        if (response.ok) {
                // Store tableName and password in the window object
                window.tableName = tableName;
                window.password = password;
                window.masterpassword = masterpassword;
            document.getElementById("App").style.display = "table";
            document.getElementById("login").style.display = "block";
            document.getElementById("masterpassword").style.display = "none";
            message.textContent = "login succesfull";


            fetchData();        
        } else {
            message.textContent = result;
        }
    };

    const handleCreateTable = async () => {
        const tableName = tableNameInput.value;
        const password = passwordInput.value;
        //const password = passwordInput.value;
        document.getElementById("masterpassword").style.display = "block";
        const masterpassword = masterPasswordInput.value;

        const response = await fetch("/rserver/create_table", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ table_name: tableName, password ,masterpassword}),
        });
        const result = await response.text();
        console.log("sending create table request...");
        if (response.ok) {
            document.getElementById("App").style.display = "table";
            document.getElementById("login").style.display = "none";
            document.getElementById("masterpassword").style.display = "none";
            message.textContent = "table created";

        } else {
            message.textContent = result;
        }
    };

    loginBtn.addEventListener("click", handleLogin);
    createTableBtn.addEventListener("click", handleCreateTable);
});