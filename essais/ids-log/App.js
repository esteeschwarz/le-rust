function showInputForm() {
    document.getElementById("input-form").style.display = "block";
    document.getElementById("data-table").style.display = "none";
}

function showDataTable() {
    document.getElementById("input-form").style.display = "none";
    document.getElementById("data-table").style.display = "block";
    fetchData();
}

async function saveData(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    const data = Object.fromEntries(formData.entries());

    try {
        const response = await fetch("http://localhost:5001/save", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(data),
        });
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
        const response = await fetch("http://localhost:5001/data");
        if (!response.ok) {
            throw new Error("Failed to fetch data");
        }
        const data = await response.json();
        const tbody = document.getElementById("data-body");
        tbody.innerHTML = data
            .map(
                (row) => `
                <tr>
                    <td>${row.id}</td>
                    <td>${row.field1}</td>
                    <td>${row.field2}</td>
                    <td>${row.field3}</td>
                    <td>${row.field4}</td>
                    <td>${row.field5}</td>
                    <td>${row.field6}</td>
                    <td>${row.field7}</td>
                    <td>${row.field8}</td>
                    <td>${row.field9}</td>
                    <td>${row.timestamp}</td>
                </tr>
            `
            )
            .join("");
    } catch (error) {
        console.error("Error:", error);
        alert("Failed to fetch data");
    }
}