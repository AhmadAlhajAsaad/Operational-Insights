# Equans Operational Insights

Repository voor het Equans Operational Insights project.
A full-stack web application for operational insights and analytics, built with a Rust backend (Axum) and React frontend (TypeScript + Vite).

---

## Doel

Een platform om gebruiks- en licentiegegevens uit o.a. Atlassian, GitHub en JFrog
te verzamelen en te presenteren in een professioneel dashboard, zodat SLS Digital
beter inzicht krijgt in kosten, usage en chargeback.

---

## 🎯 What This Program Does

This application consists of two parts:

1. **Backend (Server)**: A Rust application that runs on your computer and provides data through an API. It listens on `http://localhost:8080`.
2. **Frontend (User Interface)**: A React web application that displays information in your browser. It runs on `http://localhost:5174` (or similar port).

The frontend connects to the backend to fetch data and display it to the user.

---

## 🚀 Quick Start Options

### Option 1: Development Container (Recommended for Team Consistency) 

The easiest way to get started with a fully configured development environment:

**Using GitHub Codespaces:**
1. Click the green "Code" button on GitHub
2. Select "Codespaces" → "Create codespace"
3. Wait 3-5 minutes for the environment to build
4. Start coding with everything pre-installed!

**Using VS Code Dev Containers (Local):**
1. Install [Docker Desktop](https://www.docker.com/products/docker-desktop/) and [VS Code](https://code.visualstudio.com/)
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Clone this repository and open it in VS Code
4. Click "Reopen in Container" when prompted

✅ **What's included**: Rust, Node.js, PostgreSQL, GitHub Copilot, all linters pre-configured!  
📚 **Learn more**: See [.devcontainer/README.md](.devcontainer/README.md)

### Option 2: Manual Setup (Traditional Approach)

If you prefer installing tools directly on your machine:

---

## 📥 Prerequisites - Software You Need to Install

Before you can run this application, you need to install the following software on your computer:

### 1. **Rust** (for the backend)

Rust is a programming language used to build the backend server.

**Installation:**
1. Go to: https://www.rust-lang.org/tools/install
2. Download and run the installer for Windows
3. Follow the installation wizard (use default options)
4. After installation, **restart your computer** or open a new PowerShell window
5. Verify installation by opening PowerShell and typing:
   ```powershell
   rustc --version
   ```
   You should see something like: `rustc 1.xx.x`

### 2. **Node.js** (for the frontend)

Node.js allows you to run JavaScript applications and comes with npm (Node Package Manager).

**Installation:**
1. Go to: https://nodejs.org/
2. Download the **LTS (Long Term Support)** version for Windows
3. Run the installer and follow the wizard (use default options)
4. After installation, open a new PowerShell window
5. Verify installation:
   ```powershell
   node --version
   npm --version
   ```
   You should see version numbers for both

### 3. **Git** (optional but recommended)

Git helps you track changes to your code.

**Installation:**
1. Go to: https://git-scm.com/download/win
2. Download and run the installer
3. Use default options during installation
4. Verify:
   ```powershell
   git --version
   ```

### 4. **Docker Desktop** (optional - for database)

Only needed if you want to run a PostgreSQL database locally.

**Installation:**
1. Go to: https://www.docker.com/products/docker-desktop/
2. Download and install Docker Desktop for Windows
3. Start Docker Desktop after installation
4. Verify it's running (you'll see a whale icon in your system tray)

---

## 🚀 First Time Setup

After installing the prerequisites, follow these steps **once** to prepare the application:

### Step 1: Open PowerShell

1. Press `Windows Key + X`
2. Select "Windows PowerShell" or "Terminal"

### Step 2: Navigate to the Project Folder

```powershell
# Replace this path with where you extracted/cloned the project
cd C:\Users\YourUsername\Downloads\Equans-operational-insights
```

### Step 3: Install Frontend Dependencies

```powershell
# Navigate to the frontend folder
cd frontend

# Install all required packages (this takes a few minutes)
npm install

# Go back to the project root
cd ..
```

### Step 4: Build the Backend (First Time)

```powershell
# Navigate to the backend folder
cd backend

# Build the Rust application (this takes several minutes the first time)
cargo build

# Go back to the project root
cd ..
```

**Note:** The first time you build the backend, it will download and compile many dependencies. This is normal and can take 5-10 minutes.

---

## ▶️ How to Run the Application

You need to run both the backend and frontend. Open **TWO** PowerShell windows:

### PowerShell Window 1: Start the Backend

```powershell
# Navigate to the project folder
cd C:\Users\YourUsername\Downloads\Equans-operational-insights

# Navigate to backend folder
cd backend

# Run the backend server
cargo run
```

