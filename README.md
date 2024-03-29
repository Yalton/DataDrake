## Data Drake

## Table Of Contents
- [Data Drake](#data-drake)
- [Table Of Contents](#table-of-contents)
- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
    - [Non-Docker](#non-docker)
    - [Docker](#docker)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

## Introduction

Data Drake is an application with a simple purpose: to provide persistent, on-demand file indexing. Inspired by the GNOME Disk Utility, but with the added benefit of persistent storage, Data Drake allows you to scan directories and determine, with as much or as little granularity as you like, what is taking up all your space.

Data Drake is built using Sveltekit for the frontend and Rust for the backend, providing a fast and efficient user experience.

## Features

- Persistent storage of file indexing results
- Customizable scanning granularity
- Intuitive and user-friendly interface
- Fast and efficient performance
- Cross-platform compatibility

## Getting Started

To get started with Data Drake, follow these steps:

### Prerequisites
Before installing Data Drake, ensure that you have the following dependencies installed on your local machine:

- Node.js (>=18.0.0)
- NPM (>=6.0.0)
- Rust (>=1.57.0) and Cargo (Rust's package manager)

You can check if you have these dependencies installed by running `node -v`, `npm -v`, `yarn -v`, and `rustc --version` in your terminal.

### Installation

#### Non-Docker

1. Add `DATA_DRAKE_AUTH_TOKEN` to your system environment variables and generate a secure string for the value.

2. Navigate to the `backend` directory and bring it up with the following command:

   ```bash
   cd backend
   cargo run
   ```

3. Navigate to the `frontend` directory:

   ```bash
   cd ../frontend
   ```

4. Create a `.env` file within the `frontend` directory and add the following variables:

   ```
   DATA_DRAKE_AUTH_TOKEN=your_auth_token_value
   DATA_DRAKE_SERVER_URI=http://localhost:8000
   ```

   Replace `your_auth_token_value` with the secure string you generated for `DATA_DRAKE_AUTH_TOKEN` and update `DATA_DRAKE_SERVER_URI` with the URI where your server is running.

5. Build the frontend with the following command:

   ```bash
   npm run build
   ```

6. Run the frontend with the following command:

   ```bash
   npm run preview
   ```

#### Docker

1. Navigate to the `frontend` directory:

   ```bash
   cd frontend
   ```

2. Create a `.env` file within the `frontend` directory and add the following variables:

   ```
   DATA_DRAKE_AUTH_TOKEN=your_auth_token_value
   DATA_DRAKE_SERVER_URI=http://backend:8000
   ```

   Replace `your_auth_token_value` with the secure string you generated for `DATA_DRAKE_AUTH_TOKEN`. Note that the `DATA_DRAKE_SERVER_URI` should use the service name `backend` as defined in the Docker Compose file.

3. Open the `docker-compose.yml` file in the project root directory.

4. In the `backend` service section, add the following volume mount to specify the desired directories for scanning:

   ```yaml
   services:
     backend:
       ...
       volumes:
         - /path/to/scan/directory:/app/scanned_data
   ```

   Replace `/path/to/scan/directory` with the actual path to the directory you want to scan.

5. Build the project with the following command:

   ```bash
   docker compose build
   ```

6. Run the project with the following command:

   ```bash
   docker compose up
   ```

   This will start both the backend and frontend services defined in the Docker Compose file.

Make sure you have Docker and Docker Compose installed on your machine before running the Docker-specific commands.
## Usage

Once you have Data Drake up and running, you can start scanning directories and analyzing disk usage. Simply select the directory you want to scan, choose the desired scanning granularity, and let Data Drake do the rest. The results will be persistently stored, allowing you to access them later and track changes over time.

## Roadmap

Here are some of the features and improvements planned for future releases of Data Drake:
- [ ] Enhanced visualization options for disk usage analysis
- [ ] Support for remote file systems and network drives
- [ ] Integration with cloud storage services
- [ ] Advanced filtering and search capabilities
- [ ] Customizable reporting and export options

## Contributing

We welcome contributions from the community to help improve Data Drake. If you would like to contribute, please follow these steps:
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Create a new Pull Request

## License

Data Drake is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

If you have any questions, suggestions, or feedback, feel free to reach out to us:
- Email: contact@datadrake.com
- Project Link: [https://github.com/your-username/data-drake](https://github.com/your-username/data-drake)

## Acknowledgments

We would like to express our gratitude to the following communities and projects for their support and inspiration:
- Sveltekit community
- Rust community
- GNOME Disk Utility project

Special thanks to all the contributors who have helped make Data Drake better!