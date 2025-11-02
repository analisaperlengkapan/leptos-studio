use leptos_studio::builder::canvas::CanvasComponent;
use leptos_studio::builder::component_library::{LibraryComponent, PropSchema};
use leptos_studio::builder::export::{generate_leptos_code, ExportPreset};
#[test]
fn test_library_component_props_schema_mixed_required_and_desc() {
    let comp = LibraryComponent {
        name: "MixedProps".to_string(),
        kind: "Input".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: Some(vec![
            PropSchema {
                name: "foo".to_string(),
                prop_type: "string".to_string(),
                required: true,
                description: Some("desc foo".to_string()),
            },
            PropSchema {
                name: "bar".to_string(),
                prop_type: "number".to_string(),
                required: false,
                description: None,
            },
            PropSchema {
                name: "baz".to_string(),
                prop_type: "bool".to_string(),
                required: true,
                description: None,
            },
        ]),
        description: Some("desc".to_string()),
    };
    let schema = comp.props_schema.as_ref().unwrap();
    assert!(schema[0].required);
    assert!(!schema[1].required);
    assert!(schema[2].required);
    assert_eq!(schema[0].description.as_deref(), Some("desc foo"));
    assert!(schema[1].description.is_none());
    assert!(schema[2].description.is_none());
    assert_eq!(comp.description.as_deref(), Some("desc"));
}

#[test]
fn test_export_input_placeholder_long_unicode() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "panjang🌏🚀😀テストabcdefghijklmnopqrstuvwxyz0123456789".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains(
        "<input placeholder=\"panjang🌏🚀😀テストabcdefghijklmnopqrstuvwxyz0123456789\" />"
    ));
}

#[test]
fn test_library_component_serialization_roundtrip() {
    let original = LibraryComponent {
        name: "SerTest".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>Ser</div>".to_string()),
        category: "Test".to_string(),
        props_schema: Some(vec![PropSchema {
            name: "foo".to_string(),
            prop_type: "string".to_string(),
            required: true,
            description: Some("desc".to_string()),
        }]),
        description: Some("desc".to_string()),
    };
    let json = serde_json::to_string(&original).expect("serialize");
    let decoded: LibraryComponent = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(format!("{:?}", original), format!("{:?}", decoded));
}
#[test]
fn test_library_component_props_schema_required_and_desc_some() {
    let comp = LibraryComponent {
        name: "ReqDescSome".to_string(),
        kind: "Input".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: Some(vec![
            PropSchema {
                name: "foo".to_string(),
                prop_type: "string".to_string(),
                required: true,
                description: Some("desc foo".to_string()),
            },
            PropSchema {
                name: "bar".to_string(),
                prop_type: "number".to_string(),
                required: true,
                description: Some("desc bar".to_string()),
            },
        ]),
        description: None,
    };
    let schema = comp.props_schema.as_ref().unwrap();
    assert!(schema[0].required);
    assert!(schema[1].required);
    assert_eq!(schema[0].description.as_deref(), Some("desc foo"));
    assert_eq!(schema[1].description.as_deref(), Some("desc bar"));
}

#[test]
fn test_export_input_placeholder_empty_and_whitespace_special() {
    let layout = vec![
        CanvasComponent::Input {
            placeholder: "".to_string(),
        },
        CanvasComponent::Input {
            placeholder: "   !@#  ".to_string(),
        },
    ];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"\" />"));
    assert!(code.contains("<input placeholder=\"   !@#  \" />"));
}
#[test]
fn test_library_component_props_schema_required_and_desc_none() {
    let comp = LibraryComponent {
        name: "ReqDescNone".to_string(),
        kind: "Input".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: Some(vec![
            PropSchema {
                name: "foo".to_string(),
                prop_type: "string".to_string(),
                required: true,
                description: None,
            },
            PropSchema {
                name: "bar".to_string(),
                prop_type: "number".to_string(),
                required: false,
                description: None,
            },
        ]),
        description: None,
    };
    let schema = comp.props_schema.as_ref().unwrap();
    assert!(schema[0].required);
    assert!(!schema[1].required);
    assert!(schema[0].description.is_none());
    assert!(schema[1].description.is_none());
}

