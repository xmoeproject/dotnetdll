use dotnetdll::prelude::*;

mod common;

#[test]
pub fn write() {
    common::write_fixture(
        "resources",
        |ctx| {
            ctx.resolution.manifest_resources.extend([resource::ManifestResource {
                attributes: vec![],
                name: "strings.resources".into(),
                visibility: resource::Visibility::Public,
                //////
                // using (var res = new ResourceWriter("./strings.resources")) {
                //   res.AddResource("string", "foo bar");
                // }
                //////
                implementation: resource::Implementation::CurrentFile(include_bytes!("./strings.resources").as_slice().into()),
            }]);

            let mscorlib = ctx.mscorlib;
            let console: MethodType = BaseType::class(ctx.console).into();

            let resource_manager_t: MethodType = BaseType::class(
                ctx.resolution
                    .push_type_reference(type_ref! { System.Resources.ResourceManager in #mscorlib }),
            )
            .into();

            let type_t: MethodType = BaseType::class(
                ctx.resolution
                    .push_type_reference(type_ref! { System.Type in #mscorlib }),
            )
            .into();
            let runtime_type_handle = BaseType::valuetype(
                ctx.resolution
                    .push_type_reference(type_ref! { System.RuntimeTypeHandle in #mscorlib })
            ).into();
            let assembly_t: MethodType = BaseType::class(
                ctx.resolution
                    .push_type_reference(type_ref! { System.Reflection.Assembly in #mscorlib })
            ).into();

            (
                vec![],
                vec![],
                asm! {
                    load_string "strings";
                    load_token_type BaseType::class(ctx.class);
                    call ctx.resolution.push_method_reference(method_ref! { static @type_t @type_t::GetTypeFromHandle(#runtime_type_handle) });
                    call_virtual ctx.resolution.push_method_reference(method_ref! { @assembly_t @type_t::get_Assembly() });
                    new_object ctx.resolution.push_method_reference(method_ref! { void @resource_manager_t::.ctor(string, @assembly_t) });
                    load_string "string";
                    call_virtual ctx.resolution.push_method_reference(method_ref! { string @resource_manager_t::GetString(string) });
                    call ctx.resolution.push_method_reference(method_ref! { static void #console::WriteLine(string) });
                    Return;
                },
            )
        },
        b"foo bar\n",
    )
    .unwrap();
}
