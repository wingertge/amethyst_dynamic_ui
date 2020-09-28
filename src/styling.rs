use crate::HashMap;
use amethyst::{
    assets::AssetStorage,
    core::ecs::{Component, DenseVecStorage},
    renderer::palette::Srgba,
    ui::{FontAsset, FontHandle, TextSection}
};
use derivative::Derivative;
use glyph_brush::rusttype::Scale;
use itertools::Itertools;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, num::ParseIntError, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Style {
    pub start_tags: String,
    pub end_tags: String
}

#[derive(Default, Clone, Debug)]
pub struct Styles(HashMap<String, Style>);

impl Styles {
    pub fn add_stylesheet(&mut self, styles: HashMap<String, Style>) {
        self.0.extend(styles);
    }
}

pub struct FontFamily {
    pub regular: FontHandle,
    pub bold: Option<FontHandle>,
    pub italic: Option<FontHandle>,
    pub bold_italic: Option<FontHandle>
}

pub struct HtmlText {
    font: FontFamily,
    default_color: [f32; 4],
    default_font_size: f32
}

impl Component for HtmlText {
    type Storage = DenseVecStorage<Self>;
}

impl HtmlText {
    pub fn new(font: FontFamily, color: [f32; 4], font_size: f32) -> Self {
        Self {
            font,
            default_color: color,
            default_font_size: font_size
        }
    }

    pub fn parse(
        &self,
        text: &str,
        styles: &Styles,
        fonts: &AssetStorage<FontAsset>
    ) -> Vec<TextSection> {
        parse_html(
            text,
            self.default_color,
            self.default_font_size,
            &self.font,
            &styles.0,
            fonts
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CachedGlyph {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) advance_width: f32
}

#[derive(Derivative)]
#[derivative(Debug)]
struct StyleStack<'a> {
    color: VecDeque<[f32; 4]>,
    font_size: VecDeque<f32>,
    #[derivative(Debug = "ignore")]
    font: VecDeque<FontHandle>,
    text: Option<&'a str>,
    font_style: FontStyle,
    small_caps: bool
}

impl StyleStack<'_> {
    fn new(color: [f32; 4], font_size: f32, font: FontHandle) -> Self {
        Self {
            color: VecDeque::from(vec![color]),
            font_size: VecDeque::from(vec![font_size]),
            font: VecDeque::from(vec![font]),
            text: None,
            font_style: FontStyle::Regular,
            small_caps: false
        }
    }
}

