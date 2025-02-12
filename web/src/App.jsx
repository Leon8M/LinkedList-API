import React, { useState } from "react";

function App() {
    const [output, setOutput] = useState("");
    const API_BASE = "http://localhost:8000";

    const handleAdd = async () => {
        const data = document.getElementById("data").value;
        const response = await fetch(`${API_BASE}/add`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ data: parseInt(data) }),
        });
        const result = await response.text();
        setOutput(result);
    };

    const handleSearch = async () => {
        const key = document.getElementById("searchKey").value;
        const response = await fetch(`${API_BASE}/search?key=${key}`);
        const result = await response.text();
        setOutput(result);
    };

    const handleInsert = async () => {
        const data = document.getElementById("insertData").value;
        const index = document.getElementById("insertIndex").value;
        const response = await fetch(`${API_BASE}/insert`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ data: parseInt(data), index: parseInt(index) }),
        });
        const result = await response.text();
        setOutput(result);
    };

    const handleRemove = async () => {
        const key = document.getElementById("removeKey").value;
        const response = await fetch(`${API_BASE}/remove`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ key: parseInt(key) }),
        });
        const result = await response.text();
        setOutput(result);
    };

    const handleSize = async () => {
        const response = await fetch(`${API_BASE}/size`);
        const result = await response.text();
        setOutput(result);
    };

    return (
        <div className="min-h-screen bg-gray-100 py-8 px-4 sm:px-6 lg:px-8">
            <div className="max-w-3xl mx-auto bg-white shadow-lg rounded-lg p-6">
                <h1 className="text-3xl font-bold text-center text-gray-800 mb-6">
                LinkedList API
                </h1>

                <div className="space-y-4">
                    <div className="flex flex-col sm:flex-row gap-2">
                        <input
                            type="number"
                            id="data"
                            placeholder="Enter data"
                            className="flex-1 p-2 border border-gray-300 rounded-lg"
                        />
                        <button
                            onClick={handleAdd}
                            className="bg-blue-500 text-white p-2 rounded-lg hover:bg-blue-600 transition"
                        >
                            Add
                        </button>
                    </div>

                    <div className="flex flex-col sm:flex-row gap-2">
                        <input
                            type="number"
                            id="searchKey"
                            placeholder="Enter key to search"
                            className="flex-1 p-2 border border-gray-300 rounded-lg"
                        />
                        <button
                            onClick={handleSearch}
                            className="bg-green-500 text-white p-2 rounded-lg hover:bg-green-600 transition"
                        >
                            Search
                        </button>
                    </div>

                    <div className="flex flex-col sm:flex-row gap-2">
                        <input
                            type="number"
                            id="insertData"
                            placeholder="Enter data"
                            className="flex-1 p-2 border border-gray-300 rounded-lg"
                        />
                        <input
                            type="number"
                            id="insertIndex"
                            placeholder="Enter index"
                            className="flex-1 p-2 border border-gray-300 rounded-lg"
                        />
                        <button
                            onClick={handleInsert}
                            className="bg-yellow-500 text-white p-2 rounded-lg hover:bg-yellow-600 transition"
                        >
                            Insert
                        </button>
                    </div>

                    <div className="flex flex-col sm:flex-row gap-2">
                        <input
                            type="number"
                            id="removeKey"
                            placeholder="Enter key to remove"
                            className="flex-1 p-2 border border-gray-300 rounded-lg"
                        />
                        <button
                            onClick={handleRemove}
                            className="bg-red-500 text-white p-2 rounded-lg hover:bg-red-600 transition"
                        >
                            Remove
                        </button>
                    </div>

                    <div className="flex justify-center">
                        <button
                            onClick={handleSize}
                            className="bg-purple-500 text-white p-2 rounded-lg hover:bg-purple-600 transition"
                        >
                            Get Size
                        </button>
                    </div>
                </div>

                <div className="mt-6 p-4 bg-gray-50 rounded-lg">
                    <p className="text-gray-800 text-center">{output}</p>
                </div>
            </div>
        </div>
    );
}

export default App;