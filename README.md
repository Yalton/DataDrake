## Data Drake

## Table Of Contents
- [Data Drake](#data-drake)
- [Table Of Contents](#table-of-contents)
- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
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
1. Clone the repository to your local machine:
   ```bash
   git clone https://github.com/your-username/data-drake.git
   ```

2. Navigate into the project directory:
   ```bash
   cd data-drake
   ```

3. Install the dependencies:
   ```bash
   yarn install
   ```

4. Build the Rust backend:
   ```bash
   cargo build --release
   ```

5. Start the development server:
   ```bash
   yarn dev
   ```

6. Open your browser and go to [http://localhost:3000](http://localhost:3000) to view Data Drake in action.

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