import { useState, ChangeEvent, FormEvent } from "react";
import "./styles.css";

// Define the type for formData
interface FormData {
  field1: string;
  field2: string;
  field3: string;
  field4: string;
  field5: string;
  field6: string;
  field7: string;
  field8: string;
  field9: string;
}

function App() {
  const [formData, setFormData] = useState<FormData>({
    field1: "",
    field2: "",
    field3: "",
    field4: "",
    field5: "",
    field6: "",
    field7: "",
    field8: "",
    field9: "",
  });

  // Added type annotation for ChangeEvent<HTMLInputElement>
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData({ ...formData, [name]: value });
  };

  // Added type annotation for FormEvent<HTMLFormElement>
  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    try {
      const response = await fetch("http://localhost:5000/save", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(formData),
      });

      if (!response.ok) {
        throw new Error(`Error: ${response.statusText}`);
      }

      const result = await response.json();
      alert(`Data saved with ID: ${result.id}`);
    } catch (error) {
      console.error("Error saving data:", error);
      alert("Failed to save data. Please try again.");
    }
  };

  return (
    <div className="App">
      <h1>Database Entry Form</h1>
      <form onSubmit={handleSubmit}>
        {[...Array(9)].map((_, i) => (
          <div key={i}>
            <label htmlFor={`field${i + 1}`}>Field {i + 1}:</label>
            <input
              type="text"
              id={`field${i + 1}`}
              name={`field${i + 1}`}
              value={formData[`field${i + 1}` as keyof FormData]} 
              onChange={handleChange}
              required
            />
          </div>
        ))}
        <button type="submit">Save</button>
      </form>
    </div>
  );
}

export default App;