function showInputForm() {
    document.getElementById("App").style.display = "table";
    document.getElementById("input-form").style.display = "block";
    document.getElementById("data-table").style.display = "none";
    document.getElementById("login").style.display = "block";

}

function showDataTable() {
    document.getElementById("App").style.display = "table";
    document.getElementById("input-form").style.display = "none";
    document.getElementById("login").style.display = "block";
    document.getElementById("data-table").style.display = "block";
    fetchData();
}
function showLogin() {
    document.getElementById("App").style.display = "table";
    document.getElementById("login").style.display = "block";
    // fetchData();
}
async function saveData(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    const data = Object.fromEntries(formData.entries());
    //const metaData = new LoginRequest(event.target);
    //const meta = Object.fromEntries(metaData.entries());

   // try {
        // const response = await fetch("http://mini12:4173/save", {
            try {
                // const response = await fetch("http://mini12:4173/save", {
                // const response = await fetch("/rserver/save", {
                //         method: "POST",
                //     headers: { "Content-Type": "application/json" },
                //     body: JSON.stringify(data),
                // });
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
        // const response = await fetch("http://mini12:4173/data");
        // const response = await fetch("/rserver/data"); //content error // .rs: was get(), now post()
    //    const response = await fetch("/data"); //404
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
            throw new Error("failed endpoint /data\n");
        }
        const data = await response.json();
        console.log("data received:")
        console.log(data)
        //const tbody = document.getElementById("data-body");
        const tcbody = document.getElementById("data-c-body");
        // tcbody.innerHTML = data
        //     .map(
        //         (row) => `
        //             <th>${row.timestamp}</th>
        //             <tr>${row.field1}</tr>
        //             <tr>${row.field2}</tr>
        //             <tr>${row.field3}</tr>
        //             <tr>${row.field4}</tr>
        //             <tr>${row.field5}</tr>
        //             <tr>${row.field6}</tr>
        //             <tr>${row.field7}</tr>
        //             <tr>${row.field8}</tr>
        //             <tr>${row.field9}</tr>
        //         `
        //     )
            tcbody.innerHTML = data
            .map(
                (row) => `
                    <p class="date">${row.timestamp}</p>
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
        // tbody.innerHTML = data
        //     .map(
        //         (row) => `
        //         <tr>
        //             <td>${row.id}</td>
        //             <td>${row.field1}</td>
        //             <td>${row.field2}</td>
        //             <td>${row.field3}</td>
        //             <td>${row.field4}</td>
        //             <td>${row.field5}</td>
        //             <td>${row.field6}</td>
        //             <td>${row.field7}</td>
        //             <td>${row.field8}</td>
        //             <td>${row.field9}</td>
        //             <td>${row.timestamp}</td>
        //         </tr>
        //     `
        //     )
            .join("");
    } catch (error) {
        console.error("Error:", error);
        alert("Failed to fetch data");
    }
}