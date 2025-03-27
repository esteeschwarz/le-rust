function showInputForm() {
    document.getElementById("App").style.display = "table-row";
    document.getElementById("input-form").style.display = "block";
    document.getElementById("data-table").style.display = "none";
//    document.getElementById("login").style.display = "inline-table";
    document.getElementById("login").style.display = "none";

}

function showDataTable() {
    document.getElementById("App").style.display = "table-row";
    document.getElementById("input-form").style.display = "none";
    document.getElementById("login").style.display = "none";
    document.getElementById("data-table").style.display = "block";
    fetchData();
}
function showLogin() {
    document.getElementById("App").style.display = "table-row";
    document.getElementById("login").style.display = "inline-table";
    // fetchData();
}
async function saveData(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    const data = Object.fromEntries(formData.entries());
            try {
                const response = await fetch("/rserver/save", {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({
                        data,
                        // meta
                        table_name: window.tableName,
                        password: window.password
                    }),
                });
                console.log("savin post() data rserver/save");
                console.log(window.tableName);
                console.log(window.password);
            
                console.log("< window.tableName");
            
        if (!response.ok) {
            throw new Error("Failed to save data");
        }
        alert("Data saved successfully");
        event.target.reset();
    } catch (error) {
        console.error("Error:", error);
        alert("Failed to save data");
    }
}

async function fetchData() {
    try {
      const response = await fetch("/rserver/data", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ table_name: window.tableName, password: window.password }),
    });
    console.log("fetching post() data rserver/data");
    console.log(window.tableName);
    console.log(window.password);
    console.log("< window.tableName");


        if (!response.ok) {
            throw new Error("failed endpoint rserver/data\n");
        }
        const data = await response.json();
        console.log("data received:")
        console.log(data)
        const tcbody = document.getElementById("data-c-body");
             tcbody.innerHTML = data
            .map(
                (row) => `
                    <h2 class="date">${row.timestamp}</h2>
                    <p>${row.field1}</p>
                    <p>${row.field2}</p>
                    <p>${row.field3}</p>
                    <p>${row.field4}</p>
                    <p>${row.field5}</p>
                    <p>${row.field6}</p>
                    <p>${row.field7}</p>
                    <p>${row.field8}</p>
                    <p>${row.field9}</p>
                `
            )
            .join("");
    } catch (error) {
        console.error("Error:", error);
        alert("Failed to fetch data");
    }
}