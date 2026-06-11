use crate::converter::ConvertedDocument;
use crate::token_estimator::estimate_tokens;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Markdown,
    PlainText,
    PromptWrappedMarkdown,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputStyle {
    Clean,
    Compact,
    Detailed,
}

pub fn format_document(
    document: &ConvertedDocument,
    output_format: OutputFormat,
    output_style: OutputStyle,
) -> String {
    match output_format {
        OutputFormat::Markdown => format_markdown(document, output_style),
        OutputFormat::PlainText => format_plain_text(document, output_style),
        OutputFormat::PromptWrappedMarkdown => {
            format_prompt_wrapped_markdown(document, output_style)
        }
    }
}

fn format_markdown(document: &ConvertedDocument, output_style: OutputStyle) -> String {
    match output_style {
        OutputStyle::Compact => document.content.trim().to_string(),

        OutputStyle::Clean => {
            format!(
                "# Extracted from: {}\n\n{}",
                document.file_name,
                document.content.trim()
            )
        }

        OutputStyle::Detailed => {
            let estimated_tokens = estimate_tokens(&document.content);

            format!(
                "# Extracted from: {}\n\n- File type: {}\n- Estimated tokens: ~{}\n\n---\n\n{}",
                document.file_name,
                document.file_extension.as_deref().unwrap_or("unknown"),
                estimated_tokens,
                document.content.trim()
            )
        }
    }
}

fn format_plain_text(document: &ConvertedDocument, output_style: OutputStyle) -> String {
    match output_style {
        OutputStyle::Compact => document.content.trim().to_string(),

        OutputStyle::Clean => {
            format!(
                "Extracted from: {}\n\n{}",
                document.file_name,
                document.content.trim()
            )
        }

        OutputStyle::Detailed => {
            let estimated_tokens = estimate_tokens(&document.content);

            format!(
                "Extracted from: {}\nFile type: {}\nEstimated tokens: ~{}\n\n{}",
                document.file_name,
                document.file_extension.as_deref().unwrap_or("unknown"),
                estimated_tokens,
                document.content.trim()
            )
        }
    }
}

fn format_prompt_wrapped_markdown(
    document: &ConvertedDocument,
    output_style: OutputStyle,
) -> String {
    let formatted = format_markdown(document, output_style);

    format!(
        "I extracted the following content from a file. Please use it as context.\n\n<document>\n{}\n</document>",
        formatted.trim()
    )
}
