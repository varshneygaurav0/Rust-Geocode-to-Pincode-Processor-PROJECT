📍 Rust Geocode to Pincode Processor

This is a containerized Rust application designed to perform reverse geocoding. It reads a list of geographic coordinates (latitude and longitude) from an Excel file, queries the [Nominatim (OpenStreetMap) API](https://www.google.com/search?q=https://nominatim.openstreetmap.org/ui/reverse.html) to find the corresponding postal code for each, and then writes the results to a new Excel file.

The entire application is packaged with Docker, allowing for a simple, one-command setup and execution on any machine with Docker installed.

-----

 ✨ Features

  * **Easy to Run**: The entire environment is containerized with Docker and orchestrated with a single `docker-compose` command. You don't need to install Rust or manage dependencies on your local machine.
  * **Excel Processing**: Reads input data directly from a `.xlsx` file and writes the output to a new `.xlsx` file.
  * **External API Integration**: Connects to the public Nominatim API to fetch real-world address data.
  * **Robust Logging**: The application prints detailed logs to the console during execution, making it easy to monitor its progress and debug issues.

-----

🛠️ Tech Stack

  * **Language**: [Rust](https://www.rust-lang.org/)
  * **Containerization**: [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)
  * **Key Crates**:
      * `reqwest`: HTTP client for making API calls.
      * `calamine`: For reading `.xlsx` files.
      * `xlsxwriter`: For writing the output `.xlsx` file.
      * `serde`: For parsing the JSON response from the API.

-----

📋 Prerequisites

Before you begin, ensure you have the following installed on your system:

  * **Docker**: [Download Docker](https://www.docker.com/products/docker-desktop/)
  * **Docker Compose**: This is included with all standard Docker Desktop installations.

-----

⚙️ Getting Started

Follow these steps to set up and run the application.

1\. Clone the Repository

First, clone this repository to your local machine.

```bash
git clone <your-repository-url>
cd <your-repository-name>
```

2\. Prepare the Input File

The application requires a specific folder structure to work correctly.

Create an `input` directory in the project root:

```bash
mkdir input
```

Inside this `input` directory, place your Excel file. By default, the application is configured to read a file named **`Input_one.xlsx`**.

The Excel file **must** have its data in `Sheet1` with the following two columns:

  * **Column A**: An identifier (e.g., Outlet Code).
  * **Column B**: The geographic coordinates in the format `"latitude,longitude"`.

**Example `Input_one.xlsx`:**

| Outlet Code | Geocode                |
|-------------|------------------------|
| OUTLET001   | 19.0760,72.8777        |
| OUTLET002   | 28.6139,77.2090        |
| OUTLET003   | 12.9716,77.5946        |

3\. Run the Application

Execute the following command from the project's root directory. This command will build the Docker image and run the application inside a container.

```bash
docker-compose up --build
```

4\. Check the Output

Once the application finishes, it will create a new file named **`pincode_output.xlsx`** inside the `output` directory in your project folder. This file will contain the identifiers and their corresponding fetched postal codes.

-----

📁 Project File Structure

```
.
├── input/
│   └── Input_one.xlsx      # Your input data file
├── output/
│   └── pincode_output.xlsx # Generated result file
├── src/
│   └── main.rs             # Main Rust application logic
├── Cargo.toml              # Rust project dependencies
├── Dockerfile              # Instructions to build the Docker image
└── docker-compose.yml      # Configuration to run the container
```