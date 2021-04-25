mod syntax;
mod template_builder;
pub use self::template_builder::TemplateBuilder;
/// Module contains errors for `TemplateBuilder` 
pub mod template_build_error;
mod build_error;
mod template;
pub use self::template::Template;
mod argument_types_differ_error;
/// Module contains errors for `Template` 
pub mod template_error;