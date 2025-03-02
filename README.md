# 🎨 **Artfolio** – A Modern Art Portfolio  

Artfolio is a sleek, responsive portfolio for artists, built with **Angular** and **Rust**. It features a dynamic gallery, **automatic watermarking**, and a **fast, secure backend**. Optimized for desktop & mobile, with cloud storage support for seamless hosting.  

🚀 **Tech Stack:** Angular • Rust (Rocket/Axum) • PostgreSQL • Cloud Storage  

---  

## 🛠 **Installation & Setup**  

### **Prerequisites**  
Ensure you have the following installed:  
- **Node.js** (LTS) & npm  
- **Angular CLI** (`npm install -g @angular/cli`)  
- **Rust** (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)  
- **PostgreSQL** (or any preferred database)  

### **Clone the Repository**  
```sh  
git clone https://github.com/yourusername/artfolio.git  
cd artfolio  
```

### **Backend Setup (Rust)**  
1. Navigate to the backend directory:  
   ```sh  
   cd backend  
   ```  
2. Create a `.env` file with your database credentials:  
   ```sh  
   DATABASE_URL=postgres://user:password@localhost/artfolio  
   ```  
3. Install dependencies and run the server:  
   ```sh  
   cargo run  
   ```  

### **Frontend Setup (Angular)**  
1. Navigate to the frontend directory:  
   ```sh  
   cd frontend  
   ```  
2. Install dependencies:  
   ```sh  
   npm install  
   ```  
3. Start the development server:  
   ```sh  
   ng serve  
   ```  
4. Open **http://localhost:4200** in your browser.  

---  

## 📸 **Features**  
- 🖼 **Dynamic Art Gallery** – Showcase and organize artwork beautifully  
- 🔒 **Automatic Watermarking** – Protect images with embedded watermarks  
- ⚡ **Fast & Secure Rust Backend** – Handles image processing efficiently  
- 📱 **Fully Responsive** – Optimized for all devices  
- ☁️ **Cloud Storage Support** – Store images securely  
