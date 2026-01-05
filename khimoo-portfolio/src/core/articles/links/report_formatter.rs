use anyhow::{Context, Result};

use super::{ValidationReport, ValidationError, ValidationWarning, ValidationErrorType, ValidationWarningType};

/// Report formatter for validation results
/// Provides multiple output formats for validation reports
pub struct ValidationReportFormatter;

impl ValidationReportFormatter {
    /// Format validation report as JSON
    pub fn format_json(report: &ValidationReport) -> Result<String> {
        serde_json::to_string_pretty(report)
            .context("Failed to serialize validation report to JSON")
    }

    /// Format validation report for console output
    pub fn format_console(report: &ValidationReport) -> String {
        let mut output = String::new();

        // Header
        output.push_str("🔍 Link Validation Report\n");
        output.push_str(&format!("📅 Generated: {}\n\n", report.validation_date));

        // Summary
        output.push_str("📊 Summary:\n");
        output.push_str(&format!("   📚 Total articles: {}\n", report.summary.total_articles));
        output.push_str(&format!("   🔗 Total links: {}\n", report.summary.total_links));

        if report.summary.broken_links > 0 {
            output.push_str(&format!("   ❌ Broken links: {}\n", report.summary.broken_links));
        } else {
            output.push_str("   ✅ All links valid\n");
        }

        if report.summary.invalid_references > 0 {
            output.push_str(&format!("   ⚠️  Invalid references: {}\n", report.summary.invalid_references));
        }

        if report.summary.orphaned_articles > 0 {
            output.push_str(&format!("   🏝️  Orphaned articles: {}\n", report.summary.orphaned_articles));
        }

        output.push_str(&format!("   📄 Articles with errors: {}\n", report.summary.articles_with_errors));
        output.push_str(&format!("   ⚠️  Articles with warnings: {}\n", report.summary.articles_with_warnings));

        // Errors section
        if !report.errors.is_empty() {
            output.push_str("\n❌ Errors:\n");
            for (i, error) in report.errors.iter().enumerate() {
                output.push_str(&format!("{}. ", i + 1));
                output.push_str(&Self::format_error(error));
                output.push('\n');
            }
        }

        // Warnings section
        if !report.warnings.is_empty() {
            output.push_str("\n⚠️  Warnings:\n");
            for (i, warning) in report.warnings.iter().enumerate() {
                output.push_str(&format!("{}. ", i + 1));
                output.push_str(&Self::format_warning(warning));
                output.push('\n');
            }
        }

        // Article statistics (top problematic articles)
        if !report.article_stats.is_empty() {
            output.push_str("\n📈 Article Statistics:\n");

            // Find articles with most issues
            let mut articles_with_issues: Vec<_> = report.article_stats
                .iter()
                .filter(|(_, stats)| stats.has_errors || stats.has_warnings)
                .collect();

            articles_with_issues.sort_by(|a, b| {
                let a_issues = a.1.broken_outbound_links + a.1.invalid_related_articles;
                let b_issues = b.1.broken_outbound_links + b.1.invalid_related_articles;
                b_issues.cmp(&a_issues)
            });

            for (slug, stats) in articles_with_issues.iter().take(10) {
                output.push_str(&format!("   📄 {}: ", slug));

                let mut issue_parts = Vec::new();
                if stats.broken_outbound_links > 0 {
                    issue_parts.push(format!("{} broken links", stats.broken_outbound_links));
                }
                if stats.invalid_related_articles > 0 {
                    issue_parts.push(format!("{} invalid references", stats.invalid_related_articles));
                }

                if !issue_parts.is_empty() {
                    output.push_str(&issue_parts.join(", "));
                } else if stats.has_warnings {
                    output.push_str("warnings only");
                }

                output.push_str(&format!(" ({} out, {} in)", stats.outbound_links, stats.inbound_links));
                output.push('\n');
            }
        }

        // Footer with recommendations
        if report.summary.broken_links > 0 || report.summary.invalid_references > 0 {
            output.push_str("\n💡 Recommendations:\n");
            output.push_str("   • Fix broken links by updating article references\n");
            output.push_str("   • Remove invalid entries from related_articles in front matter\n");
            output.push_str("   • Consider creating missing articles if they are frequently referenced\n");
        }

        if report.summary.orphaned_articles > 0 {
            output.push_str("   • Add links to/from orphaned articles to improve connectivity\n");
        }

        output
    }

