# Single-Threaded Web Server in Rust

A lightweight and efficient single-threaded web server built from scratch in Rust. This project demonstrates core concepts of systems programming, memory safety, and web communication using the Rust programming language.

---

## **Features**

- Serves static HTML files to clients via HTTP.
- Handles file uploads to the `page_save` directory.
- Displays a list of currently served files upon starting the server.
- Basic HTTP request handling and error responses.
- Demonstrates foundational networking principles (TCP communication, HTTP parsing).

---

## **Project Structure**

### Main Functionalities:
1. **Server Operations**
   - Listens on a specified port (`127.0.0.1:7878` by default).
   - Serves HTML files from the `page_save` directory.
   - Provides 404 error handling for non-existent files.

2. **File Management**
   - Supports uploading new files to the server.
   - Checks for duplicate filenames to prevent overwrites.

3. **HTTP Response Handling**
   - Sends appropriate HTTP response headers and body content.
   - Determines MIME type and file size for efficient serving.

---

## **Setup Instructions**

### Prerequisites:
- Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- Ensure the `page_save` directory exists in the project root.

### Clone the Repository:
```bash
git clone https://github.com/your-repo/single-threaded-web-server.git
cd single-threaded-web-server
```

### Run the Server:
```bash
cargo run
```

### Directory for HTML Files:
- Place all static HTML files you want to serve in the `page_save` folder.
- A default `hello.html` file is expected for the root request (`/`).
- Ensure a `404.html` file exists for handling invalid requests.

---

## **Usage Instructions**

1. **Start the Server**
   - Run the program and select `1` in the menu to start the server.
   - The server will list all files currently available in the `page_save` directory.

2. **Upload Files**
   - Select `2` in the menu to upload a file.
   - Provide the file name and file path as prompted.

3. **Access the Server**
   - Open a browser and go to `http://127.0.0.1:7878`.
   - Append the filename in the URL to access specific files (e.g., `http://127.0.0.1:7878/filename.html`).

4. **Error Handling**
   - Requests for non-existent files return a 404 error with the `404.html` page.

---

## **Key Technical Details**

- **Language:** Rust
- **Networking:** Uses the `std::net` module for TCP communication.
- **HTTP Parsing:** Processes basic HTTP request lines to identify the requested files.
- **File I/O:** Efficiently reads and writes files using Rust's `std::fs` module.
- **Memory Safety:** Ensures safe operations with Rust's ownership and error-handling features.

---

## **Future Enhancements**

- Add support for multi-threading to handle concurrent requests.
- Implement HTTPS for secure connections.
- Extend MIME type support for serving other static files (CSS, JS, images).
- Add logging for better monitoring and debugging.
- Create a more interactive and user-friendly front-end.

---

Thanks for Visiting my Repository
