use std::collections::VecDeque;
use cooklang::{ast::build_ast, Converter, CooklangParser, Extensions};
use wasm_bindgen::prelude::*;
use crate::{ast::AstInterop, event::EventsInterop, metadata::MetadataInterop, recipe::{RecipeResultInterop, ScalableRecipeInterop, ScaledRecipeInterop}, render};

#[wasm_bindgen]
pub struct CooklangParserHandle(CooklangParser);

#[wasm_bindgen]
impl CooklangParserHandle {

    #[wasm_bindgen(constructor)]
    pub fn new(extensions: Option<Extensions>, converter: Option<Converter>) -> Self {
        Self (
            CooklangParser::new(
                extensions.unwrap_or_default(), 
                converter.unwrap_or_default(),
            )
        )
    }

    pub fn parse(&self, input: &str) -> RecipeResultInterop {
        let result = self.0.parse(input);
        RecipeResultInterop(result)
    }

    #[wasm_bindgen(js_name = "scaleRecipe")]
    pub fn scale_recipe(&self, recipe: ScalableRecipeInterop, scale: f64) -> ScaledRecipeInterop {
        let scaled = recipe.0.scale(scale, self.0.converter());
        ScaledRecipeInterop(scaled)
    }

    #[wasm_bindgen(js_name = "defaultScaleRecipe")]
    pub fn default_scale_recipe(&self, recipe: ScalableRecipeInterop) -> ScaledRecipeInterop {
        let scaled = recipe.0.default_scale();
        ScaledRecipeInterop(scaled)
    }

    #[wasm_bindgen(js_name = "renderRecipe")]
    pub fn render_recipe(&self, recipe: ScaledRecipeInterop) -> String {
        render(recipe.0, self.0.converter())
    }

    #[wasm_bindgen(js_name = "parseEvents")]
    pub fn parse_events(&self, input: &str) -> EventsInterop {
        let events = cooklang::parser::PullParser::new(input, self.0.extensions());
        let mut event_queue = VecDeque::new();
        for e in events {
            event_queue.push_back(e.into());
        }
        EventsInterop(event_queue)
    }

    #[wasm_bindgen(js_name = "parseAst")]
    pub fn parse_ast(&self, input: &str) -> Option<AstInterop> {
        let events = cooklang::parser::PullParser::new(input, self.0.extensions());
        let (ast, _) = build_ast(events).into_tuple();
        if let Some(ast) = ast {
            Some(AstInterop(ast.into()))
        } else {
            None
        }
    }

    #[wasm_bindgen(js_name = "parseMetadata")]
    pub fn parse_metadata(&self, input: &str) -> Option<MetadataInterop> {
        let (meta, _report) = self.0.parse_metadata(input).into_tuple();
        meta.map(|m| {
            MetadataInterop {
                tags: m.tags().map(|tags| tags.into_iter().map(|tag| tag.to_string()).collect()),
                author: m.author(),
                source: m.source(),
                time: m.time(self.0.converter()),
                servings: m.servings(),
                locale: m.locale().clone().map(|(lang, region)| {
                    (lang.to_string(), region.map(str::to_string))
                }),
            }
        })
    }
}


pub mod helpers {
    use cooklang::error::SourceReport;
    use wasm_bindgen::prelude::*;

    use super::RecipeResultInterop;

    #[wasm_bindgen(js_name = "intoRsonText")]
    pub fn into_rson_text(result: RecipeResultInterop) -> Option<String> {
        let value = result.0.output()?;
        Some(format!("{value:#?}"))
    }

    #[wasm_bindgen(js_name = "intoErrorText")]
    pub fn into_error_text(report: SourceReport, input: &str) -> Option<String> {
        let mut buf = Vec::new();
        report.write("playground", input, true, &mut buf).unwrap();
        let ansi_error = String::from_utf8_lossy(&buf);
        let error = ansi_to_html::convert(&ansi_error).unwrap_or_else(|_| ansi_error.into_owned());
        if error.is_empty() {
            None
        } else {
            Some(error)
        }
    }


}
