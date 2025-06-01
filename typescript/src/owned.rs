use cooklang::{ast::Ast, error::SourceDiag, parser::{BlockKind, Event, QuantityValue}, Span};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, tsify::Tsify)]
pub struct OwnedText {
    pub text: String,
    pub span: Span,
}

/// Owned version of Event for wasm_bindgen compatibility
#[derive(Debug, tsify::Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub enum OwnedEvent {
    YAMLFrontMatter(OwnedText),
    Metadata { key: OwnedText, value: OwnedText },
    Section { name: Option<OwnedText> },
    Start(BlockKind),
    End(BlockKind),
    Text(OwnedText),
    Ingredient(OwnedLocated<OwnedIngredient>),
    Cookware(OwnedLocated<OwnedCookware>),
    Timer(OwnedLocated<OwnedTimer>),
    Error(SourceDiag),
    Warning(SourceDiag),
}

#[derive(Debug, tsify::Tsify, Serialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct OwnedLocated<T> where T: Clone {
    pub data: T,
    pub span: Span,
}

#[derive(Debug, tsify::Tsify, Serialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct OwnedIngredient {
    pub name: OwnedText,
    pub quantity: Option<OwnedQuantity>,
    pub alias: Option<OwnedText>,
    pub modifiers: String,
    pub note: Option<OwnedText>,
}

#[derive(Debug, tsify::Tsify, Serialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct OwnedCookware {
    pub name: OwnedText,
    pub quantity: Option<QuantityValue>,
    pub alias: Option<OwnedText>,
    pub modifiers: String,
    pub note: Option<OwnedText>,
}

#[derive(Debug, tsify::Tsify, Serialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct OwnedTimer {
    pub name: Option<OwnedText>,
    pub quantity: Option<OwnedQuantity>,
}

#[derive(Debug, tsify::Tsify, Serialize, Clone)]
#[tsify(into_wasm_abi)]
pub struct OwnedQuantity {
    pub value: QuantityValue,
    pub unit: Option<OwnedText>,
}

impl From<Event<'_>> for OwnedEvent {
    fn from(event: Event<'_>) -> Self {
        match event {
            Event::YAMLFrontMatter(text) => OwnedEvent::YAMLFrontMatter(
                OwnedText {
                    text: text.text().into_owned(),
                    span: text.span(),
                },
            ),
            Event::Metadata { key, value } => OwnedEvent::Metadata {
                key: OwnedText {
                    text: key.text().into_owned(),
                    span: key.span(),
                },
                value: OwnedText {
                    text: value.text().into_owned(),
                    span: value.span(),
                },
            },
            Event::Section { name } => OwnedEvent::Section {
                name: name.map(|n| OwnedText {
                    text: n.text().into_owned(),
                    span: n.span(),
                }),
            },
            Event::Start(kind) => OwnedEvent::Start(kind),
            Event::End(kind) => OwnedEvent::End(kind),
            Event::Text(text) => OwnedEvent::Text(
                OwnedText {
                    text: text.text().into_owned(),
                    span: text.span(),
                },
            ),
            Event::Ingredient(located) => OwnedEvent::Ingredient(OwnedLocated {
                data: OwnedIngredient {
                    name: OwnedText {
                        text: located.name.text().into_owned(),
                        span: located.name.span(),
                    },
                    quantity: located.quantity.clone().map(|q| OwnedQuantity {
                        value: q.value.clone(),
                        unit: q.unit.as_ref().map(|u| OwnedText {
                            text: u.text().into_owned(),
                            span: u.span(),
                        }),
                    }),
                    alias: located.alias.as_ref().map(|a| OwnedText {
                        text: a.text().into_owned(),
                        span: a.span(),
                    }),
                    modifiers: located.modifiers.to_string(),
                    note: located.note.as_ref().map(|n| OwnedText {
                        text: n.text().into_owned(),
                        span: n.span(),
                    }),
                },
                span: located.span(),
            }),
            Event::Cookware(located) => OwnedEvent::Cookware(OwnedLocated {
                data: OwnedCookware {
                    name: OwnedText {
                        text: located.name.text().into_owned(),
                        span: located.name.span(),
                    },
                    quantity: located.quantity.clone().map(|q| q.into_inner()),
                    alias: located.alias.as_ref().map(|a| OwnedText {
                        text: a.text().into_owned(),
                        span: a.span(),
                    }),
                    modifiers: located.modifiers.to_string(),
                    note: located.note.as_ref().map(|n| OwnedText {
                        text: n.text().into_owned(),
                        span: n.span(),
                    }),
                },
                span: located.span(),
            }),
            Event::Timer(located) => OwnedEvent::Timer(OwnedLocated {
                data: OwnedTimer {
                    name: located.name.as_ref().map(|n| OwnedText {
                        text: n.text().into_owned(),
                        span: n.span(),
                    }),
                    quantity: located.quantity.clone().map(|q| OwnedQuantity {
                        value: q.value.clone(),
                        unit: q.unit.as_ref().map(|u| OwnedText {
                            text: u.text().into_owned(),
                            span: u.span(),
                        }),
                    }),
                },
                span: located.span(),
            }),
            Event::Error(diag) => OwnedEvent::Error(diag),
            Event::Warning(diag) => OwnedEvent::Warning(diag),
        }
    }
}

