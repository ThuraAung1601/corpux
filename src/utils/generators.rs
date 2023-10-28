use std::fs::File;
use std::io::Write;

use crate::Counts;
use crate::LangInfo;

pub fn generate_html_table(count: &[Counts], file_names: &[String]) -> String {
    // HTML table generation using HTML string
    let mut html: String = String::new();

    html.push_str("<h3>Data statistics of the text file(s)</h3>");

    html.push_str("
    <style>
    table {
        border: 1px solid black;
        border-collapse: collapse;
    }
    th, td {
      border: 1px solid black;
      padding: 8px;
    }
    </style>");

    // table start
    html.push_str("
    <table>
    ");

    // table head
    html.push_str("
    <tr class=\"bordered-table\">
    <th style=\"text-align:center\">Counts</th>
    ");

    for _i in 0..file_names.len() {
        let text = format!("
        <th style=\"text-align:center\">{}</th>", file_names[_i]);
        html.push_str(&text);
    }

    html.push_str("</tr>");

    html.push_str("
    <tr>
    <td style=\"text-align:center\">Word Count</td>
    ");

    for c in count {
        let text = format!("
        <td style=\"text-align:right\">{}</td>", c.word_count);
        html.push_str(&text);
    }
    
    html.push_str("</tr>");

    html.push_str("
    <tr>
    <td style=\"text-align:center\">Unique Words</td>
    ");

    for c in count {
        let text = format!("
        <td style=\"text-align:right\">{}</td>", c.unique_word_count);
        html.push_str(&text);
    }
    html.push_str("</tr>");

    html.push_str("
    <tr>
    <td style=\"text-align:center\">Line Count</td>
    ");

    for c in count {
        let text = format!("
        <td style=\"text-align:right\">{}</td>", c.line_count);
        html.push_str(&text);
    }

    html.push_str("</tr>");

    // table end
    html.push_str("
    </table>
    ");

    html
}

fn generate_svg(lang: &[String], char_vec: &[usize], percent_vec: &[f64]) -> String {
    // SVG generation for language distribution bar graph
    
    // Initialize an empty SVG string
    let mut svg = String::new();

    // Define the chart dimensions
    let width = 1400;
    let height = 150;

    // Define the data and labels for the bar chart
    let data = char_vec.to_vec();
    let labels = lang.to_vec();
    let percent_labels = percent_vec.to_vec();



    // Calculate the maximum value for scaling
    let max_value = *data.iter().max().unwrap_or(&0);

    // Define the bar width and spacing
    let bar_width = width as f64 / ((data.len() as f64)*2.0);
    let bar_spacing = 5.0;

    // Start the SVG document
    svg.push_str(&format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">",
        width, height
    ));

    // Draw the bars and labels
    let mut x = bar_spacing;
    let y_scale = height as f64 / max_value as f64;

    for ((value, label),percent) in data.iter().zip(labels.iter()).zip(percent_labels) {
        let bar_height = (*value as f64 * y_scale * 0.5) as i32;
        let bar_x = x as i32;

        // Draw the bar
        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"yellow\" />",
            bar_x,
            height - bar_height,
            bar_width as i32,
            bar_height
        ));

        // Add the label under the bar
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
            bar_x + bar_width as i32 / 2,
            height - bar_height-30,
            label
        ));

        // Add the percent label under the bar
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\">{:.2}%</text>",
            bar_x + bar_width as i32 / 2,
            height - bar_height-10,
            percent
        ));

        // Increment x position for the next bar
        x += bar_width + bar_spacing;
    }

    // Close the SVG document
    svg.push_str("</svg>");

    svg
}

pub fn generate_many_svgs(lang_infos: Vec<Vec<LangInfo>>, file_names: Vec<String>) {
    // Generate svg files for each text file under the folder
    
    for (lang_ls, file_name) in lang_infos.iter().zip(file_names.iter()) {
        // Separate lang, total char and percentage vectors
        let lang_vec: Vec<String> = lang_ls.iter().map(|info| info.lang.clone()).collect();
        let char_vec: Vec<usize> = lang_ls.iter().map(|info| info.total_character).collect();
        let percent_vec: Vec<f64> = lang_ls.iter().map(|info| info.percentage).collect();

        // Generate the SVG content
        let svg = generate_svg(&lang_vec, &char_vec, &percent_vec);

        let file = format!("{}.svg", file_name);
        let mut output_file = File::create(file).expect("Failed to create output file");
        write!(output_file, "{}", svg).expect("Failed to write to output file");
    }
}