    /// Format a single validation error
    pub fn format_error(error: &ValidationError) -> String {
        let error_type_str = match error.error_type {
            ValidationErrorType::BrokenLink => "🔗 Broken Link",
            ValidationErrorType::InvalidRelatedArticle => "📋 Invalid Related Article",
            ValidationErrorType::MissingMetadata => "📝 Missing Metadata",
            ValidationErrorType::InvalidMetadata => "❌ Invalid Metadata",
            ValidationErrorType::CircularReference => "🔄 Circular Reference",
            ValidationErrorType::OrphanedArticle => "🏝️  Orphaned Article",
        };

        let mut formatted = format!("{}: {} → {}",
            error_type_str,
            error.source_article,
            error.target_reference
        );

        if let Some(context) = &error.context {
            formatted.push_str(&format!(" ({})", context));
        }

        if let Some(suggestion) = &error.suggestion {
            formatted.push_str(&format!(" | 💡 {}", suggestion));
        }

        formatted
    }

    /// Format a single validation warning
    pub fn format_warning(warning: &ValidationWarning) -> String {
        let warning_type_str = match warning.warning_type {
            ValidationWarningType::UnusedTag => "🏷️  Unused Tag",
            ValidationWarningType::LowImportanceWithManyLinks => "📈 Low Importance, Many Links",
            ValidationWarningType::HighImportanceWithFewLinks => "📉 High Importance, Few Links",
            ValidationWarningType::MissingBacklinks => "🔗 Missing Backlinks",
            ValidationWarningType::InconsistentCasing => "🔤 Inconsistent Casing",
        };

        let mut formatted = format!("{}: {}", warning_type_str, warning.source_article);

        if let Some(target) = &warning.target_reference {
            formatted.push_str(&format!(" → {}", target));
        }

        if let Some(context) = &warning.context {
            formatted.push_str(&format!(" ({})", context));
        }

        if let Some(suggestion) = &warning.suggestion {
            formatted.push_str(&format!(" | 💡 {}", suggestion));
        }

        formatted
    }

    /// Generate a compact summary for CI/CD environments
    pub fn format_ci_summary(report: &ValidationReport) -> String {
        let mut output = String::new();

        if report.summary.broken_links == 0 && report.summary.invalid_references == 0 {
            output.push_str("✅ All links valid");
        } else {
            output.push_str("❌ Validation failed:");
            if report.summary.broken_links > 0 {
                output.push_str(&format!(" {} broken links", report.summary.broken_links));
            }
            if report.summary.invalid_references > 0 {
                output.push_str(&format!(" {} invalid references", report.summary.invalid_references));
            }
        }

        if report.summary.articles_with_warnings > 0 {
            output.push_str(&format!(" ({} warnings)", report.summary.articles_with_warnings));
        }

        output
    }

    /// Generate markdown format report
    pub fn format_markdown(report: &ValidationReport) -> String {
        let mut output = String::new();

        // Header
        output.push_str("# Link Validation Report\n\n");
        output.push_str(&format!("**Generated:** {}\n\n", report.validation_date));

        // Summary
        output.push_str("## Summary\n\n");
        output.push_str(&format!("- **Total articles:** {}\n", report.summary.total_articles));
        output.push_str(&format!("- **Total links:** {}\n", report.summary.total_links));
        output.push_str(&format!("- **Broken links:** {}\n", report.summary.broken_links));
        output.push_str(&format!("- **Invalid references:** {}\n", report.summary.invalid_references));
        output.push_str(&format!("- **Orphaned articles:** {}\n", report.summary.orphaned_articles));
        output.push_str(&format!("- **Articles with errors:** {}\n", report.summary.articles_with_errors));
        output.push_str(&format!("- **Articles with warnings:** {}\n\n", report.summary.articles_with_warnings));

        // Errors section
        if !report.errors.is_empty() {
            output.push_str("## Errors\n\n");
            for error in &report.errors {
                output.push_str(&format!("- **{}**: {} → {}", 
                    Self::error_type_to_string(&error.error_type),
                    error.source_article,
                    error.target_reference
                ));
                
                if let Some(context) = &error.context {
                    output.push_str(&format!(" ({})", context));
                }
                
                if let Some(suggestion) = &error.suggestion {
                    output.push_str(&format!("\n  - *Suggestion: {}*", suggestion));
                }
                
                output.push_str("\n");
            }
            output.push('\n');
        }

        // Warnings section
        if !report.warnings.is_empty() {
            output.push_str("## Warnings\n\n");
            for warning in &report.warnings {
                output.push_str(&format!("- **{}**: {}", 
                    Self::warning_type_to_string(&warning.warning_type),
                    warning.source_article
                ));
                
                if let Some(target) = &warning.target_reference {
                    output.push_str(&format!(" → {}", target));
                }
                
                if let Some(context) = &warning.context {
                    output.push_str(&format!(" ({})", context));
                }
                
                if let Some(suggestion) = &warning.suggestion {
                    output.push_str(&format!("\n  - *Suggestion: {}*", suggestion));
                }
                
                output.push_str("\n");
            }
        }

        output
    }

