use dotnetdll::prelude::*;

mod common;

#[test]
pub fn write() {
    common::write_fixture(
        "fields_props",
        |ctx| {
            let console_type = BaseType::class(ctx.console.into()).into();
            let write_line = ctx
                .resolution
                .push_method_reference(method_ref! { static void #console_type::WriteLine(string, object, object) });

            let static_field = ctx.resolution.push_field(
                ctx.class,
                Field::static_member(Accessibility::Private, "static_field".into(), ctype! { int }),
            );
            let instance_field = ctx.resolution.push_field(
                ctx.class,
                Field::instance(Accessibility::Private, "instance_field".into(), ctype! { uint }),
            );

            let static_type: MethodType = ctx.resolution[static_field].return_type.clone().into();
            let static_prop = ctx.resolution.push_property(
                ctx.class,
                Property::new(
                    "StaticProperty".into(),
                    Parameter::new(ParameterType::Value(static_type.clone())),
                ),
            );
            let static_getter = ctx.resolution.set_property_getter(
                static_prop,
                Method::new(
                    Accessibility::Public,
                    msig! { static @static_type () },
                    "get_StaticProperty".into(),
                    Some(body::Method::new(vec![
                        Instruction::LoadStaticField {
                            volatile: false,
                            field: static_field.into(),
                        },
                        Instruction::Return,
                    ])),
                ),
            );
            let static_setter = ctx.resolution.set_property_setter(
                static_prop,
                Method::new(
                    Accessibility::Public,
                    msig! { static void (@static_type) },
                    "set_StaticProperty".into(),
                    Some(body::Method::new(vec![
                        Instruction::LoadArgument(0),
                        Instruction::StoreStaticField {
                            volatile: false,
                            field: static_field.into(),
                        },
                        Instruction::Return,
                    ])),
                ),
            );

            let instance_type: MethodType = ctx.resolution[instance_field].return_type.clone().into();
            let instance_prop = ctx.resolution.push_property(
                ctx.class,
                Property::new(
                    "InstanceProperty".into(),
                    Parameter::new(ParameterType::Value(instance_type.clone())),
                ),
            );
            let instance_getter = ctx.resolution.set_property_getter(
                instance_prop,
                Method::new(
                    Accessibility::Public,
                    msig! { @instance_type () },
                    "get_InstanceProperty".into(),
                    Some(body::Method::new(vec![
                        Instruction::LoadArgument(0),
                        Instruction::LoadField {
                            unaligned: None,
                            volatile: false,
                            field: instance_field.into(),
                        },
                        Instruction::Return,
                    ])),
                ),
            );
            let instance_setter = ctx.resolution.set_property_setter(
                instance_prop,
                Method::new(
                    Accessibility::Public,
                    msig! { void (@instance_type) },
                    "set_InstanceProperty".into(),
                    Some(body::Method::new(vec![
                        Instruction::LoadArgument(0),
                        Instruction::LoadArgument(1),
                        Instruction::StoreField {
                            unaligned: None,
                            volatile: false,
                            field: instance_field.into(),
                        },
                        Instruction::Return,
                    ])),
                ),
            );

            (
                vec![LocalVariable::new(BaseType::class(ctx.class.into()).into())],
                vec![
                    // init static
                    Instruction::LoadConstantInt32(-1),
                    Instruction::Call {
                        tail_call: false,
                        method: static_setter.into(),
                    },
                    // init object and instance
                    Instruction::NewObject(ctx.default_ctor.into()),
                    Instruction::Duplicate,
                    Instruction::StoreLocal(0),
                    Instruction::LoadConstantInt32(1),
                    Instruction::Call {
                        tail_call: false,
                        method: instance_setter.into(),
                    },
                    // increment static
                    Instruction::Call {
                        tail_call: false,
                        method: static_getter.into(),
                    },
                    Instruction::LoadConstantInt32(1),
                    Instruction::Add,
                    Instruction::Call {
                        tail_call: false,
                        method: static_setter.into(),
                    },
                    // increment instance
                    Instruction::LoadLocalVariable(0),
                    Instruction::Duplicate,
                    Instruction::Call {
                        tail_call: false,
                        method: instance_getter.into(),
                    },
                    Instruction::LoadConstantInt32(1),
                    Instruction::Add,
                    Instruction::Call {
                        tail_call: false,
                        method: instance_setter.into(),
                    },
                    // call writeline
                    Instruction::LoadString("{0}, {1}".encode_utf16().collect()),
                    Instruction::Call {
                        tail_call: false,
                        method: static_getter.into(),
                    },
                    Instruction::Box(static_type),
                    Instruction::LoadLocalVariable(0),
                    Instruction::Call {
                        tail_call: false,
                        method: instance_getter.into(),
                    },
                    Instruction::Box(instance_type),
                    Instruction::Call {
                        tail_call: false,
                        method: write_line.into(),
                    },
                    Instruction::Return,
                ],
            )
        },
        b"0, 2\n",
    )
    .unwrap();
}
