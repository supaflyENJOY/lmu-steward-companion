# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Architecture Overview

This is a **Tauri + SvelteKit** desktop application for analyzing LMU racing replays. The application has a dual architecture:

### Rust Backend (`src-tauri/`)
- **Main entry point**: `src-tauri/src/main.rs` - simple wrapper that calls `lib.rs::run()`
- **Core functionality**: `src-tauri/src/lib.rs` - contains all Tauri commands and application logic
- **Key modules**:
  - `lmu_rest_api/` - Client for communicating with LMU Watch API (localhost REST endpoints)
  - `lmu_results_parser/` - Parses race result XML files and extracts contact/collision data
  - `google_auth/` - OAuth2 authentication for Google Sheets integration
  - `lmu_file_system.rs` - File system utilities for finding LMU installation

### Frontend (`src/`)
- **SvelteKit 5** with TypeScript support via JSConfig
- **UI Components**: Using shadcn-svelte components in `lib/components/ui/`
- **Styling**: TailwindCSS 4.x with custom configuration
- **Routes**: 
  - `/` - Main replay list page
  - `/replay/[id]` - Individual replay analysis page

## Key Functionality

The application provides:
1. **Replay Discovery**: Finds and lists LMU replay files
2. **Contact Analysis**: Parses race results to identify driver contacts/collisions
3. **Export Capabilities**: 
   - Excel export for contact data
   - Google Sheets integration with OAuth2 authentication
4. **Replay Control**: Can play specific replays and jump to contact moments

## Development Commands

### Frontend Development
```bash
# Start development server
npm run dev

# Build frontend
npm run build

# Preview built frontend
npm run preview

# Type checking and linting
npm run check
npm run check:watch
```

### Tauri Development
```bash
# Run Tauri development (includes frontend hot reload)
npm run tauri dev

# Build Tauri application
npm run tauri build

# Access Tauri CLI directly
npm run tauri [command]
```

## Environment Setup

### Required Environment Variables
The application requires Google API credentials at **build time**:
- `GOOGLE_CLIENT_ID` - Google OAuth2 client ID
- `GOOGLE_CLIENT_SECRET` - Google OAuth2 client secret

These are embedded into the binary during compilation using Rust's `env!()` macro (see `lib.rs:165-172`).

### Runtime Configuration
- OAuth2 tokens are stored in the application data directory
- The application expects LMU Watch API to be running on localhost (default configuration)

## Code Architecture Patterns

### Tauri Commands
All backend functionality is exposed through Tauri commands in `lib.rs`:
- `get_matched_replays()` - Retrieves list of available replays
- `play_replay(replay_idx)` - Starts replay playback
- `export_contacts_to_excel_command()` - Exports contact data to Excel
- `export_contacts_to_google_sheets_command()` - Exports to Google Sheets with OAuth2
- `get_contacts_for_replay()` - Gets contact data for frontend display
- `play_contact()` - Jumps to specific contact moment in replay

### Error Handling
- Backend uses `Result<T, String>` for Tauri command return types
- Comprehensive error logging with `[MODULE]` prefixes for debugging
- Frontend handles async loading states and error display

### Data Flow
1. Frontend calls Tauri commands using `@tauri-apps/api/core::invoke()`
2. Backend interacts with LMU Watch API (REST) and parses XML result files
3. Contact detection uses time-based analysis (3-second threshold for existing contacts)
4. Export functionality integrates with external services (Excel files, Google Sheets)

## Testing and Quality

The codebase uses:
- **Rust**: Standard `cargo test` for backend testing
- **Frontend**: SvelteKit's built-in type checking with `svelte-check`
- **Linting**: Use `npm run check` for frontend validation

## Key Dependencies

### Backend (Rust)
- `tauri` - Desktop application framework
- `reqwest` - HTTP client for API calls
- `quick-xml` - XML parsing for race results
- `rust_xlsxwriter` - Excel file generation
- `oauth2` - Google OAuth2 authentication
- `steamlocate` - Finding Steam/LMU installation

### Frontend (JavaScript/Svelte)
- `@sveltejs/kit` - SvelteKit framework
- `@tauri-apps/api` - Tauri frontend API bindings
- `bits-ui` + shadcn-svelte - UI component library
- `tailwindcss` - Utility-first CSS framework