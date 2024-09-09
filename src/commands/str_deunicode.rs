use deunicode::deunicode;
use nu_plugin::{EngineInterface, EvaluatedCall, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, ShellError, Signature, Span, Type, Value};

use crate::StrutilsPlugin;

pub struct StrDeunicode;

impl SimplePluginCommand for StrDeunicode {
    type Plugin = StrutilsPlugin;

    fn name(&self) -> &str {
        "str deunicode"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .input_output_types(vec![(Type::String, Type::String)])
            .category(Category::Strings)
    }

    fn description(&self) -> &str {
        "Convert Unicode string to pure ASCII."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["convert", "ascii"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "deunicode a string",
            example: "'A…C' | str deunicode",
            result: Some(Value::test_string("A...C")),
        }]
    }

    fn run(
        &self,
        _plugin: &StrutilsPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(do_deunicode(input, call.head))
    }
}

fn do_deunicode(input: &Value, head: Span) -> Value {
    match input {
        Value::String { val, .. } => Value::string(deunicode(val), head),
        Value::Error { .. } => input.clone(),
        _ => Value::error(
            ShellError::OnlySupportsThisInputType {
                exp_input_type: "string".into(),
                wrong_type: input.get_type().to_string(),
                dst_span: head,
                src_span: input.span(),
            },
            head,
        ),
    }
}

#[test]
fn test_examples() -> Result<(), nu_protocol::ShellError> {
    use nu_plugin_test_support::PluginTest;

    // This will automatically run the examples specified in your command and compare their actual
    // output against what was specified in the example.
    //
    // We recommend you add this test to any other commands you create, or remove it if the examples
    // can't be tested this way.

    PluginTest::new("strutils", StrutilsPlugin.into())?.test_command_examples(&StrDeunicode)
}