#[test]
fn test_export_input_placeholder_unicode_and_emoji() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "テスト🌟😀".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"テスト🌟😀\" />"));
}
#[test]
fn test_export_custom_component_unicode_name() {
    let layout = vec![CanvasComponent::Custom {
        name: "コンポーネント🌟".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "コンポーネント🌟".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>UnicodeName</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("UnicodeName"));
    assert!(code.contains("Custom: コンポーネント🌟"));
}

#[test]
fn test_export_input_placeholder_escape_chars() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "\"quote\" \\ backslash".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"\"quote\" \\ backslash\" />"));
}

#[test]
fn test_export_container_many_children() {
    let mut children = Vec::new();
    for i in 0..20 {
        children.push(CanvasComponent::Button {
            label: format!("Btn{}", i),
        });
    }
    let layout = vec![CanvasComponent::Container { children }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    for i in 0..20 {
        assert!(code.contains(&format!("<button>{{\"Btn{}\"}}</button>", i)));
    }
}

#[test]
fn test_library_component_props_schema_types() {
    let comp = LibraryComponent {
        name: "TypedProps".to_string(),
        kind: "Input".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: Some(vec![
            PropSchema {
                name: "placeholder".to_string(),
                prop_type: "string".to_string(),
                required: true,
                description: None,
            },
            PropSchema {
                name: "max_length".to_string(),
                prop_type: "number".to_string(),
                required: false,
                description: None,
            },
        ]),
        description: None,
    };
    let schema = comp.props_schema.as_ref().unwrap();
    assert_eq!(schema.len(), 2);
    assert_eq!(schema[0].prop_type, "string");
    assert_eq!(schema[1].prop_type, "number");
}
#[test]
fn test_export_custom_component_empty_props_schema_vec() {
    let layout = vec![CanvasComponent::Custom {
        name: "EmptyProps".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "EmptyProps".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>EmptyProps</div>".to_string()),
        category: "Test".to_string(),
        props_schema: Some(vec![]),
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("EmptyProps"));
    assert!(custom_components[0]
        .props_schema
        .as_ref()
        .unwrap()
        .is_empty());
}

#[test]
fn test_export_input_placeholder_whitespace() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "   ".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"   \" />"));
}

