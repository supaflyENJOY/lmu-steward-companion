use crate::{lmu_rest_api::types::Vehicle, lmu_results_parser::types::Contact};
use rust_xlsxwriter::{Color, Format, Formula, Workbook, XlsxError};

/// Converts seconds (f32) to "hh:mm:ss" string.
fn seconds_to_hms(seconds: f32) -> String {
    let total_seconds = seconds.round() as u32;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

const RULINGS: [&str; 10] = [
    "NFA",
    "Racing Incident",
    "Warning",
    "TP +5s",
    "TP +10s",
    "TP +15s",
    "TP +20s",
    "TP +30s",
    "DT",
    "SG30",
];

/// Exports a list of contacts to an Excel file at the given path.
/// Columns: Number, Contact Time, Involved Players, Ruling, Responsible Car, Description
pub fn export_contacts_to_excel(
    contacts: &[Contact],
    standings: &[Vehicle],
    path: &str,
) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Incidents")?;

    // Header format (bold)
    let header_format = Format::new().set_bold();

    // Write header
    let headers = [
        "#",
        "Session Time",
        "Involved Players",
        "Ruling",
        "Responsible Car",
        "Description",
        "Resolved",
    ];
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, &header_format.clone())?;
    }

    // Write data rows
    for (i, contact) in contacts.iter().enumerate() {
        let row = (i + 1) as u32;
        worksheet.write_number(row, 0, (i + 1) as f64)?; // Number
        worksheet.write_string(row, 1, seconds_to_hms(contact.et))?; // Contact Time (session time)
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
        worksheet.write_string_with_format(row, 2, &players_str, &Format::new().set_text_wrap())?; // Involved Players
        worksheet.set_column_width(2, 30)?;
        worksheet.write_string(row, 3, "")?; // Ruling
        worksheet.set_column_width(3, 20)?;
        worksheet.write_string(row, 4, "")?; // Responsible Car
        worksheet.write_string(row, 5, "")?; // Description
        worksheet.set_column_width(5, 40)?;
        worksheet.insert_checkbox_with_format(
            row,
            6,
            false,
            &Format::new()
                .set_background_color(Color::Green)
                .set_checkbox(),
        )?;
    }

    // Add data validation for "Ruling" column (column 3, D) for all data rows
    use rust_xlsxwriter::DataValidation;
    // NOTE: rust_xlsxwriter does not currently support setting a dropdown (list) data validation from a range or explicit list.
    // The hidden "Options" sheet is created for future extensibility.
    // To enable the dropdown in Excel, open the file and set the data validation for the "Ruling" column to use =Options!$A$1:$A$2.
    // If a future version of rust_xlsxwriter adds this feature, update this section accordingly.

    let dv = DataValidation::new().allow_list_formula(Formula::from(
        format!("=Rulings!$A$1:$A${}", RULINGS.len()).as_str(),
    ));
    worksheet.add_data_validation(1, 3, contacts.len() as u32, 3, &dv)?;

    let options_sheet = workbook.add_worksheet();
    options_sheet.set_name("Rulings")?;
    for (i, ruling) in RULINGS.iter().enumerate() {
        options_sheet.write_string(i as u32, 0, *ruling)?;
    }
    options_sheet.set_hidden(true);

    workbook.save(path)
}
