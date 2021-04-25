# One-line template
One-line template is library for serialization structures in one-line string. 
This library is helpful for path formatting.

## Template format

* Path navigation: `{self}`, `{self.field_1.field_2}`, `{self.0}`
* Value processing: `{self | uint:hex() | string:substr(0uint, 2uint)|string:unwrap_or('--')}`

## Example
```
[dependencies]
oneline-template = "0.1"
```

```rust
use oneline_template::template::TemplateBuilder;

#[derive(serde_derive::Serialize)]
struct FileId {
  file_id: u32,
}

fn main() {
  let template_builder = TemplateBuilder::new();
  let template = template_builder.build("directory/file_no_{ self.file_id }.json").expect("Unable to create template");
  
  let file_id = FileId {file_id: 1};
  let path = template.serialize(&file_id).expect("Unable to format path");
  assert_eq!("directory/file_no_1.json", &path);
}
```
## Custom function

You can implement your own processing function. Contribution in this project is welcomed!

```rust

use oneline_template::function_executor::*;
use oneline_template::template::TemplateBuilder;

/// Function: `uint::neg`
/// 
/// Input: `uint`
///
/// Returns `uint`
pub struct Neg;

impl FunctionExecutor for Neg {
   fn schema(&self) -> FunctionSchema {
       FunctionSchema::new("uint:neg")
   }

   fn call(&self, value: Value, _arguments: &[Value]) -> Result<Value, FunctionError> {
       let value = value.as_uint()?;
       let value = !value;
       let value = Value::UInt(value);
       return Ok(value);
   }
}

#[derive(serde_derive::Serialize)]
struct FileId {
  file_id: u32,
}

fn main() {
  let template_builder = TemplateBuilder::new();
  let template_builder = template_builder.with_function(Neg);
  let template = template_builder.build("directory/file_no_{ self.file_id | uint:neg( ) | uint:hex( ) }.json").expect("Unable to create template");
  
  let file_id = FileId {file_id: 1};
  let path = template.serialize(&file_id).expect("Unable to format path");
  assert_eq!("directory/file_no_fffffffffffffffffffffffffffffffe.json", &path);
}
```