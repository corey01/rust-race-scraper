use scraper::{ElementRef, Selector, Html};
use anyhow::{Result, Context}; // Optional but recommended

// Trait extension for HTML element processing
pub trait HtmlUtils {
    fn find_element<'a>(&'a self, selector: &str) -> Result<ElementRef<'a>>;
}

pub trait HtmlElementExt {
    fn direct_text(&self) -> String;
    fn attr(&self, name: &str) -> Option<String>;
}

impl HtmlElementExt for ElementRef<'_> {
    fn direct_text(&self) -> String {
        self.children()
            .filter_map(|node| node.value().as_text().map(|t| t.trim()))
            .collect::<Vec<&str>>()
            .join(" ")
            .trim()
            .to_string()
    }

    fn attr(&self, name: &str) -> Option<String> {
        self.value().attr(name).map(|s| s.to_string())
    }
}

pub fn find_element<'a>(html: &'a Html, selector: &str) -> Result<ElementRef<'a>> {
    let parsed_selector = Selector::parse(selector)
        .map_err(|e| anyhow::anyhow!("Invalid CSS selector '{}': {}", selector, e))?;
    
    html.select(&parsed_selector)
        .next()
        .context(format!("Element not found with selector '{}'", selector))
}

pub fn get_direct_text(element: &ElementRef, selector: &str) -> Option<String> {
    element.select(&Selector::parse(selector).ok()?)
        .next()
        .map(|e| e.direct_text())
        // .map(|potential_string| potential_string.trim().to_owned())
}

pub fn get_attribute(html: &Html, selector: &str, attr_name: &str) -> Result<String> {
    let element = find_element(html, selector)?;
    element.value()
        .attr(attr_name)
        .map(|s| s.to_string())
        .context(format!("Attribute '{}' not found on element matching '{}'", attr_name, selector))
}