#[derive(Debug, Serialize, Clone, tsify::Tsify)]
pub enum OwnedItem {
    Text(OwnedText),
    Ingredient(OwnedLocated<OwnedIngredient>),
    Cookware(OwnedLocated<OwnedCookware>),
    Timer(OwnedLocated<OwnedTimer>),
}

#[derive(Debug, Serialize, Clone, tsify::Tsify)]
pub enum OwnedBlock {
    Metadata { key: OwnedText, value: OwnedText },
    Section { name: Option<OwnedText> },
    Step { items: Vec<OwnedItem> },
    TextBlock(Vec<OwnedText>),
}

#[derive(Debug, Serialize, Clone, tsify::Tsify)]
pub struct OwnedAst {
    pub blocks: Vec<OwnedBlock>,
}

impl From<Ast<'_>> for OwnedAst {
    fn from(ast: Ast<'_>) -> Self {
        OwnedAst {
            blocks: ast.blocks.into_iter().map(|block| {
                match block {
                    cooklang::parser::Block::Metadata { key, value } => {
                        OwnedBlock::Metadata {
                            key: OwnedText {
                                text: key.text().into_owned(),
                                span: key.span(),
                            },
                            value: OwnedText {
                                text: value.text().into_owned(),
                                span: value.span(),
                            },
                        }
                    }
                    cooklang::parser::Block::Section { name } => {
                        OwnedBlock::Section {
                            name: name.map(|n| OwnedText {
                                text: n.text().into_owned(),
                                span: n.span(),
                            }),
                        }
                    }
                    cooklang::parser::Block::Step { items } => {
                        OwnedBlock::Step {
                            items: items.into_iter().map(|item| {
                                match item {
                                    cooklang::parser::Item::Text(text) => OwnedItem::Text(OwnedText {
                                        text: text.text().into_owned(),
                                        span: text.span(),
                                    }),
                                    cooklang::parser::Item::Ingredient(located) => OwnedItem::Ingredient(OwnedLocated {
                                        data: OwnedIngredient {
                                            name: OwnedText {
                                                text: located.name.text().into_owned(),
                                                span: located.name.span(),
                                            },
                                            quantity: located.quantity.clone().map(|q| OwnedQuantity {
                                                value: q.value.clone(),
                                                unit: q.unit.as_ref().map(|u| OwnedText {
                                                    text: u.text().into_owned(),
                                                    span: u.span(),
                                                }),
                                            }),
                                            alias: located.alias.as_ref().map(|a| OwnedText {
                                                text: a.text().into_owned(),
                                                span: a.span(),
                                            }),
                                            modifiers: located.modifiers.to_string(),
                                            note: located.note.as_ref().map(|n| OwnedText {
                                                text: n.text().into_owned(),
                                                span: n.span(),
                                            }),
                                        },
                                        span: located.span(),
                                    }),
                                    cooklang::parser::Item::Cookware(located) => OwnedItem::Cookware(OwnedLocated {
                                        data: OwnedCookware {
                                            name: OwnedText {
                                                text: located.name.text().into_owned(),
                                                span: located.name.span(),
                                            },
                                            quantity: located.quantity.clone().map(|q| q.into_inner()),
                                            alias: located.alias.as_ref().map(|a| OwnedText {
                                                text: a.text().into_owned(),
                                                span: a.span(),
                                            }),
                                            modifiers: located.modifiers.to_string(),
                                            note: located.note.as_ref().map(|n| OwnedText {
                                                text: n.text().into_owned(),
                                                span: n.span(),
                                            }),
                                        },
                                        span: located.span(),
                                    }),
                                    cooklang::parser::Item::Timer(located) => OwnedItem::Timer(OwnedLocated {
                                        data: OwnedTimer {
                                            name: located.name.as_ref().map(|n| OwnedText {
                                                text: n.text().into_owned(),
                                                span: n.span(),
                                            }),
                                            quantity: located.quantity.clone().map(|q| OwnedQuantity {
                                                value: q.value.clone(),
                                                unit: q.unit.as_ref().map(|u| OwnedText {
                                                    text: u.text().into_owned(),
                                                    span: u.span(),
                                                }),
                                            }),
                                        },
                                        span: located.span(),
                                    }),
                                }
                            }).collect(),
                        }
                    }
                    cooklang::parser::Block::TextBlock(texts) => {
                        OwnedBlock::TextBlock(
                            texts.into_iter().map(|text| OwnedText {
                                text: text.text().into_owned(),
                                span: text.span(),
                            }).collect()
                        )
                    }
                }
            }).collect(),
        }
    }
}