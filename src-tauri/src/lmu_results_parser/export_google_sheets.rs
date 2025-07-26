//! Export contacts to Google Sheets using Google Sheets API v4.

use crate::google_auth::GoogleAuth;
use crate::lmu_rest_api::types::Vehicle;
use crate::lmu_results_parser::types::Contact;
use serde_json::json;

/// Rulings and their license point values
const RULINGS: [(&str, i32); 11] = [
    ("NFA", 0),
    ("Racing Incident", 0),
    ("Warning", 1),
    ("TP +5s", 2),
    ("TP +10s", 2),
    ("TP +15s", 4),
    ("TP +20s", 4),
    ("TP +30s", 4),
    ("DT", 6),
    ("SG10", 6),
    ("SG30", 6),
];

/// Converts seconds (f32) to "hh:mm:ss" string.
fn seconds_to_hms(seconds: f32) -> String {
    let total_seconds = seconds.round() as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

/// Exports contacts to a new Google Sheet and returns the spreadsheet URL.
/// Returns Result<String, String> where Ok is the spreadsheet URL.
pub async fn export_contacts_to_google_sheets(
    contacts: &[Contact],
    standings: &[Vehicle],
    google_auth: &GoogleAuth,
) -> Result<String, String> {

    // 1. Get access token
    let access_token = google_auth.get_access_token().await
        .map_err(|e| format!("OAuth2 error: {e}"))?;

    // 2. Create spreadsheet
    let client = reqwest::Client::new();
    let spreadsheet_title = "LMU Steward Incidents";
    let create_sheet_body = json!({
        "properties": { "title": spreadsheet_title },
        "sheets": [
            { "properties": { "title": "Entrylist" } },
            { "properties": { "title": "Incidents" } },
            { "properties": { "title": "Rulings", "hidden": true } }
        ]
    });

    let resp = client
        .post("https://sheets.googleapis.com/v4/spreadsheets")
        .bearer_auth(&access_token)
        .json(&create_sheet_body)
        .send()
        .await
        .map_err(|e| {
            let error_msg = format!("Failed to create spreadsheet: {e}");
            error_msg
        })?;

    let status = resp.status();

    if !status.is_success() {
        let error_body = resp.text().await.unwrap_or_default();
        let error_msg = format!(
            "Google Sheets API error (status {}): {}",
            status, error_body
        );
        return Err(error_msg);
    }

    let resp_json: serde_json::Value = resp.json().await.map_err(|e| {
        let error_msg = format!("Failed to parse spreadsheet creation response: {e}");
        error_msg
    })?;


    let spreadsheet_id = resp_json
        .get("spreadsheetId")
        .and_then(|v| v.as_str())
        .ok_or("No spreadsheetId in response")?
        .to_string();

    // Extract actual sheet IDs from the response
    let sheets = resp_json
        .get("sheets")
        .and_then(|v| v.as_array())
        .ok_or("No sheets array in response")?;

    let entrylist_sheet_id = sheets
        .get(0)
        .and_then(|sheet| sheet.get("properties"))
        .and_then(|props| props.get("sheetId"))
        .and_then(|id| id.as_i64())
        .ok_or("Could not get entrylist sheet ID")? as i32;

    let incidents_sheet_id = sheets
        .get(1)
        .and_then(|sheet| sheet.get("properties"))
        .and_then(|props| props.get("sheetId"))
        .and_then(|id| id.as_i64())
        .ok_or("Could not get incidents sheet ID")? as i32;

    let rulings_sheet_id = sheets
        .get(2)
        .and_then(|sheet| sheet.get("properties"))
        .and_then(|props| props.get("sheetId"))
        .and_then(|id| id.as_i64())
        .ok_or("Could not get rulings sheet ID")? as i32;


    // 3. Prepare batchUpdate requests for formatting, data, validation, etc.
    let mut requests = vec![];

    // Write headers (bold)
    let headers = [
        "#",
        "Session Time",
        "Involved Players",
        "Ruling",
        "Responsible Car",
        "Description",
        "Resolved",
        "Applied",
    ];
    let header_row = headers.iter().map(|s| json!({ "userEnteredValue": { "stringValue": s }, "userEnteredFormat": { "textFormat": { "bold": true } } })).collect::<Vec<_>>();

    // Prepare data rows
    let mut data_rows = vec![header_row];
    for (i, contact) in contacts.iter().enumerate() {
        let players_str = contact
            .players
            .iter()
            .map(|id| {
                let vehicle = standings.iter().find(|v| v.slot_id == *id as i32);
                if let Some(vehicle) = vehicle {
                    vehicle.driver_name.clone()
                } else {
                    id.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");


        data_rows.push(vec![
            json!({ "userEnteredValue": { "numberValue": (i + 1) as f64 } }),
            json!({ "userEnteredValue": { "stringValue": seconds_to_hms(contact.et) } }),
            json!({ "userEnteredValue": { "stringValue": players_str }, "userEnteredFormat": { "wrapStrategy": "WRAP" } }),
            json!({}), // Ruling (empty, dropdown will be set)
            json!({}), // Responsible Car (empty)
            json!({}), // Description (empty)
            json!({ "userEnteredValue": { "boolValue": false }, "dataValidation": { "condition": { "type": "BOOLEAN" } } }), // Resolved checkbox
            json!({ "userEnteredValue": { "boolValue": false }, "dataValidation": { "condition": { "type": "BOOLEAN" } } }), // Applied checkbox
        ]);
    }


    // Set values for Incidents sheet
    requests.push(json!({
        "updateCells": {
            "rows": data_rows.into_iter().map(|row| json!({ "values": row })).collect::<Vec<_>>(),
            "fields": "*",
            "start": { "sheetId": incidents_sheet_id, "rowIndex": 0, "columnIndex": 0 }
        }
    }));

    // Set column widths and text wrapping
    requests.push(json!({
        "updateDimensionProperties": {
            "range": { "sheetId": incidents_sheet_id, "dimension": "COLUMNS", "startIndex": 2, "endIndex": 3 },
            "properties": { "pixelSize": 160 },
            "fields": "pixelSize"
        }
    }));
    requests.push(json!({
        "updateDimensionProperties": {
            "range": { "sheetId": incidents_sheet_id, "dimension": "COLUMNS", "startIndex": 3, "endIndex": 4 },
            "properties": { "pixelSize": 120 },
            "fields": "pixelSize"
        }
    }));
    requests.push(json!({
        "updateDimensionProperties": {
            "range": { "sheetId": incidents_sheet_id, "dimension": "COLUMNS", "startIndex": 4, "endIndex": 5 },
            "properties": { "pixelSize": 160 },
            "fields": "pixelSize"
        }
    }));
    requests.push(json!({
        "updateDimensionProperties": {
            "range": { "sheetId": incidents_sheet_id, "dimension": "COLUMNS", "startIndex": 5, "endIndex": 6 },
            "properties": { "pixelSize": 300 },
            "fields": "pixelSize"
        }
    }));
    requests.push(json!({
        "updateDimensionProperties": {
            "range": { "sheetId": entrylist_sheet_id, "dimension": "COLUMNS", "startIndex": 1, "endIndex": 2 },
            "properties": { "pixelSize": 160 },
            "fields": "pixelSize"
        }
    }));

    // Write RULINGS to hidden sheet (two columns: Ruling Name, License Points)
    let rulings_rows = RULINGS
        .iter()
        .map(|(r, pts)| {
            vec![
                json!({ "userEnteredValue": { "stringValue": *r } }),
                json!({ "userEnteredValue": { "numberValue": *pts as f64 } }),
            ]
        })
        .collect::<Vec<_>>();
    requests.push(json!({
        "updateCells": {
            "rows": rulings_rows.into_iter().map(|row| json!({ "values": row })).collect::<Vec<_>>(),
            "fields": "*",
            "start": { "sheetId": rulings_sheet_id, "rowIndex": 0, "columnIndex": 0 }
        }
    }));

    // Data validation dropdown for "Ruling" column (D)
    // Using correct Google Sheets API v4 format for ONE_OF_RANGE condition

    // Create the correct condition structure for Google Sheets API v4
    let correct_condition = json!({
        "type": "ONE_OF_RANGE",
        "values": [
            {
                "userEnteredValue": format!("=Rulings!A1:A{}", RULINGS.len())
            }
        ]
    });

    requests.push(json!({
        "setDataValidation": {
            "range": {
                "sheetId": incidents_sheet_id,
                "startRowIndex": 1,
                "endRowIndex": contacts.len() as i32 * 2,
                "startColumnIndex": 3,
                "endColumnIndex": 4
            },
            "rule": {
                "condition": correct_condition,
                "showCustomUi": true,
                "strict": true
            }
        }
    }));

    // Data validation dropdown for "Responsible Car" column (E)
    // Reference Entrylist!B2:B (Driver Name)
    let entrylist_driver_count = standings.len();
    let responsible_car_condition = json!({
        "type": "ONE_OF_RANGE",
        "values": [
            {
                "userEnteredValue": format!("=Entrylist!B2:B{}", entrylist_driver_count + 1)
            }
        ]
    });
    requests.push(json!({
        "setDataValidation": {
            "range": {
                "sheetId": incidents_sheet_id,
                "startRowIndex": 1,
                "endRowIndex": contacts.len() as i32 * 2,
                "startColumnIndex": 4,
                "endColumnIndex": 5
            },
            "rule": {
                "condition": responsible_car_condition,
                "showCustomUi": true,
                "strict": true
            }
        }
    }));

    // Checkbox for "Resolved" column (G)
    requests.push(json!({
        "setDataValidation": {
            "range": {
                "sheetId": incidents_sheet_id,
                "startRowIndex": 1,
                "endRowIndex": contacts.len() as i32 * 2,
                "startColumnIndex": 6,
                "endColumnIndex": 7
            },
            "rule": {
                "condition": { "type": "BOOLEAN" },
                "showCustomUi": true,
                "strict": true
            }
        }
    }));
    // Checkbox for "Applied" column (H)
    requests.push(json!({
        "setDataValidation": {
            "range": {
                "sheetId": incidents_sheet_id,
                "startRowIndex": 1,
                "endRowIndex": contacts.len() as i32 * 2,
                "startColumnIndex": 7,
                "endColumnIndex": 8
            },
            "rule": {
                "condition": { "type": "BOOLEAN" },
                "showCustomUi": true,
                "strict": true
            }
        }
    }));

    // Conditional formatting rules
    // Red for incomplete incidents (highest priority), Blue for "Applied" checked, Green for "Resolved" checked
    // All apply to the entire data range (excluding headers)
    let row_count = contacts.len() as i32;
    // Red: Responsible Car (E) not empty AND Description (C) empty
    requests.push(json!({
        "addConditionalFormatRule": {
            "rule": {
                "ranges": [{
                    "sheetId": incidents_sheet_id,
                    "startRowIndex": 1,
                    "endRowIndex": row_count * 2 + 1,
                    "startColumnIndex": 0,
                    "endColumnIndex": 8
                }],
                "booleanRule": {
                    "condition": {
                        "type": "CUSTOM_FORMULA",
                        "values": [{ "userEnteredValue": "=NOT(OR(AND($F2<>\"\", $E2<>\"\"), AND($F2=\"\", $E2=\"\")))" }]
                    },
                    "format": {
                        "backgroundColor": { "red": 1.0, "green": 0.78, "blue": 0.78 }
                    }
                }
            },
            "index": 0 // Highest priority
        }
    }));
    // Blue: Applied (H) is TRUE
    requests.push(json!({
        "addConditionalFormatRule": {
            "rule": {
                "ranges": [{
                    "sheetId": incidents_sheet_id,
                    "startRowIndex": 1,
                    "endRowIndex": row_count * 2 + 1,
                    "startColumnIndex": 0,
                    "endColumnIndex": 8
                }],
                "booleanRule": {
                    "condition": {
                        "type": "CUSTOM_FORMULA",
                        "values": [{ "userEnteredValue": "=$H2=TRUE" }]
                    },
                    "format": {
                        "backgroundColor": { "red": 0.7, "green": 0.85, "blue": 1.0 }
                    }
                }
            },
            "index": 1 // Second priority
        }
    }));
    // Green: Resolved (G) is TRUE
    requests.push(json!({
        "addConditionalFormatRule": {
            "rule": {
                "ranges": [{
                    "sheetId": incidents_sheet_id,
                    "startRowIndex": 1,
                    "endRowIndex": row_count * 2 + 1,
                    "startColumnIndex": 0,
                    "endColumnIndex": 8
                }],
                "booleanRule": {
                    "condition": {
                        "type": "CUSTOM_FORMULA",
                        "values": [{ "userEnteredValue": "=$G2=TRUE" }]
                    },
                    "format": {
                        "backgroundColor": { "red": 0.8, "green": 1.0, "blue": 0.8 }
                    }
                }
            },
            "index": 2 // Third priority
        }
    }));

    // --- Entrylist Sheet ---
    // Columns: Car Index, Driver Name, Car Number, License Points
    let entrylist_headers = ["Car Index", "Driver Name", "Car Number", "License Points"];
    let entrylist_header_row = entrylist_headers.iter().map(|s| json!({ "userEnteredValue": { "stringValue": s }, "userEnteredFormat": { "textFormat": { "bold": true } } })).collect::<Vec<_>>();

    let mut entrylist_rows = vec![entrylist_header_row];
    for (i, vehicle) in standings.iter().enumerate() {
        // License Points formula - Fixed to handle empty values and avoid #N/A errors
        let license_points_formula = format!(
            "=SUMPRODUCT((Incidents!E2:E{row_end}=B{row})*(Incidents!D2:D{row_end}<>\"\")*(IFERROR(VLOOKUP(Incidents!D2:D{row_end},Rulings!A:B,2,FALSE),0)))",
            row_end = contacts.len() * 2 + 1,
            row = i + 2
        );

        entrylist_rows.push(vec![
            json!({ "userEnteredValue": { "numberValue": vehicle.slot_id as f64 } }),
            json!({ "userEnteredValue": { "stringValue": vehicle.driver_name.clone() } }),
            json!({ "userEnteredValue": { "stringValue": vehicle.car_number.clone() } }),
            json!({ "userEnteredValue": { "formulaValue": license_points_formula } }),
        ]);
    }
    requests.push(json!({
        "updateCells": {
            "rows": entrylist_rows.into_iter().map(|row| json!({ "values": row })).collect::<Vec<_>>(),
            "fields": "*",
            "start": { "sheetId": entrylist_sheet_id, "rowIndex": 0, "columnIndex": 0 }
        }
    }));

    // Batch update
    let batch_update_url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate",
        spreadsheet_id
    );
    let batch_body = json!({ "requests": requests });


    // Log each request type for debugging
    for (_i, request) in requests.iter().enumerate() {
        if let Some(obj) = request.as_object() {
            let unknown = "unknown".to_string();
            let request_type = obj.keys().next().unwrap_or(&unknown);

            // Special logging for setDataValidation requests
            if request_type == "setDataValidation" {
                if let Some(_validation) = obj.get("setDataValidation") {
                }
            }
        }
    }


    let batch_resp = client
        .post(&batch_update_url)
        .bearer_auth(&access_token)
        .json(&batch_body)
        .send()
        .await
        .map_err(|e| format!("Failed to batchUpdate: {e}"))?;

    let batch_status = batch_resp.status();

    if !batch_status.is_success() {
        let err = batch_resp.text().await.unwrap_or_default();
        return Err(format!(
            "Google Sheets batchUpdate error (status {}): {}",
            batch_status, err
        ));
    }

    // Log successful batch response
    let _batch_response_text = batch_resp.text().await.unwrap_or_default();

    // Return spreadsheet URL
    let url = format!("https://docs.google.com/spreadsheets/d/{}", spreadsheet_id);
    Ok(url)
}
