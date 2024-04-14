use crate::ecmascript::{
    builders::builtin_function_builder::BuiltinFunctionBuilder,
    builtins::{ArgumentsList, Behaviour, Builtin},
    execution::{Agent, JsResult, RealmIdentifier},
    types::{IntoObject, Object, String, Value, BUILTIN_STRING_MEMORY},
};

pub(crate) struct WeakMapConstructor;
impl Builtin for WeakMapConstructor {
    const NAME: String = BUILTIN_STRING_MEMORY.Map;

    const LENGTH: u8 = 1;

    const BEHAVIOUR: Behaviour = Behaviour::Constructor(WeakMapConstructor::behaviour);
}

impl WeakMapConstructor {
    fn behaviour(
        _agent: &mut Agent,
        _this_value: Value,
        _arguments: ArgumentsList,
        _new_target: Option<Object>,
    ) -> JsResult<Value> {
        todo!()
    }

    pub(crate) fn create_intrinsic(agent: &mut Agent, realm: RealmIdentifier) {
        let intrinsics = agent.get_realm(realm).intrinsics();
        let weak_map_prototype = intrinsics.weak_map_prototype();
        let this = intrinsics.weak_map();
        let this_object_index = intrinsics.weak_map_base_object();

        BuiltinFunctionBuilder::new_intrinsic_constructor::<WeakMapConstructor>(
            agent,
            realm,
            this,
            Some(this_object_index),
        )
        .with_property_capacity(1)
        .with_prototype_property(weak_map_prototype.into_object())
        .build();
    }
}