    /// Write validation report to files
    pub fn write_report_files(report: &ValidationReport, output_dir: &std::path::Path) -> Result<()> {
        // Ensure output directory exists
        std::fs::create_dir_all(output_dir)
            .context("Failed to create output directory")?;

        // Write JSON report
        let json_path = output_dir.join("validation-report.json");
        let json_content = Self::format_json(report)?;
        std::fs::write(&json_path, json_content)
            .with_context(|| format!("Failed to write JSON report to {:?}", json_path))?;

        // Write console report
        let console_path = output_dir.join("validation-report.txt");
        let console_content = Self::format_console(report);
        std::fs::write(&console_path, console_content)
            .with_context(|| format!("Failed to write console report to {:?}", console_path))?;

        // Write markdown report
        let markdown_path = output_dir.join("validation-report.md");
        let markdown_content = Self::format_markdown(report);
        std::fs::write(&markdown_path, markdown_content)
            .with_context(|| format!("Failed to write markdown report to {:?}", markdown_path))?;

        // Write CI summary
        let ci_path = output_dir.join("validation-summary.txt");
        let ci_content = Self::format_ci_summary(report);
        std::fs::write(&ci_path, ci_content)
            .with_context(|| format!("Failed to write CI summary to {:?}", ci_path))?;

        Ok(())
    }

    fn error_type_to_string(error_type: &ValidationErrorType) -> &'static str {
        match error_type {
            ValidationErrorType::BrokenLink => "Broken Link",
            ValidationErrorType::InvalidRelatedArticle => "Invalid Related Article",
            ValidationErrorType::MissingMetadata => "Missing Metadata",
            ValidationErrorType::InvalidMetadata => "Invalid Metadata",
            ValidationErrorType::CircularReference => "Circular Reference",
            ValidationErrorType::OrphanedArticle => "Orphaned Article",
        }
    }

    fn warning_type_to_string(warning_type: &ValidationWarningType) -> &'static str {
        match warning_type {
            ValidationWarningType::UnusedTag => "Unused Tag",
            ValidationWarningType::LowImportanceWithManyLinks => "Low Importance, Many Links",
            ValidationWarningType::HighImportanceWithFewLinks => "High Importance, Few Links",
            ValidationWarningType::MissingBacklinks => "Missing Backlinks",
            ValidationWarningType::InconsistentCasing => "Inconsistent Casing",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::articles::links::{ValidationSummary, ValidationError, ValidationErrorType};

    #[test]
    fn test_format_json() {
        let report = ValidationReport {
            validation_date: "2024-01-01T00:00:00Z".to_string(),
            summary: ValidationSummary {
                total_articles: 1,
                total_links: 1,
                broken_links: 0,
                invalid_references: 0,
                orphaned_articles: 0,
                circular_references: 0,
                articles_with_errors: 0,
                articles_with_warnings: 0,
            },
            errors: Vec::new(),
            warnings: Vec::new(),
            article_stats: std::collections::HashMap::new(),
        };

        let json = ValidationReportFormatter::format_json(&report).unwrap();
        assert!(json.contains("validation_date"));
        assert!(json.contains("2024-01-01T00:00:00Z"));
    }

    #[test]
    fn test_format_error() {
        let error = ValidationError {
            error_type: ValidationErrorType::BrokenLink,
            source_article: "test-article".to_string(),
            target_reference: "missing-article".to_string(),
            context: Some("wiki link".to_string()),
            line_number: None,
            suggestion: Some("Did you mean 'existing-article'?".to_string()),
        };

        let formatted = ValidationReportFormatter::format_error(&error);
        assert!(formatted.contains("Broken Link"));
        assert!(formatted.contains("test-article"));
        assert!(formatted.contains("missing-article"));
        assert!(formatted.contains("wiki link"));
        assert!(formatted.contains("Did you mean"));
    }

    #[test]
    fn test_format_ci_summary() {
        let report = ValidationReport {
            validation_date: "2024-01-01T00:00:00Z".to_string(),
            summary: ValidationSummary {
                total_articles: 1,
                total_links: 1,
                broken_links: 0,
                invalid_references: 0,
                orphaned_articles: 0,
                circular_references: 0,
                articles_with_errors: 0,
                articles_with_warnings: 1,
            },
            errors: Vec::new(),
            warnings: Vec::new(),
            article_stats: std::collections::HashMap::new(),
        };

        let summary = ValidationReportFormatter::format_ci_summary(&report);
        assert!(summary.contains("✅ All links valid"));
        assert!(summary.contains("(1 warnings)"));
    }
}