fn parse_html(
    text: &str,
    default_color: [f32; 4],
    default_font_size: f32,
    font: &FontFamily,
    styles: &HashMap<String, Style>,
    fonts: &AssetStorage<FontAsset>
) -> Vec<TextSection> {
    let styles_regex = Regex::new(r#"(<style="(?P<key>.*)">)(?P<content>.*)(</style>)"#).unwrap();
    let expanded = styles_regex.replace_all(text, |caps: &Captures| {
        let key = &caps["key"];
        let content = &caps["content"];
        let replacement = styles.get(key);
        if let Some(replacement) = replacement {
            format!(
                "{}{}{}",
                replacement.start_tags, content, replacement.end_tags
            )
        } else {
            log::warn!("Missing style: {}", key);
            content.to_string()
        }
    });
    let mut style = StyleStack::new(default_color, default_font_size, font.regular.clone());
    let mut sections = Vec::new();
    let tag_split_regex =
        Regex::new(r#"(<(?P<closing>/)?(?P<tag>.*?)(=(?P<tag_value>[^>]*))?>)|(?P<text>[^<]*)"#)
            .unwrap();
    for part in tag_split_regex.captures_iter(&expanded) {
        let tag = part.name("tag");
        let is_closing = part.name("closing").is_some();
        let tag_value = part.name("tag_value");
        let content = part.name("text");
        if let Some(tag) = tag {
            apply_style(&mut style, &mut sections, fonts);

            let tag_type = TagType::from_tag(tag.as_str());
            match tag_type {
                TagType::Color => {
                    if is_closing {
                        style.color.pop_front();
                    } else {
                        let value = RGBA::from_str(tag_value.unwrap().as_str()).unwrap();
                        style.color.push_front(value.to_linear());
                    }
                }
                TagType::Size => {
                    if is_closing {
                        style.font_size.pop_front();
                    } else {
                        let value =
                            font_size_from_str(tag_value.unwrap().as_str(), style.font_size[0]);
                        style.font_size.push_front(value);
                    }
                }
                TagType::Font => {
                    if is_closing {
                        //font.pop_front();
                    } else {
                        log::warn!("Font setting currently unsupported")
                    }
                }
                TagType::Bold => {
                    if is_closing {
                        style.font.pop_front();
                        style.font_style = match style.font_style {
                            FontStyle::BoldItalic => FontStyle::Italic,
                            _ => FontStyle::Regular
                        };
                    } else {
                        style.font_style = match style.font_style {
                            FontStyle::Italic | FontStyle::BoldItalic => FontStyle::BoldItalic,
                            _ => FontStyle::Bold
                        };
                        let handle = font_from_style(style.font_style, &font);
                        style.font.push_front(handle);
                    }
                }
                TagType::Italic => {
                    if is_closing {
                        style.font.pop_front();
                        style.font_style = match style.font_style {
                            FontStyle::BoldItalic => FontStyle::Bold,
                            _ => FontStyle::Regular
                        };
                    } else {
                        style.font_style = match style.font_style {
                            FontStyle::Bold | FontStyle::BoldItalic => FontStyle::BoldItalic,
                            _ => FontStyle::Italic
                        };
                        let handle = font_from_style(style.font_style, &font);
                        style.font.push_front(handle);
                    }
                }
                TagType::SmallCaps => {
                    style.small_caps = !is_closing;
                }
                TagType::Unsupported => {
                    log::warn!("Unsupported tag {}", tag.as_str());
                }
            }
        } else if let Some(text) = content {
            apply_style(&mut style, &mut sections, fonts);
            style.text = Some(text.as_str());
        }
    }

    // Apply style to remaining text
    apply_style(&mut style, &mut sections, fonts);

    if style.color.len() > 1
        || style.font.len() > 1
        || style.font_size.len() > 1
        || style.small_caps
        || style.text.is_some()
    {
        log::warn!(
            "Found unbalanced tags in HTML string, remainder: {:?}",
            style
        );
    }
    log::debug!("Sections: {:?}", sections);

    sections
}

fn apply_style(
    style: &mut StyleStack,
    sections: &mut Vec<TextSection>,
    fonts: &AssetStorage<FontAsset>
) {
    fn section(text: String, style: &StyleStack) -> TextSection {
        TextSection {
            text,
            color: style.color[0],
            font: style.font[0].clone(),
            font_size: style.font_size[0]
        }
    }

    if let Some(text) = style.text.take() {
        if style.small_caps {
            let x_height = fonts
                .get(&style.font[0])
                .map(|font| font.0.glyph('x').scaled(Scale::uniform(style.font_size[0])))
                .and_then(|x| x.exact_bounding_box())
                .map(|bbox| bbox.height() * 2.0)
                .unwrap_or_else(|| style.font_size[0] / 2.0);
            log::debug!("X height: {}", x_height);
            // group consecutive lowercase chars separately from uppercase and uncased.
            // lowercase will be upper-cased and rendered at 0.5em, the rest at 1em
            let groups = text.chars().group_by(|c| !c.is_uppercase());
            for (is_lower, chars) in &groups {
                let text = chars.collect::<String>();
                let mut section = section(text, style);
                if is_lower {
                    section.font_size = x_height;
                    section.text = section.text.to_uppercase();
                }
                sections.push(section);
            }
        } else {
            sections.push(section(text.to_string(), style));
        }
    }
}

fn font_from_style(style: FontStyle, fonts: &FontFamily) -> FontHandle {
    let handle = match style {
        FontStyle::Regular => fonts.regular.clone(),
        FontStyle::Bold => {
            if let Some(bold) = &fonts.bold {
                bold.clone()
            } else {
                log::warn!("Used bold tag but no bold font set");
                fonts.regular.clone()
            }
        }
        FontStyle::BoldItalic => {
            if let Some(bold_italic) = &fonts.bold_italic {
                bold_italic.clone()
            } else {
                log::warn!("Used bold and italic but no bold_italic font set");
                fonts.regular.clone()
            }
        }
        FontStyle::Italic => {
            if let Some(italic) = &fonts.italic {
                italic.clone()
            } else {
                log::warn!("Used italic but no italic font set");
                fonts.regular.clone()
            }
        }
    };
    handle
}

fn font_size_from_str(value: &str, current: f32) -> f32 {
    if value.ends_with('%') {
        let percentage: u32 = value[..value.len() - 1].parse().unwrap();
        let factor = percentage as f32 / 100.0;
        current * factor
    } else if value.ends_with("em") {
        let factor: f32 = value[..value.len() - 2].parse().unwrap();
        current * factor
    } else {
        let font_size: u32 = value.parse().unwrap();
        font_size as f32
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum FontStyle {
    Regular,
    Bold,
    BoldItalic,
    Italic
}

#[derive(Debug, PartialEq)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl FromStr for RGBA {
    type Err = ParseIntError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let r = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b = u8::from_str_radix(&hex_code[5..7], 16)?;
        let a = if hex_code.len() == 9 {
            u8::from_str_radix(&hex_code[7..9], 16)?
        } else {
            255
        };
        Ok(RGBA { r, g, b, a })
    }
}

impl RGBA {
    fn to_linear(&self) -> [f32; 4] {
        let linear = Srgba::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0
        )
        .into_linear();
        let (r, g, b, a) = linear.into_components();
        [r, g, b, a]
    }
}

enum TagType {
    Color,
    Size,
    Font,
    Bold,
    Italic,
    SmallCaps,
    Unsupported
}

impl TagType {
    fn from_tag(tag: &str) -> Self {
        match tag {
            "color" => Self::Color,
            "size" => Self::Size,
            "font" => Self::Font,
            "b" => Self::Bold,
            "i" => Self::Italic,
            "smallcaps" => Self::SmallCaps,
            _ => Self::Unsupported
        }
    }
}
