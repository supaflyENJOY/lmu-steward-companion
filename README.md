# LMU Steward Companion

> **🤖 AI-Assisted Development**: This project was partly created with Large Language Models (LLMs). Have fun reading the code :D

A desktop application for analyzing Le Mans Ultimate (LMU) racing replays, providing comprehensive contact/collision analysis and export capabilities for racing stewards and organizers.

## 🏁 Overview

LMU Steward Companion is a **Tauri + SvelteKit** desktop application that helps racing stewards analyze incidents in Le Mans Ultimate replays. The application automatically discovers replay files, parses race results to identify driver contacts and collisions, and provides tools to export data for further analysis.

### Key Features

- **Automatic Replay Discovery**: Finds and lists all available LMU replay files
- **Contact/Collision Analysis**: Parses race results XML to identify driver incidents with precise timing
- **Replay Control**: Play specific replays and jump directly to contact moments  
- **Export Capabilities**: 
  - Excel export for contact data analysis
  - Google Sheets integration with OAuth2 authentication
- **Steward Tools**: Streamlined workflow for incident review and documentation

## 🚀 Quick Start

### Prerequisites

- **Node.js** (v18 or later)
- **Rust** (latest stable)
- **Le Mans Ultimate** installed via Steam

### Environment Setup

The application requires Google API credentials for Sheets integration. Create a `.env` file in the project root:

```env
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
```

### Installation & Development

```bash
# Clone the repository
git clone https://github.com/supaflyENJOY/lmu-steward-companion
cd lmu-steward-companion

# Install dependencies
npm install

# Start development server (includes frontend hot reload)
npm run tauri dev
```

### Building for Production

```bash
# Build the application
npm run tauri build
```

The built application will be available in `src-tauri/target/release/bundle/`.

## 🛠️ Development

### Project Structure

```
lmu-steward-companion/
├── src/                    # SvelteKit frontend
│   ├── routes/            # Application pages
│   │   ├── +page.svelte   # Main replay list
│   │   └── replay/[id]/   # Individual replay analysis
│   └── lib/components/    # Reusable UI components
├── src-tauri/             # Rust backend
│   └── src/
│       ├── lib.rs         # Core Tauri commands
│       ├── lmu_rest_api/  # LMU Watch API client
│       ├── lmu_results_parser/ # XML parsing & analysis
│       ├── google_auth/   # OAuth2 integration
│       └── lmu_file_system.rs # File discovery
└── static/                # Static assets
```

### Architecture

**Frontend (SvelteKit 5)**
- TypeScript support via JSConfig
- TailwindCSS 4.x for styling
- shadcn-svelte UI components
- Tauri API integration for backend communication

**Backend (Rust)**
- Tauri commands for frontend-backend communication
- XML parsing for race result analysis
- OAuth2 for Google Sheets authentication
- Excel file generation capabilities

### Available Scripts

**Frontend Development:**
```bash
npm run dev          # Start Vite development server
npm run build        # Build frontend for production
npm run preview      # Preview built frontend
npm run check        # Type checking and validation
npm run check:watch  # Watch mode for type checking
```

**Tauri Development:**
```bash
npm run tauri dev    # Run Tauri development mode
npm run tauri build  # Build Tauri application
npm run tauri [cmd]  # Access Tauri CLI directly
```

## 📋 Core Functionality

### Replay Analysis Workflow

1. **Discovery**: Application scans LMU installation for replay files
2. **Selection**: Choose replay from automatically generated list
3. **Analysis**: Parse race results XML to identify contacts/collisions
4. **Review**: Jump to specific contact moments in replay
5. **Export**: Generate reports for steward analysis

### Contact Detection Algorithm

The application uses time-based analysis to identify driver contacts:
- Analyzes race result XML data for proximity events
- 3-second threshold for grouping related incidents
- Tracks driver positions and timing for precise incident mapping

### Export Options

**Excel Export:**
- Comprehensive contact data with timing information
- Driver details and incident classifications
- Formatted for steward review workflows

**Google Sheets Integration:**
- OAuth2 authentication for secure access
- Direct upload to specified spreadsheets
- Real-time collaboration capabilities

## 🔧 Configuration

### Runtime Configuration

- OAuth2 tokens stored in application data directory
- Replay file paths automatically detected via Steam installation

### Build-time Configuration

Google API credentials are embedded during compilation using Rust's `env!()` macro. Ensure environment variables are set before building.

## 🧪 Testing

```bash
# Backend testing
cd src-tauri
cargo test

# Frontend validation
npm run check
```

## 📚 Dependencies

### Backend (Rust)
- **tauri** - Desktop application framework
- **reqwest** - HTTP client for API communication
- **quick-xml** - XML parsing for race results
- **rust_xlsxwriter** - Excel file generation
- **oauth2** - Google OAuth2 authentication
- **steamlocate** - Steam/LMU installation discovery

### Frontend (TypeScript/Svelte)
- **@sveltejs/kit** - SvelteKit framework
- **@tauri-apps/api** - Tauri frontend bindings
- **bits-ui + shadcn-svelte** - UI component library
- **tailwindcss** - Utility-first CSS framework

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

For support and questions:
- Check existing [Issues](https://github.com/supaflyENJOY/lmu-steward-companion/issues)
- Create a new issue with detailed information
- Include system information and error logs when reporting bugs

---

Built with ❤️ for the Le Mans Ultimate racing community