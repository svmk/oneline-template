use crate::template::syntax::template_token::TemplateToken;

#[derive(Debug)]
pub enum Token {
    Text(String),
    Template(TemplateToken),
}