#[test]
fn test_export_deeply_nested_container_and_custom() {
    let mut comp = CanvasComponent::Custom {
        name: "DeepCustom".to_string(),
    };
    for _ in 0..5 {
        comp = CanvasComponent::Container {
            children: vec![comp],
        };
    }
    let layout = vec![comp];
    let custom_components = vec![LibraryComponent {
        name: "DeepCustom".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>DeepCustomContent</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.matches("<div class=\"container\">").count() >= 5);
    assert!(code.contains("DeepCustomContent"));
}

#[test]
fn test_library_component_description_none() {
    let comp = LibraryComponent {
        name: "NoDesc".to_string(),
        kind: "Button".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: None,
        description: None,
    };
    assert!(comp.description.is_none());
}
#[test]
fn test_export_custom_component_no_props_schema() {
    let layout = vec![CanvasComponent::Custom {
        name: "NoProps".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "NoProps".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>No props</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("No props"));
}

#[test]
fn test_export_large_mixed_layout() {
    let mut layout = Vec::new();
    for i in 0..30 {
        layout.push(CanvasComponent::Button {
            label: format!("Btn{}", i),
        });
        layout.push(CanvasComponent::Text {
            content: format!("Text{}", i),
        });
        layout.push(CanvasComponent::Input {
            placeholder: format!("Ph{}", i),
        });
        layout.push(CanvasComponent::Container {
            children: vec![
                CanvasComponent::Button {
                    label: format!("InnerBtn{}", i),
                },
                CanvasComponent::Text {
                    content: format!("InnerText{}", i),
                },
            ],
        });
    }
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    for i in 0..30 {
        assert!(code.contains(&format!("Btn{}", i)));
        assert!(code.contains(&format!("Text{}", i)));
        assert!(code.contains(&format!("Ph{}", i)));
        assert!(code.contains(&format!("InnerBtn{}", i)));
        assert!(code.contains(&format!("InnerText{}", i)));
    }
}

#[test]
fn test_canvas_component_container_with_mixed_children() {
    let comp = CanvasComponent::Container {
        children: vec![
            CanvasComponent::Button {
                label: "A".to_string(),
            },
            CanvasComponent::Input {
                placeholder: "B".to_string(),
            },
            CanvasComponent::Text {
                content: "C".to_string(),
            },
        ],
    };
    if let CanvasComponent::Container { children } = &comp {
        assert_eq!(children.len(), 3);
    } else {
        panic!("Not a container");
    }
}

#[test]
fn test_export_custom_component_with_description() {
    let layout = vec![CanvasComponent::Custom {
        name: "DescComp".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "DescComp".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>Desc</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: Some("A description".to_string()),
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Desc"));
    assert!(custom_components[0].description.as_deref() == Some("A description"));
}
#[test]
fn test_export_button_unicode_and_long_label() {
    let layout = vec![CanvasComponent::Button {
        label: "你好🌏🚀a very very very very very very very very long label!".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("你好🌏🚀a very very very very very very very very long label!"));
}

#[test]
fn test_export_deeply_nested_10_levels() {
    let mut comp = CanvasComponent::Button {
        label: "Deepest".to_string(),
    };
    for _ in 0..10 {
        comp = CanvasComponent::Container {
            children: vec![comp],
        };
    }
    let layout = vec![comp];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.matches("<div class=\"container\">").count() >= 10);
    assert!(code.contains("Deepest"));
}

#[test]
fn test_export_custom_component_invalid_template() {
    let layout = vec![CanvasComponent::Custom {
        name: "Invalid".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "Invalid".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>Unclosed".to_string()), // invalid HTML
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Unclosed"));
}

#[test]
fn test_export_text_with_special_and_control_chars() {
    let layout = vec![CanvasComponent::Text {
        content: "Line1\nLine2\t\u{0007}".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("Line1"));
    assert!(code.contains("Line2"));
}

#[test]
fn test_canvas_component_button_empty_label() {
    let comp = CanvasComponent::Button {
        label: String::new(),
    };
    if let CanvasComponent::Button { label } = &comp {
        assert!(label.is_empty());
    } else {
        panic!("Not a button");
    }
}

#[test]
fn test_canvas_component_custom_empty_name() {
    let comp = CanvasComponent::Custom {
        name: String::new(),
    };
    if let CanvasComponent::Custom { name } = &comp {
        assert!(name.is_empty());
    } else {
        panic!("Not a custom");
    }
}
// Integration test: Simulate layout changes and export

#[test]
fn test_export_code_generation() {
    let layout = vec![
        CanvasComponent::Button {
            label: "Hello".to_string(),
        },
        CanvasComponent::Text {
            content: "World".to_string(),
        },
        CanvasComponent::Container {
            children: vec![CanvasComponent::Input {
                placeholder: "Type here".to_string(),
            }],
        },
    ];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    let expected = r#"// Generated by Leptos Studio
use leptos::*;

#[component]
pub fn GeneratedView() -> impl IntoView {
    view! {
        <button>{"Hello"}</button>
        <span>{"World"}</span>
        <div class="container">
            <input placeholder="Type here" />
        </div>
    }
}
"#;
    let normalize = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    assert_eq!(
        normalize(&code),
        normalize(expected),
        "Output tidak sesuai snapshot.\nOutput:\n{}\nExpected:\n{}",
        code,
        expected
    );
}

#[test]
fn test_export_with_custom_component() {
    let layout = vec![CanvasComponent::Custom {
        name: "MyCustom".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "MyCustom".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>Custom!</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(
        code.contains("Custom: MyCustom"),
        "Custom component marker missing"
    );
    assert!(code.contains("Custom!"), "Custom template missing");
}

#[test]
fn test_export_empty_layout() {
    let layout: Vec<CanvasComponent> = vec![];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(
        code.contains("GeneratedView"),
        "Should still generate component"
    );
    assert!(code.contains("view!"), "Should still generate view! macro");
}

#[test]
fn test_export_nested_container() {
    let layout = vec![CanvasComponent::Container {
        children: vec![
            CanvasComponent::Button {
                label: "A".to_string(),
            },
            CanvasComponent::Input {
                placeholder: "B".to_string(),
            },
        ],
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(
        code.contains("<div class=\"container\">"),
        "Container div missing"
    );
    assert!(
        code.contains("<button>{\"A\"}</button>"),
        "Nested button missing"
    );
    assert!(
        code.contains("<input placeholder=\"B\" />"),
        "Nested input missing"
    );
}

#[test]
fn test_export_custom_component_not_found() {
    let layout = vec![CanvasComponent::Custom {
        name: "NotExist".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("Custom: NotExist"), "Custom marker missing");
    assert!(
        code.contains("Template not found"),
        "Fallback template missing"
    );
}

#[test]
fn test_export_mixed_components() {
    let layout = vec![
        CanvasComponent::Button {
            label: "Btn".to_string(),
        },
        CanvasComponent::Input {
            placeholder: "Ph".to_string(),
        },
        CanvasComponent::Text {
            content: "Teks".to_string(),
        },
    ];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<button>{\"Btn\"}</button>"));
    assert!(code.contains("<input placeholder=\"Ph\" />"));
    assert!(code.contains("<span>{\"Teks\"}</span>"));
}

#[test]
fn test_export_deeply_nested() {
    let layout = vec![CanvasComponent::Container {
        children: vec![CanvasComponent::Container {
            children: vec![CanvasComponent::Button {
                label: "Deep".to_string(),
            }],
        }],
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(
        code.matches("<div class=\"container\">").count() >= 2,
        "Should have at least 2 container divs"
    );
    assert!(code.contains("<button>{\"Deep\"}</button>"));
}

#[test]
fn test_export_input_empty_placeholder() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"\" />"));
}

#[test]
fn test_export_custom_component_empty_template() {
    let layout = vec![CanvasComponent::Custom {
        name: "EmptyCustom".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "EmptyCustom".to_string(),
        kind: "Custom".to_string(),
        template: Some(String::new()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Custom: EmptyCustom"));
    // Should still output an empty comment block
    assert!(code.contains("/*  */"));
}

#[test]
fn test_export_custom_component_null_template() {
    let layout = vec![CanvasComponent::Custom {
        name: "NullCustom".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "NullCustom".to_string(),
        kind: "Custom".to_string(),
        template: None,
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Custom: NullCustom"));
    assert!(code.contains("Template not found"));
}

#[test]
fn test_export_nested_custom_and_basic() {
    let layout = vec![CanvasComponent::Container {
        children: vec![
            CanvasComponent::Custom {
                name: "C1".to_string(),
            },
            CanvasComponent::Button {
                label: "Btn".to_string(),
            },
        ],
    }];
    let custom_components = vec![LibraryComponent {
        name: "C1".to_string(),
        kind: "Custom".to_string(),
        template: Some("<b>CustomC1</b>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Custom: C1"));
    assert!(code.contains("CustomC1"));
    assert!(code.contains("<button>{\"Btn\"}</button>"));
}

#[test]
fn test_export_order_variation() {
    let layout1 = vec![
        CanvasComponent::Button {
            label: "A".to_string(),
        },
        CanvasComponent::Text {
            content: "B".to_string(),
        },
    ];
    let layout2 = vec![
        CanvasComponent::Text {
            content: "B".to_string(),
        },
        CanvasComponent::Button {
            label: "A".to_string(),
        },
    ];
    let code1 = generate_leptos_code(&layout1, &[], ExportPreset::Plain);
    let code2 = generate_leptos_code(&layout2, &[], ExportPreset::Plain);
    assert_ne!(code1, code2, "Order of components should affect output");
}

#[test]
fn test_export_label_with_whitespace() {
    let layout = vec![CanvasComponent::Button {
        label: "  spaced  ".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<button>{\"  spaced  \"}</button>"));
}

#[test]
fn test_export_custom_component_special_chars() {
    let layout = vec![CanvasComponent::Custom {
        name: "C$#@!".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "C$#@!".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>!@#$%^&*</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Custom: C$#@!"));
    assert!(code.contains("!@#$%^&*"));
}

#[test]
fn test_export_repeated_components() {
    let layout = vec![
        CanvasComponent::Button {
            label: "X".to_string(),
        },
        CanvasComponent::Button {
            label: "X".to_string(),
        },
        CanvasComponent::Button {
            label: "X".to_string(),
        },
    ];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert_eq!(
        code.matches("<button>{\"X\"}</button>").count(),
        3,
        "Should have 3 identical buttons"
    );
}

#[test]
fn test_export_custom_multiline_template() {
    let layout = vec![CanvasComponent::Custom {
        name: "Multi".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "Multi".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>Line1\nLine2\nLine3</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Line1"));
    assert!(code.contains("Line2"));
    assert!(code.contains("Line3"));
}

#[test]
fn test_export_input_special_chars() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "<>&\"'".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"<>&\"'\" />"));
}

#[test]
fn test_export_nested_custom_container_input() {
    let layout = vec![CanvasComponent::Container {
        children: vec![
            CanvasComponent::Custom {
                name: "C2".to_string(),
            },
            CanvasComponent::Input {
                placeholder: "Z".to_string(),
            },
        ],
    }];
    let custom_components = vec![LibraryComponent {
        name: "C2".to_string(),
        kind: "Custom".to_string(),
        template: Some("<i>NestedC2</i>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Custom: C2"));
    assert!(code.contains("NestedC2"));
    assert!(code.contains("<input placeholder=\"Z\" />"));
}

#[test]
fn test_export_large_layout_stress() {
    let mut layout = Vec::new();
    for i in 0..100 {
        layout.push(CanvasComponent::Button {
            label: format!("Btn{}", i),
        });
    }
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    for i in 0..100 {
        assert!(
            code.contains(&format!("<button>{{\"Btn{}\"}}</button>", i)),
            "Missing Btn{}",
            i
        );
    }
}

#[test]
fn test_export_custom_unicode_template() {
    let layout = vec![CanvasComponent::Custom {
        name: "Uni".to_string(),
    }];
    let custom_components = vec![LibraryComponent {
        name: "Uni".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>你好🌏</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("你好🌏"));
}

#[test]
fn test_export_input_with_emoji() {
    let layout = vec![CanvasComponent::Input {
        placeholder: "😀🚀".to_string(),
    }];
    let code = generate_leptos_code(&layout, &[], ExportPreset::Plain);
    assert!(code.contains("<input placeholder=\"😀🚀\" />"));
}

#[test]
fn test_export_deeply_nested_custom_and_basic() {
    let layout = vec![CanvasComponent::Container {
        children: vec![
            CanvasComponent::Custom {
                name: "DeepC".to_string(),
            },
            CanvasComponent::Container {
                children: vec![
                    CanvasComponent::Button {
                        label: "B1".to_string(),
                    },
                    CanvasComponent::Input {
                        placeholder: "P1".to_string(),
                    },
                ],
            },
        ],
    }];
    let custom_components = vec![LibraryComponent {
        name: "DeepC".to_string(),
        kind: "Custom".to_string(),
        template: Some("<div>DeepCustom</div>".to_string()),
        category: "Test".to_string(),
        props_schema: None,
        description: None,
    }];
    let code = generate_leptos_code(&layout, &custom_components, ExportPreset::Plain);
    assert!(code.contains("Custom: DeepC"));
    assert!(code.contains("DeepCustom"));
    assert!(code.contains("<button>{\"B1\"}</button>"));
    assert!(code.contains("<input placeholder=\"P1\" />"));
}

#[test]
fn test_canvas_component_serialization_roundtrip() {
    let original = CanvasComponent::Container {
        children: vec![
            CanvasComponent::Button {
                label: "A".to_string(),
            },
            CanvasComponent::Input {
                placeholder: "B".to_string(),
            },
        ],
    };
    let json = serde_json::to_string(&original).expect("serialize");
    let decoded: CanvasComponent = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(format!("{:?}", original), format!("{:?}", decoded));
}

#[test]
fn test_undo_redo_stack_behavior() {
    let mut undo_stack: Vec<Vec<CanvasComponent>> = vec![];
    let mut redo_stack: Vec<Vec<CanvasComponent>> = vec![];
    let mut state = vec![CanvasComponent::Button {
        label: "A".to_string(),
    }];
    // Simulasi edit
    undo_stack.push(state.clone());
    state.push(CanvasComponent::Text {
        content: "B".to_string(),
    });
    // Undo
    if let Some(prev) = undo_stack.pop() {
        redo_stack.push(state.clone());
        state = prev;
    }
    assert_eq!(state.len(), 1);
    // Redo
    if let Some(next) = redo_stack.pop() {
        undo_stack.push(state.clone());
        state = next;
    }
    assert_eq!(state.len(), 2);
}

#[test]
fn test_library_component_props_schema() {
    let comp = LibraryComponent {
        name: "TestInput".to_string(),
        kind: "Input".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: Some(vec![PropSchema {
            name: "placeholder".to_string(),
            prop_type: "string".to_string(),
            required: false,
            description: Some("desc".to_string()),
        }]),
        description: Some("desc".to_string()),
    };
    assert_eq!(comp.props_schema.as_ref().unwrap().len(), 1);
    assert_eq!(comp.props_schema.as_ref().unwrap()[0].name, "placeholder");
    assert_eq!(
        comp.props_schema.as_ref().unwrap()[0]
            .description
            .as_deref(),
        Some("desc")
    );
}

#[test]
fn test_library_component_default_basic() {
    let btn = LibraryComponent {
        name: "Button".to_string(),
        kind: "Button".to_string(),
        template: None,
        category: "Basic".to_string(),
        props_schema: None,
        description: None,
    };
    assert_eq!(btn.name, "Button");
    assert_eq!(btn.kind, "Button");
    assert_eq!(btn.category, "Basic");
    assert!(btn.props_schema.is_none());
}

#[test]
fn test_canvas_component_equality() {
    let a = CanvasComponent::Button {
        label: "X".to_string(),
    };
    let b = CanvasComponent::Button {
        label: "X".to_string(),
    };
    let c = CanvasComponent::Button {
        label: "Y".to_string(),
    };
    assert_eq!(format!("{:?}", a), format!("{:?}", b));
    assert_ne!(format!("{:?}", a), format!("{:?}", c));
}

#[test]
fn test_canvas_component_deserialize_invalid_json() {
    let bad_json = "{\"Button\":{\"label\":123}}";
    let result: Result<CanvasComponent, _> = serde_json::from_str(bad_json);
    assert!(result.is_err(), "Should error on invalid label type");
}

#[test]
fn test_canvas_component_empty_container() {
    let comp = CanvasComponent::Container { children: vec![] };
    if let CanvasComponent::Container { children } = &comp {
        assert!(children.is_empty());
    } else {
        panic!("Not a container");
    }
}

#[test]
fn test_canvas_component_input_no_placeholder() {
    let comp = CanvasComponent::Input {
        placeholder: String::new(),
    };
    if let CanvasComponent::Input { placeholder } = &comp {
        assert!(placeholder.is_empty());
    } else {
        panic!("Not an input");
    }
}

#[test]
fn test_canvas_component_text_empty() {
    let comp = CanvasComponent::Text {
        content: String::new(),
    };
    if let CanvasComponent::Text { content } = &comp {
        assert!(content.is_empty());
    } else {
        panic!("Not a text");
    }
}