**Expected output:**
```
Backend is running on http://localhost:8080
```

**Keep this window open!** The backend needs to keep running.

### PowerShell Window 2: Start the Frontend

```powershell
# Navigate to the project folder
cd C:\Users\YourUsername\Downloads\Equans-operational-insights

# Navigate to frontend folder
cd frontend

# Start the frontend development server
npm start
```

**Expected output:**
```
VITE v7.2.6  ready in 275 ms

➜  Local:   http://localhost:5174/
```

**Keep this window open!** The frontend needs to keep running.

### Step 3: Open Your Browser

1. Open your web browser (Chrome, Edge, Firefox, etc.)
2. Go to: `http://localhost:5174` (or the port shown in your terminal)
3. You should see the Equans Operational Insights Dashboard
4. If the backend is running correctly, you'll see a green "✅ Backend connected" message

---

## 🛑 How to Stop the Application

To stop the application:

1. Go to each PowerShell window where the application is running
2. Press `Ctrl + C`
3. Wait for the process to stop (you'll see the command prompt return)

You can close the PowerShell windows after stopping the processes.
---

## ✏️ Making Changes to the Code

### Changing Backend Code (Rust)

1. Open files in `backend/src/` with a text editor (VS Code, Notepad++, etc.)
2. Make your changes
3. **Save the file**
4. Stop the backend (Ctrl+C in the backend terminal)
5. Run `cargo run` again to restart with your changes

**Example: Changing the port**

Open `backend/src/main.rs` and find this line:
```rust
let port: u16 = std::env::var("BACKEND_PORT")
    .unwrap_or_else(|_| "8080".into())
```

Change `"8080"` to another port number, like `"9000"`:
```rust
let port: u16 = std::env::var("BACKEND_PORT")
    .unwrap_or_else(|_| "9000".into())
```

### Changing Frontend Code (React/TypeScript)

1. Open files in `frontend/src/` with a text editor
2. Make your changes
3. **Save the file**
4. The frontend will **automatically reload** in your browser (hot reload)
5. No need to restart!

**Example: Changing the title**

Open `frontend/src/App.tsx` and find:
```tsx
<h1>Equans Operational Insights Dashboard</h1>
```

Change it to:
```tsx
<h1>My Custom Dashboard</h1>
```

Save the file and your browser will automatically update!

### Installing New Dependencies

**Backend (Rust):**
1. Open `backend/Cargo.toml`
2. Add the dependency under `[dependencies]`, example:
   ```toml
   serde = { version = "1", features = ["derive"] }
   ```
3. Run `cargo build` to download the new dependency

**Frontend (Node.js):**
1. Open PowerShell in the `frontend` folder
2. Run:
   ```powershell
   npm install package-name
   ```
   Example: `npm install axios`

---

## 📚 Additional Information

### Useful Commands

**Backend:**
```powershell
cargo run          # Run the application
cargo build        # Compile the application
cargo test         # Run tests
cargo clean        # Clean build artifacts (fixes some errors)
```

**Frontend:**
```powershell
npm start          # Start development server
npm run build      # Build for production
npm run lint       # Check code quality
npm install        # Install dependencies
```

## 🧹 Cleaning Files Before Uploading to GitHub

Before uploading your code to GitHub, you need to clean up generated files and folders that shouldn't be shared.

### Why Clean Up?

Generated files like compiled code, dependencies, and logs:
- Make your repository very large (hundreds of MB or GB)
- Are unnecessary (others can regenerate them)
- Can contain sensitive information
- Make it harder to see actual code changes

### What Gets Automatically Ignored

The `.gitignore` file at the root of this project automatically excludes:

**Backend (Rust):**
- `backend/target/` - All compiled code (can be 100+ MB)
- `backend/Cargo.lock` - Dependency lock file (regenerated automatically)
- `.env` files - May contain passwords/secrets

**Frontend (Node.js):**
- `frontend/node_modules/` - All npm packages (can be 200+ MB)
- `frontend/dist/` - Built/compiled frontend files
- `package-lock.json` - Dependency lock file

**General:**
- Log files (`*.log`)
- OS files (`.DS_Store`, `Thumbs.db`)
- IDE files (`.vscode/`, `.idea/`)
- Temporary files

### ⚡ Automated Cleanup (Recommended)

We've created automated cleanup scripts that do everything for you with one command!

**PowerShell Script**

```powershell
# Navigate to project root
cd C:\Users\YourUsername\Downloads\Equans-operational-insights

# Run the cleanup script
.\cleanup.ps1
```

**After running the script, you'll see:**
```
✨ Cleanup complete!

Your project is now ready for Git.
You can now run:
  git add .
  git commit -m "Your commit message"
  git push
```
