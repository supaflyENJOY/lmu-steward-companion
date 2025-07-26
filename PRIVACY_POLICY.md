# Privacy Policy for LMU Steward Companion

**Effective Date:** July 26, 2025  
**Last Updated:** July 26, 2025

## Overview

LMU Steward Companion ("we," "our," or "the Application") is a desktop application designed to analyze Le Mans Ultimate (LMU) racing replays for stewards and racing organizers. This Privacy Policy explains how we collect, use, and protect your information when you use our application.

## Information We Collect

### Local Data Processing
- **Replay Files**: The application accesses LMU replay files stored locally on your computer for analysis purposes
- **Race Results**: XML files containing race data are parsed locally to identify contacts and collisions
- **Application Settings**: User preferences and configuration data stored locally on your device

### Google Services Integration
When you choose to use Google Sheets export functionality:
- **Google Account Information**: Basic profile information (email address) for authentication
- **Google Sheets Access**: Permission to create and modify spreadsheets in your Google Drive
- **OAuth2 Tokens**: Securely stored authentication tokens for Google API access

## How We Use Your Information

### Primary Functions
- **Replay Analysis**: Process local replay files to identify racing incidents
- **Data Export**: Generate Excel files and optionally export to Google Sheets
- **Application Operation**: Maintain user preferences and application state

### Google Sheets Integration
- **Authentication**: Verify your identity for secure Google API access
- **Data Export**: Upload racing incident data to spreadsheets you specify
- **Collaboration**: Enable sharing of analysis results with other stewards

## Data Storage and Security

### Local Storage
- All replay analysis is performed locally on your device
- No racing data is transmitted to external servers (except Google Sheets when explicitly requested)
- Application data stored in standard system directories with appropriate file permissions

### Google Services
- OAuth2 tokens stored securely using system keychain/credential manager
- All Google API communications use HTTPS encryption
- We only request minimum necessary permissions for Google Sheets functionality

## Data Sharing and Third Parties

### No Third-Party Sharing
- We do not sell, trade, or share your personal information with third parties
- Racing data remains on your local device unless you explicitly export to Google Sheets
- No analytics or tracking services are integrated into the application

### Google Services
- When using Google Sheets export, data is shared directly between your device and Google's services
- This sharing occurs only when you explicitly initiate an export operation
- We do not store or access your Google Sheets data beyond the export process

## Your Rights and Choices

### Data Control
- **Local Data**: You can delete all local application data by uninstalling the application
- **Google Access**: You can revoke Google API permissions at any time through your Google Account settings
- **Export Control**: All data exports to external services require your explicit consent

### Opt-Out Options
- Google Sheets integration is entirely optional
- You can use all core replay analysis features without connecting any external services
- Authentication tokens can be cleared through the application settings

## Children's Privacy

LMU Steward Companion is not intended for use by children under 13 years of age. We do not knowingly collect personal information from children under 13.

## Data Retention

### Local Data
- Application data is retained until you uninstall the application or manually delete files
- Temporary analysis files are automatically cleaned up after processing

### Google Authentication
- OAuth2 tokens are retained until you revoke access or uninstall the application
- No long-term storage of Google account information beyond authentication tokens

## Changes to This Policy

We may update this Privacy Policy from time to time. Any changes will be reflected in the "Last Updated" date above. Continued use of the application after changes indicates acceptance of the updated policy.

## Technical Implementation

### OAuth2 Compliance
- Implements standard OAuth2 authorization flow for Google API access
- Uses secure token storage mechanisms provided by the operating system
- Follows Google's API Terms of Service and security best practices

### Minimal Data Access
- Only requests essential Google Sheets permissions (read/write spreadsheets)
- No access to other Google services or personal data
- Transparent permission requests with clear explanations

## Contact Information

For questions about this Privacy Policy or our data practices, please contact:

**Project Repository**: [GitHub Issues](https://github.com/supaflyENJOY/lmu-steward-companion/issues)  
**Application Support**: Create an issue in the project repository with your privacy-related questions

## Compliance

This application is designed to comply with:
- Google API Services User Data Policy
- General Data Protection Regulation (GDPR) principles
- California Consumer Privacy Act (CCPA) where applicable

## Technical Details for Verification

### Google API Usage
- **Scopes Requested**: `https://www.googleapis.com/auth/spreadsheets`
- **Purpose**: Export racing incident analysis data to user-specified spreadsheets
- **Data Types**: Racing telemetry, driver information, incident timestamps (all derived from local replay files)
- **Storage**: No persistent storage of Google user data beyond OAuth2 tokens

### Application Architecture
- **Platform**: Desktop application using Tauri framework
- **Local Processing**: All analysis performed on user's device
- **Network Activity**: Limited to Google API calls when explicitly requested by user
- **Data Encryption**: All API communications use HTTPS/TLS encryption

---

*This privacy policy is specifically designed for Google Console project verification and compliance with Google API Services User Data Policy requirements.*