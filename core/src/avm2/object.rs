//! AVM2 objects.

use crate::avm2::activation::Activation;
use crate::avm2::array::ArrayStorage;
use crate::avm2::bytearray::ByteArrayStorage;
use crate::avm2::class::Class;
use crate::avm2::domain::Domain;
use crate::avm2::error;
use crate::avm2::events::{DispatchList, Event};
use crate::avm2::function::Executable;
use crate::avm2::property::Property;
use crate::avm2::regexp::RegExp;
use crate::avm2::value::{Hint, Value};
use crate::avm2::vector::VectorStorage;
use crate::avm2::vtable::{ClassBoundMethod, VTable};
use crate::avm2::Error;
use crate::avm2::Multiname;
use crate::avm2::Namespace;
use crate::avm2::QName;
use crate::bitmap::bitmap_data::BitmapDataWrapper;
use crate::context::UpdateContext;
use crate::display_object::DisplayObject;
use crate::html::TextFormat;
use crate::streams::NetStream;
use crate::string::AvmString;
use gc_arena::{Collect, Gc, GcCell, MutationContext};
use ruffle_macros::enum_trait_object;
use std::cell::{Ref, RefMut};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

mod array_object;
mod bitmapdata_object;
mod bytearray_object;
mod class_object;
mod context3d_object;
mod date_object;
mod dictionary_object;
mod dispatch_object;
mod domain_object;
mod error_object;
mod event_object;
mod function_object;
mod index_buffer_3d_object;
mod loaderinfo_object;
mod namespace_object;
mod netstream_object;
mod primitive_object;
mod program_3d_object;
mod proxy_object;
mod qname_object;
mod regexp_object;
mod script_object;
mod shader_data_object;
mod socket_object;
mod sound_object;
mod soundchannel_object;
mod stage3d_object;
mod stage_object;
mod textformat_object;
mod texture_object;
mod vector_object;
mod vertex_buffer_3d_object;
mod xml_list_object;
mod xml_object;

pub use crate::avm2::object::array_object::{array_allocator, ArrayObject, ArrayObjectWeak};
pub use crate::avm2::object::bitmapdata_object::{
    bitmap_data_allocator, BitmapDataObject, BitmapDataObjectWeak,
};
pub use crate::avm2::object::bytearray_object::{
    byte_array_allocator, ByteArrayObject, ByteArrayObjectWeak,
};
pub use crate::avm2::object::class_object::{ClassObject, ClassObjectWeak};
pub use crate::avm2::object::context3d_object::{Context3DObject, Context3DObjectWeak};
pub use crate::avm2::object::date_object::{date_allocator, DateObject, DateObjectWeak};
pub use crate::avm2::object::dictionary_object::{
    dictionary_allocator, DictionaryObject, DictionaryObjectWeak,
};
pub use crate::avm2::object::dispatch_object::{DispatchObject, DispatchObjectWeak};
pub use crate::avm2::object::domain_object::{
    application_domain_allocator, DomainObject, DomainObjectWeak,
};
pub use crate::avm2::object::error_object::{error_allocator, ErrorObject, ErrorObjectWeak};
pub use crate::avm2::object::event_object::{event_allocator, EventObject, EventObjectWeak};
pub use crate::avm2::object::function_object::{
    function_allocator, FunctionObject, FunctionObjectWeak,
};
pub use crate::avm2::object::index_buffer_3d_object::{
    IndexBuffer3DObject, IndexBuffer3DObjectWeak,
};
pub use crate::avm2::object::loaderinfo_object::{
    loader_info_allocator, LoaderInfoObject, LoaderInfoObjectWeak, LoaderStream,
};
pub use crate::avm2::object::namespace_object::{
    namespace_allocator, NamespaceObject, NamespaceObjectWeak,
};
pub use crate::avm2::object::netstream_object::{
    netstream_allocator, NetStreamObject, NetStreamObjectWeak,
};
pub use crate::avm2::object::primitive_object::{
    primitive_allocator, PrimitiveObject, PrimitiveObjectWeak,
};
pub use crate::avm2::object::program_3d_object::{Program3DObject, Program3DObjectWeak};
pub use crate::avm2::object::proxy_object::{proxy_allocator, ProxyObject, ProxyObjectWeak};
pub use crate::avm2::object::qname_object::{q_name_allocator, QNameObject, QNameObjectWeak};
pub use crate::avm2::object::regexp_object::{reg_exp_allocator, RegExpObject, RegExpObjectWeak};
pub use crate::avm2::object::script_object::{ScriptObject, ScriptObjectData, ScriptObjectWeak};
pub use crate::avm2::object::shader_data_object::{
    shader_data_allocator, ShaderDataObject, ShaderDataObjectWeak,
};
pub use crate::avm2::object::socket_object::{socket_allocator, SocketObject, SocketObjectWeak};
pub use crate::avm2::object::sound_object::{
    sound_allocator, QueuedPlay, SoundData, SoundObject, SoundObjectWeak,
};
pub use crate::avm2::object::soundchannel_object::{
    sound_channel_allocator, SoundChannelObject, SoundChannelObjectWeak,
};
pub use crate::avm2::object::stage3d_object::{
    stage_3d_allocator, Stage3DObject, Stage3DObjectWeak,
};
pub use crate::avm2::object::stage_object::{StageObject, StageObjectWeak};
pub use crate::avm2::object::textformat_object::{
    textformat_allocator, TextFormatObject, TextFormatObjectWeak,
};
pub use crate::avm2::object::texture_object::{TextureObject, TextureObjectWeak};
pub use crate::avm2::object::vector_object::{vector_allocator, VectorObject, VectorObjectWeak};
pub use crate::avm2::object::vertex_buffer_3d_object::{
    VertexBuffer3DObject, VertexBuffer3DObjectWeak,
};
pub use crate::avm2::object::xml_list_object::{
    xml_list_allocator, E4XOrXml, XmlListObject, XmlListObjectWeak,
};
pub use crate::avm2::object::xml_object::{xml_allocator, XmlObject, XmlObjectWeak};

/// Represents an object that can be directly interacted with by the AVM2
/// runtime.
#[enum_trait_object(
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Collect, Debug, Copy)]
    #[collect(no_drop)]
    pub enum Object<'gc> {
        ScriptObject(ScriptObject<'gc>),
        FunctionObject(FunctionObject<'gc>),
        PrimitiveObject(PrimitiveObject<'gc>),
        NamespaceObject(NamespaceObject<'gc>),
        ArrayObject(ArrayObject<'gc>),
        StageObject(StageObject<'gc>),
        DomainObject(DomainObject<'gc>),
        EventObject(EventObject<'gc>),
        DispatchObject(DispatchObject<'gc>),
        XmlObject(XmlObject<'gc>),
        XmlListObject(XmlListObject<'gc>),
        RegExpObject(RegExpObject<'gc>),
        ByteArrayObject(ByteArrayObject<'gc>),
        LoaderInfoObject(LoaderInfoObject<'gc>),
        ClassObject(ClassObject<'gc>),
        VectorObject(VectorObject<'gc>),
        SoundObject(SoundObject<'gc>),
        SoundChannelObject(SoundChannelObject<'gc>),
        BitmapDataObject(BitmapDataObject<'gc>),
        DateObject(DateObject<'gc>),
        DictionaryObject(DictionaryObject<'gc>),
        QNameObject(QNameObject<'gc>),
        TextFormatObject(TextFormatObject<'gc>),
        ProxyObject(ProxyObject<'gc>),
        ErrorObject(ErrorObject<'gc>),
        Stage3DObject(Stage3DObject<'gc>),
        Context3DObject(Context3DObject<'gc>),
        IndexBuffer3DObject(IndexBuffer3DObject<'gc>),
        VertexBuffer3DObject(VertexBuffer3DObject<'gc>),
        TextureObject(TextureObject<'gc>),
        Program3DObject(Program3DObject<'gc>),
        NetStreamObject(NetStreamObject<'gc>),
        ShaderDataObject(ShaderDataObject<'gc>),
        SocketObject(SocketObject<'gc>)
    }
)]
pub trait TObject<'gc>: 'gc + Collect + Debug + Into<Object<'gc>> + Clone + Copy {
    /// Get the base of this object.
    /// Any trait method implementations that were not overrided will forward the call to this instead.
    fn base(&self) -> Ref<ScriptObjectData<'gc>>;
    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc>>;

    /// Retrieve a local property of the object. The Multiname should always be public.
    ///
    /// This skips class field lookups and looks at:
    /// - object-specific storage (like arrays)
    /// - Object dynamic properties
    /// - prototype chain.
    fn get_property_local(
        self,
        name: &Multiname<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.base().get_property_local(name, activation)
    }

    /// Retrieve a property by Multiname lookup.
    ///
    /// This method should not be overridden.
    ///
    /// This corresponds directly to the AVM2 operation `getproperty`, with the
    /// exception that it does not special-case object lookups on dictionary
    /// structured objects.
    #[allow(unused_mut)] //Not unused.
    fn get_property(
        mut self,
        multiname: &Multiname<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        match self.vtable().and_then(|vtable| vtable.get_trait(multiname)) {
            Some(Property::Slot { slot_id }) | Some(Property::ConstSlot { slot_id }) => {
                self.base().get_slot(slot_id)
            }
            Some(Property::Method { disp_id }) => {
                // avmplus has a special case for XML and XMLList objects, so we need one as well
                // https://github.com/adobe/avmplus/blob/858d034a3bd3a54d9b70909386435cf4aec81d21/core/Toplevel.cpp#L629-L634
                if (self.as_xml_object().is_some() || self.as_xml_list_object().is_some())
                    && multiname.contains_public_namespace()
                {
                    return self.get_property_local(multiname, activation);
                }

                if let Some(bound_method) = self.get_bound_method(disp_id) {
                    return Ok(bound_method.into());
                }
                let vtable = self.vtable().unwrap();
                if let Some(bound_method) =
                    vtable.make_bound_method(activation, self.into(), disp_id)
                {
                    self.install_bound_method(activation.context.gc_context, disp_id, bound_method);
                    Ok(bound_method.into())
                } else {
                    Err("Method not found".into())
                }
            }
            Some(Property::Virtual { get: Some(get), .. }) => {
                self.call_method(get, &[], activation)
            }
            Some(Property::Virtual { get: None, .. }) => {
                return Err(error::make_reference_error(
                    activation,
                    error::ReferenceErrorCode::ReadFromWriteOnly,
                    multiname,
                    self.instance_of(),
                ));
            }
            None => self.get_property_local(multiname, activation),
        }
    }

    /// Same as get_property, but constructs a public Multiname for you.
    fn get_public_property(
        self,
        name: impl Into<AvmString<'gc>>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.get_property(
            &Multiname::new(activation.avm2().public_namespace, name),
            activation,
        )
    }

    /// Set a local property of the object. The Multiname should always be public.
    ///
    /// This skips class field lookups and looks at:
    /// - object-specific storage (like arrays)
    /// - Object dynamic properties
    fn set_property_local(
        self,
        name: &Multiname<'gc>,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<(), Error<'gc>> {
        let mut base = self.base_mut(activation.context.gc_context);
        base.set_property_local(name, value, activation)
    }

    /// Same as get_property_local, but constructs a public Multiname for you.
    /// TODO: this feels upside down, as in: we shouldn't need multinames/namespaces
    /// by the time we reach dynamic properties.
    /// But for now, this function is a smaller change to the core than a full refactor.
    fn set_string_property_local(
        self,
        name: impl Into<AvmString<'gc>>,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<(), Error<'gc>> {
        let name = Multiname::new(activation.avm2().public_namespace, name);
        self.set_property_local(&name, value, activation)
    }

    /// Set a property by Multiname lookup.
    ///
    /// This method should not be overridden.
    ///
    /// This corresponds directly with the AVM2 operation `setproperty`, with
    /// the exception that it does not special-case object lookups on
    /// dictionary structured objects.
    fn set_property(
        &mut self,
        multiname: &Multiname<'gc>,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<(), Error<'gc>> {
        match self.vtable().and_then(|vtable| vtable.get_trait(multiname)) {
            Some(Property::Slot { slot_id }) => {
                let value = self
                    .vtable()
                    .unwrap()
                    .coerce_trait_value(slot_id, value, activation)?;
                self.base_mut(activation.context.gc_context).set_slot(
                    slot_id,
                    value,
                    activation.context.gc_context,
                )
            }
            Some(Property::Method { .. }) => {
                // Similar to the get_property special case for XML/XMLList.
                if (self.as_xml_object().is_some() || self.as_xml_list_object().is_some())
                    && multiname.contains_public_namespace()
                {
                    return self.set_property_local(multiname, value, activation);
                }

                return Err(error::make_reference_error(
                    activation,
                    error::ReferenceErrorCode::AssignToMethod,
                    multiname,
                    self.instance_of(),
                ));
            }
            Some(Property::Virtual { set: Some(set), .. }) => {
                self.call_method(set, &[value], activation).map(|_| ())
            }
            Some(Property::ConstSlot { .. }) | Some(Property::Virtual { set: None, .. }) => {
                return Err(error::make_reference_error(
                    activation,
                    error::ReferenceErrorCode::WriteToReadOnly,
                    multiname,
                    self.instance_of(),
                ));
            }
            None => self.set_property_local(multiname, value, activation),
        }
    }

    /// Same as set_property, but constructs a public Multiname for you.
    fn set_public_property(
        &mut self,
        name: impl Into<AvmString<'gc>>,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<(), Error<'gc>> {
        self.set_property(
            &Multiname::new(activation.avm2().public_namespace, name),
            value,
            activation,
        )
    }

    /// Init a local property of the object. The Multiname should always be public.
    ///
    /// This skips class field lookups and looks at:
    /// - object-specific storage (like arrays)
    /// - Object dynamic properties
    ///
    /// This should be effectively equivalent to set_property_local,
    /// as "init" is a concept specific to class const fields.
    fn init_property_local(
        self,
        name: &Multiname<'gc>,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<(), Error<'gc>> {
        let mut base = self.base_mut(activation.context.gc_context);
        base.init_property_local(name, value, activation)
    }

    /// Initialize a property by Multiname lookup.
    ///
    /// This method should not be overridden.
    ///
    /// This corresponds directly with the AVM2 operation `initproperty`.
    fn init_property(
        &mut self,
        multiname: &Multiname<'gc>,
        value: Value<'gc>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<(), Error<'gc>> {
        match self.vtable().and_then(|vtable| vtable.get_trait(multiname)) {
            Some(Property::Slot { slot_id }) | Some(Property::ConstSlot { slot_id }) => {
                let value = self
                    .vtable()
                    .unwrap()
                    .coerce_trait_value(slot_id, value, activation)?;
                self.base_mut(activation.context.gc_context).set_slot(
                    slot_id,
                    value,
                    activation.context.gc_context,
                )
            }
            Some(Property::Method { .. }) => {
                return Err(error::make_reference_error(
                    activation,
                    error::ReferenceErrorCode::AssignToMethod,
                    multiname,
                    self.instance_of(),
                ));
            }
            Some(Property::Virtual { set: Some(set), .. }) => {
                self.call_method(set, &[value], activation).map(|_| ())
            }
            Some(Property::Virtual { set: None, .. }) => {
                return Err(error::make_reference_error(
                    activation,
                    error::ReferenceErrorCode::WriteToReadOnly,
                    multiname,
                    self.instance_of(),
                ));
            }
            None => self.init_property_local(multiname, value, activation),
        }
    }

    /// Call a local property of the object. The Multiname should always be public.
    ///
    /// This skips class field lookups and looks at:
    /// - object-specific storage (like arrays)
    /// - Object dynamic properties
    /// - prototype chain
    fn call_property_local(
        self,
        multiname: &Multiname<'gc>,
        arguments: &[Value<'gc>],
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        // Note: normally this would just call into ScriptObjectData::call_property_local
        // but because calling into ScriptObjectData borrows it for entire duration,
        // we run a risk of a double borrow if the inner call borrows again.
        let self_val: Value<'gc> = Value::from(self.into());
        let result = self
            .base()
            .get_property_local(multiname, activation)?
            .as_callable(activation, Some(multiname), Some(self_val))?;

        result.call(self_val, arguments, activation)
    }

    /// Call a named property on the object.
    ///
    /// This method should not be overridden.
    ///
    /// This corresponds directly to the `callproperty` operation in AVM2.
    #[allow(unused_mut)]
    fn call_property(
        mut self,
        multiname: &Multiname<'gc>,
        arguments: &[Value<'gc>],
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        match self.vtable().and_then(|vtable| vtable.get_trait(multiname)) {
            Some(Property::Slot { slot_id }) | Some(Property::ConstSlot { slot_id }) => {
                let obj = self.base().get_slot(slot_id)?.as_callable(
                    activation,
                    Some(multiname),
                    Some(Value::from(self.into())),
                )?;

                obj.call(Value::from(self.into()), arguments, activation)
            }
            Some(Property::Method { disp_id }) => {
                let vtable = self.vtable().unwrap();
                if let Some(ClassBoundMethod {
                    class,
                    scope,
                    method,
                }) = vtable.get_full_method(disp_id)
                {
                    if !method.needs_arguments_object() {
                        Executable::from_method(method, scope, None, Some(class)).exec(
                            Value::from(self.into()),
                            arguments,
                            activation,
                            class.into(), //Deliberately invalid.
                        )
                    } else {
                        if let Some(bound_method) = self.get_bound_method(disp_id) {
                            return bound_method.call(
                                Value::from(self.into()),
                                arguments,
                                activation,
                            );
                        }
                        let bound_method = vtable
                            .make_bound_method(activation, self.into(), disp_id)
                            .unwrap();
                        self.install_bound_method(
                            activation.context.gc_context,
                            disp_id,
                            bound_method,
                        );
                        bound_method.call(Value::from(self.into()), arguments, activation)
                    }
                } else {
                    Err("Method not found".into())
                }
            }
            Some(Property::Virtual { get: Some(get), .. }) => {
                let obj = self.call_method(get, &[], activation)?.as_callable(
                    activation,
                    Some(multiname),
                    Some(Value::from(self.into())),
                )?;

                obj.call(Value::from(self.into()), arguments, activation)
            }
            Some(Property::Virtual { get: None, .. }) => {
                return Err(error::make_reference_error(
                    activation,
                    error::ReferenceErrorCode::ReadFromWriteOnly,
                    multiname,
                    self.instance_of(),
                ));
            }
            None => self.call_property_local(multiname, arguments, activation),
        }
    }

    /// Same as call_property, but constructs a public Multiname for you.
    fn call_public_property(
        self,
        name: impl Into<AvmString<'gc>>,
        arguments: &[Value<'gc>],
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        self.call_property(
            &Multiname::new(activation.avm2().public_namespace, name),
            arguments,
            activation,
        )
    }

    /// Retrieve a slot by its index.
    fn get_slot(self, id: u32) -> Result<Value<'gc>, Error<'gc>> {
        let base = self.base();

        base.get_slot(id)
    }

    /// Set a slot by its index.
    fn set_slot(
        self,
        id: u32,
        value: Value<'gc>,
        mc: MutationContext<'gc, '_>,
    ) -> Result<(), Error<'gc>> {
        let mut base = self.base_mut(mc);

        base.set_slot(id, value, mc)
    }

    /// Initialize a slot by its index.
    fn init_slot(
        self,
        id: u32,
        value: Value<'gc>,
        mc: MutationContext<'gc, '_>,
    ) -> Result<(), Error<'gc>> {
        let mut base = self.base_mut(mc);

        base.init_slot(id, value, mc)
    }

    /// Call a method by its index.
    ///
    /// This directly corresponds with the AVM2 operation `callmethod`.
    #[allow(unused_mut)] //Not unused.
    fn call_method(
        mut self,
        id: u32,
        arguments: &[Value<'gc>],
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        if self.get_bound_method(id).is_none() {
            if let Some(vtable) = self.vtable() {
                if let Some(bound_method) = vtable.make_bound_method(activation, self.into(), id) {
                    self.install_bound_method(activation.context.gc_context, id, bound_method);
                }
            }
        }

        let bound_method = self.get_bound_method(id);
        if let Some(method_object) = bound_method {
            return method_object.call(Value::from(self.into()), arguments, activation);
        }

        Err(format!("Cannot call unknown method id {id}").into())
    }

    /// Implements the `in` opcode and AS3 operator.
    ///
    /// By default, this just calls `has_property`, but may be overridden by
    /// other object types to change the behavior of the `in` operator only.
    fn has_property_via_in(
        self,
        _activation: &mut Activation<'_, 'gc>,
        name: &Multiname<'gc>,
    ) -> Result<bool, Error<'gc>> {
        Ok(self.has_property(name))
    }

    /// Indicates whether or not a property exists on an object.
    fn has_property(self, name: &Multiname<'gc>) -> bool {
        if self.has_own_property(name) {
            true
        } else if let Some(proto) = self.proto() {
            proto.has_own_property(name)
        } else {
            false
        }
    }

    /// Same as has_property, but constructs a public Multiname for you.
    fn has_public_property(
        self,
        name: impl Into<AvmString<'gc>>,
        activation: &mut Activation<'_, 'gc>,
    ) -> bool {
        self.has_property(&Multiname::new(activation.avm2().public_namespace, name))
    }

    /// Indicates whether or not a property or trait exists on an object and is
    /// not part of the prototype chain.
    fn has_own_property(self, name: &Multiname<'gc>) -> bool {
        self.base().has_own_property(name)
    }

    /// Same as has_own_property, but constructs a public Multiname for you.
    fn has_own_property_string(
        self,
        name: impl Into<AvmString<'gc>>,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<bool, Error<'gc>> {
        Ok(self.has_own_property(&Multiname::new(activation.avm2().public_namespace, name)))
    }

    /// Returns true if an object has one or more traits of a given name.
    fn has_trait(self, name: &Multiname<'gc>) -> bool {
        let base = self.base();
        base.has_trait(name)
    }

    /// Delete a property by QName, after multiname resolution and all other
    /// considerations have been taken.
    ///
    /// This required method is only intended to be called by other TObject
    /// methods.
    fn delete_property_local(
        self,
        activation: &mut Activation<'_, 'gc>,
        name: &Multiname<'gc>,
    ) -> Result<bool, Error<'gc>> {
        let mut base = self.base_mut(activation.context.gc_context);

        Ok(base.delete_property_local(name))
    }

    /// Delete a named property from the object.
    ///
    /// Returns false if the property cannot be deleted.
    fn delete_property(
        &self,
        activation: &mut Activation<'_, 'gc>,
        multiname: &Multiname<'gc>,
    ) -> Result<bool, Error<'gc>> {
        if self.as_primitive().is_some() {
            return Err(error::make_reference_error(
                activation,
                error::ReferenceErrorCode::InvalidDelete,
                multiname,
                self.instance_of(),
            ));
        }

        match self.vtable().and_then(|vtable| vtable.get_trait(multiname)) {
            None => {
                if self
                    .instance_of_class_definition()
                    .map(|c| c.read().is_sealed())
                    .unwrap_or(false)
                {
                    Ok(false)
                } else {
                    self.delete_property_local(activation, multiname)
                }
            }
            _ => {
                // Similar to the get_property special case for XML/XMLList.
                if (self.as_xml_object().is_some() || self.as_xml_list_object().is_some())
                    && multiname.contains_public_namespace()
                {
                    return self.delete_property_local(activation, multiname);
                }
                Ok(false)
            }
        }
    }

    /// Same as delete_property, but constructs a public Multiname for you.
    fn delete_public_property(
        &self,
        activation: &mut Activation<'_, 'gc>,
        name: impl Into<AvmString<'gc>>,
    ) -> Result<bool, Error<'gc>> {
        let name = Multiname::new(activation.avm2().public_namespace, name);
        self.delete_property(activation, &name)
    }

    /// Retrieve the `__proto__` of a given object.
    ///
    /// The proto is another object used to resolve methods across a class of
    /// multiple objects. It should also be accessible as `__proto__` from
    /// `get`.
    fn proto(&self) -> Option<Object<'gc>> {
        let base = self.base();

        base.proto()
    }

    /// Change the `__proto__` on this object.
    ///
    /// This method primarily exists so that the global scope that player
    /// globals loads into can be created before its superclasses are. It
    /// should be used sparingly, if at all.
    fn set_proto(self, mc: MutationContext<'gc, '_>, proto: Object<'gc>) {
        let mut base = self.base_mut(mc);

        base.set_proto(proto)
    }

    /// Get the next enumerant index in enumerant space.
    ///
    /// Every object maintains a list of enumerants - key/value pairs indexed
    /// by arbitrary integers. This function allows clients to iterate through
    /// those indexing integers. Indexing starts with zero, and then the caller
    /// repeatedly hands prior results back into this function until it returns
    /// None.
    ///
    /// Repeated calls to this function with prior return values must
    /// eventually return `None`. Furthermore, returning `0`, while valid, is
    /// treated by AVM2 code as signalling `None`.
    fn get_next_enumerant(
        self,
        last_index: u32,
        _activation: &mut Activation<'_, 'gc>,
    ) -> Result<Option<u32>, Error<'gc>> {
        let base = self.base();

        Ok(base.get_next_enumerant(last_index))
    }

    /// Retrieve a given enumerable name by index.
    ///
    /// Enumerants are listed by index, starting from zero and iterated via
    /// `get_next_enumerant`. Only enumerants returned by that function are
    /// valid here. A value of `None` indicates that no enumerant with that
    /// index exists.
    fn get_enumerant_name(
        self,
        index: u32,
        _activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        let base = self.base();

        Ok(base.get_enumerant_name(index).unwrap_or(Value::Undefined))
    }

    /// Retrieve a given enumerable value by index.
    ///
    /// This default implementation of value retrieval assumes that the names
    /// of enumerants are also valid local names in the public namespace.
    fn get_enumerant_value(
        self,
        index: u32,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        let name = self
            .get_enumerant_name(index, activation)?
            .coerce_to_string(activation)?;
        // todo: this probably doesn't need non-public accesses
        self.get_public_property(name, activation)
    }

    /// Determine if a property is currently enumerable.
    ///
    /// Properties that do not exist are also not enumerable.
    fn property_is_enumerable(&self, name: AvmString<'gc>) -> bool {
        let base = self.base();

        base.property_is_enumerable(name)
    }

    /// Mark a dynamic property on this object as enumerable.
    fn set_local_property_is_enumerable(
        &self,
        mc: MutationContext<'gc, '_>,
        name: AvmString<'gc>,
        is_enumerable: bool,
    ) {
        let mut base = self.base_mut(mc);

        base.set_local_property_is_enumerable(name, is_enumerable)
    }

    /// Install a bound method on an object.
    fn install_bound_method(
        &mut self,
        mc: MutationContext<'gc, '_>,
        disp_id: u32,
        function: FunctionObject<'gc>,
    ) {
        let mut base = self.base_mut(mc);

        base.install_bound_method(disp_id, function)
    }

    /// Install a const trait on the global object.
    /// This should only ever be called on the `global` object, during initialization.
    fn install_const_late(
        &mut self,
        mc: MutationContext<'gc, '_>,
        name: QName<'gc>,
        value: Value<'gc>,
        class: ClassObject<'gc>,
    ) {
        let new_slot_id = self
            .vtable()
            .unwrap()
            .install_const_trait_late(mc, name, value, class);
        self.base_mut(mc)
            .install_const_slot_late(new_slot_id, value);
    }

    fn install_instance_slots(&mut self, mc: MutationContext<'gc, '_>) {
        self.base_mut(mc).install_instance_slots();
    }

    /// Call the object.
    fn call(
        self,
        _receiver: Value<'gc>,
        _arguments: &[Value<'gc>],
        _activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        Err("Object is not callable".into())
    }

    /// Construct a Class or Function and return an instance of it.
    ///
    /// As the first step in object construction, the `construct` method is
    /// called on the class object to produce an instance of that class. The
    /// constructor is then expected to perform the following steps, in order:
    ///
    /// 1. Allocate the instance object. For ES4 classes, the class's instance
    /// allocator is used to allocate the object. ES3-style classes use the
    /// prototype to derive instances.
    /// 2. Associate the instance object with the class's explicit `prototype`.
    /// 3. If the class has instance traits, install them at this time.
    /// 4. Call the constructor method with the newly-allocated object as
    /// receiver. For ES3 classes, this is just the function's associated
    /// method.
    /// 5. Yield the allocated object. (The return values of constructors are
    /// ignored.)
    fn construct(
        self,
        _activation: &mut Activation<'_, 'gc>,
        _args: &[Value<'gc>],
    ) -> Result<Object<'gc>, Error<'gc>> {
        Err("Object is not constructable".into())
    }

    /// Construct a property of this object by Multiname lookup.
    ///
    /// This corresponds directly to the AVM2 operation `constructprop`.
    fn construct_prop(
        self,
        multiname: &Multiname<'gc>,
        args: &[Value<'gc>],
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Object<'gc>, Error<'gc>> {
        let ctor = self.get_property(multiname, activation)?.as_callable(
            activation,
            Some(multiname),
            Some(Value::from(self.into())),
        )?;

        ctor.construct(activation, args)
    }

    /// Construct a parameterization of this particular type and return it.
    ///
    /// This is called specifically to parameterize generic types, of which
    /// only one exists: `Vector`. When `Vector` is applied with a given
    /// parameter, a new type is returned which can be used to construct
    /// `Vector`s of that type.
    ///
    /// If the object is not a parameterized type, this yields an error. In
    /// practice, this means only `Vector` can use this method. Parameters must
    /// be class objects or `null`, which indicates any type.
    ///
    /// When a given type is parameterized with the same parameters multiple
    /// times, each application must return the same object. This is because
    /// each application has a separate prototype that accepts dynamic
    /// parameters.
    fn apply(
        &self,
        _activation: &mut Activation<'_, 'gc>,
        _param: Value<'gc>,
    ) -> Result<ClassObject<'gc>, Error<'gc>> {
        Err("Not a parameterized type".into())
    }

    /// Determine the type of primitive coercion this object would prefer, in
    /// the case that there is no obvious reason to prefer one type over the
    /// other.
    ///
    /// All native ECMAScript objects prefer numerical coercions, except `Date`,
    /// which wants string coercions.
    fn default_hint(&self) -> Hint {
        Hint::Number
    }

    /// Implement the result of calling `Object.prototype.toString` on this
    /// object class.
    ///
    /// `toString` is a method used to request an object be coerced to a string
    /// value. The default implementation is stored here. User-specified string
    /// coercions happen by defining `toString` in a downstream class or
    /// prototype; this is then picked up by the VM runtime when doing
    /// coercions.
    fn to_string(&self, activation: &mut Activation<'_, 'gc>) -> Result<Value<'gc>, Error<'gc>> {
        let class_name = self
            .instance_of_class_definition()
            .map(|c| c.read().name().local_name())
            .unwrap_or_else(|| "Object".into());

        Ok(AvmString::new_utf8(
            activation.context.gc_context,
            format!("[object {class_name}]"),
        )
        .into())
    }

    /// Implement the result of calling `Object.prototype.toLocaleString` on this
    /// object class.
    ///
    /// `toLocaleString` is a method used to request an object be coerced to a
    /// locale-dependent string value. The default implementation appears to
    /// generate a debug-style string based on the name of the class this
    /// object is, in the format of `[object Class]` (where `Class` is the name
    /// of the class that created this object).
    fn to_locale_string(
        &self,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Value<'gc>, Error<'gc>> {
        let class_name = self
            .instance_of_class_definition()
            .map(|c| c.read().name().local_name())
            .unwrap_or_else(|| "Object".into());

        Ok(AvmString::new_utf8(
            activation.context.gc_context,
            format!("[object {class_name}]"),
        )
        .into())
    }

    /// Implement the result of calling `Object.prototype.valueOf` on this
    /// object class.
    ///
    /// `valueOf` is a method used to request an object be coerced to a
    /// primitive value. Typically, this would be a number of some kind.
    fn value_of(&self, mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error<'gc>>;

    /// Determine if this object is an instance of a given type.
    ///
    /// This uses the ES3 definition of instance, which walks the prototype
    /// chain. For the ES4 definition of instance, use `is_of_type`, which uses
    /// the class object chain and accounts for interfaces.
    ///
    /// The given object should be the class object for the given type we are
    /// checking against this object. Its prototype will be extracted and
    /// searched in the prototype chain of this object.
    fn is_instance_of(
        &self,
        activation: &mut Activation<'_, 'gc>,
        class: Object<'gc>,
    ) -> Result<bool, Error<'gc>> {
        let type_proto = class
            .get_public_property("prototype", activation)?
            .as_object();

        if let Some(type_proto) = type_proto {
            self.has_prototype_in_chain(type_proto)
        } else {
            Ok(false)
        }
    }

    /// Returns all public properties from this object's vtable, together with their values.
    /// This includes normal fields, const fields, and getter methods
    /// This is used for JSON serialization.
    // FIXME - the order doesn't currently match Flash Player
    fn public_vtable_properties(
        &self,
        activation: &mut Activation<'_, 'gc>,
    ) -> Result<Vec<(AvmString<'gc>, Value<'gc>)>, Error<'gc>> {
        if let Some(vtable) = self.vtable() {
            let mut values = Vec::new();
            for (name, prop) in vtable.public_properties() {
                match prop {
                    Property::Slot { slot_id } | Property::ConstSlot { slot_id } => {
                        values.push((name, self.base().get_slot(slot_id)?));
                    }
                    Property::Virtual { get: Some(get), .. } => {
                        values.push((name, self.call_method(get, &[], activation)?))
                    }
                    _ => {}
                }
            }
            Ok(values)
        } else {
            Ok(Vec::new())
        }
    }

    /// Determine if this object has a given prototype in its prototype chain.
    ///
    /// The given object `type_proto` should be the prototype we are checking
    /// against this object.
    fn has_prototype_in_chain(&self, type_proto: Object<'gc>) -> Result<bool, Error<'gc>> {
        let mut my_proto = self.proto();

        //TODO: Is it a verification error to do `obj instanceof bare_object`?
        while let Some(proto) = my_proto {
            if Object::ptr_eq(proto, type_proto) {
                return Ok(true);
            }

            my_proto = proto.proto()
        }

        Ok(false)
    }

    /// Determine if this object is an instance of a given type.
    ///
    /// This uses the ES4 definition of instance, which walks the class object
    /// chain and accounts for interfaces. For the ES3 definition of instance,
    /// use `is_instance_of`, which uses the prototype chain.
    ///
    /// The given object should be the class object for the given type we are
    /// checking against this object.
    fn is_of_type(
        &self,
        test_class: GcCell<'gc, Class<'gc>>,
        context: &mut UpdateContext<'_, 'gc>,
    ) -> bool {
        let my_class = self.instance_of();

        // ES3 objects are not class instances but are still treated as
        // instances of Object, which is an ES4 class.
        if my_class.is_none()
            && GcCell::ptr_eq(
                test_class,
                context.avm2.classes().object.inner_class_definition(),
            )
        {
            true
        } else if let Some(my_class) = my_class {
            my_class.has_class_in_chain(test_class)
        } else {
            false
        }
    }

    /// Get a raw pointer value for this object.
    fn as_ptr(&self) -> *const ObjectPtr;

    /// Get this object's class, if it has one.
    fn instance_of(&self) -> Option<ClassObject<'gc>> {
        let base = self.base();
        base.instance_of()
    }

    /// Get this object's vtable, if it has one.
    /// Every object with class should have a vtable
    fn vtable(&self) -> Option<VTable<'gc>> {
        let base = self.base();
        base.vtable()
    }

    fn get_bound_method(&self, id: u32) -> Option<FunctionObject<'gc>> {
        let base = self.base();
        base.get_bound_method(id)
    }

    /// Get this object's class's `Class`, if it has one.
    fn instance_of_class_definition(&self) -> Option<GcCell<'gc, Class<'gc>>> {
        self.instance_of().map(|cls| cls.inner_class_definition())
    }

    /// Get this object's class's name, formatted for debug output.
    fn instance_of_class_name(&self, mc: MutationContext<'gc, '_>) -> AvmString<'gc> {
        self.instance_of_class_definition()
            .map(|r| r.read().name().to_qualified_name(mc))
            .unwrap_or_else(|| "<Unknown type>".into())
    }

    fn set_instance_of(&self, mc: MutationContext<'gc, '_>, instance_of: ClassObject<'gc>) {
        let instance_vtable = instance_of.instance_vtable();

        let mut base = self.base_mut(mc);
        base.set_instance_of(instance_of, instance_vtable);
    }

    // Sets a different vtable for object, without changing instance_of.
    fn set_vtable(&self, mc: MutationContext<'gc, '_>, vtable: VTable<'gc>) {
        let mut base = self.base_mut(mc);
        base.set_vtable(vtable);
    }

    // Duplicates the vtable for modification without subclassing
    // Note: this detaches the vtable from the original class.
    fn fork_vtable(&self, mc: MutationContext<'gc, '_>) {
        let mut base = self.base_mut(mc);
        let vtable = base.vtable().unwrap().duplicate(mc);
        base.set_vtable(vtable);
    }

    /// Try to corece this object into a `ClassObject`.
    fn as_class_object(&self) -> Option<ClassObject<'gc>> {
        None
    }

    fn as_function_object(&self) -> Option<FunctionObject<'gc>> {
        None
    }

    /// Get this object's `Executable`, if it has one.
    fn as_executable(&self) -> Option<Ref<Executable<'gc>>> {
        None
    }

    /// Unwrap this object's `Namespace`, if the object is a boxed namespace.
    fn as_namespace(&self) -> Option<Ref<Namespace<'gc>>> {
        None
    }

    fn as_namespace_object(&self) -> Option<NamespaceObject<'gc>> {
        None
    }

    /// Unwrap this object as a `QNameObject`
    fn as_qname_object(self) -> Option<QNameObject<'gc>> {
        None
    }

    fn as_loader_info_object(&self) -> Option<&LoaderInfoObject<'gc>> {
        None
    }

    fn as_array_object(&self) -> Option<ArrayObject<'gc>> {
        None
    }

    /// Unwrap this object as array storage.
    fn as_array_storage(&self) -> Option<Ref<ArrayStorage<'gc>>> {
        None
    }

    /// Unwrap this object as bytearray.
    fn as_bytearray(&self) -> Option<Ref<ByteArrayStorage>> {
        None
    }

    fn as_bytearray_mut(&self, _mc: MutationContext<'gc, '_>) -> Option<RefMut<ByteArrayStorage>> {
        None
    }

    fn as_bytearray_object(&self) -> Option<ByteArrayObject<'gc>> {
        None
    }

    /// Unwrap this object as mutable array storage.
    fn as_array_storage_mut(
        &self,
        _mc: MutationContext<'gc, '_>,
    ) -> Option<RefMut<ArrayStorage<'gc>>> {
        None
    }

    /// Unwrap this object as vector storage.
    fn as_vector_storage(&self) -> Option<Ref<VectorStorage<'gc>>> {
        None
    }

    /// Unwrap this object as mutable vector storage.
    fn as_vector_storage_mut(
        &self,
        _mc: MutationContext<'gc, '_>,
    ) -> Option<RefMut<VectorStorage<'gc>>> {
        None
    }

    /// Get this object's `DisplayObject`, if it has one.
    fn as_display_object(&self) -> Option<DisplayObject<'gc>> {
        None
    }

    /// Associate this object with a display object, if it can support such an
    /// association.
    ///
    /// If not, then this function does nothing.
    fn init_display_object(&self, _context: &mut UpdateContext<'_, 'gc>, _obj: DisplayObject<'gc>) {
    }

    fn init_application_domain(&self, _mc: MutationContext<'gc, '_>, _domain: Domain<'gc>) {
        panic!("Tried to init an application domain on a non-ApplicationDomain object!")
    }

    /// Unwrap this object as an ApplicationDomain.
    fn as_application_domain(&self) -> Option<Domain<'gc>> {
        None
    }

    /// Unwrap this object as an event.
    fn as_event(&self) -> Option<Ref<Event<'gc>>> {
        None
    }

    /// Unwrap this object as a mutable event.
    fn as_event_mut(&self, _mc: MutationContext<'gc, '_>) -> Option<RefMut<Event<'gc>>> {
        None
    }

    /// Unwrap this object as a list of event handlers.
    fn as_dispatch(&self) -> Option<Ref<DispatchList<'gc>>> {
        None
    }

    /// Unwrap this object as a mutable list of event handlers.
    fn as_dispatch_mut(&self, _mc: MutationContext<'gc, '_>) -> Option<RefMut<DispatchList<'gc>>> {
        None
    }

    /// Unwrap this object as an immutable primitive value.
    ///
    /// This function should not be called in cases where a normal `Value`
    /// coercion would do. It *only* accounts for boxed primitives, and not
    /// `valueOf`.
    fn as_primitive(&self) -> Option<Ref<Value<'gc>>> {
        None
    }

    /// Unwrap this object as a mutable primitive value.
    fn as_primitive_mut(&self, _mc: MutationContext<'gc, '_>) -> Option<RefMut<Value<'gc>>> {
        None
    }

    /// Unwrap this object as a regexp.
    fn as_regexp_object(&self) -> Option<RegExpObject<'gc>> {
        None
    }

    /// Unwrap this object as a regexp.
    fn as_regexp(&self) -> Option<Ref<RegExp<'gc>>> {
        None
    }

    /// Unwrap this object as a mutable regexp.
    fn as_regexp_mut(&self, _mc: MutationContext<'gc, '_>) -> Option<RefMut<RegExp<'gc>>> {
        None
    }

    /// Unwrap this object's sound handle.
    fn as_sound_object(self) -> Option<SoundObject<'gc>> {
        None
    }

    /// Unwrap this object's sound instance handle.
    fn as_sound_channel(self) -> Option<SoundChannelObject<'gc>> {
        None
    }

    fn as_bitmap_data(&self) -> Option<BitmapDataWrapper<'gc>> {
        None
    }

    fn as_shader_data(&self) -> Option<ShaderDataObject<'gc>> {
        None
    }

    /// Initialize the bitmap data in this object, if it's capable of
    /// supporting said data.
    ///
    /// This should only be called to initialize the association between an AVM
    /// object and it's associated bitmap data. This association should not be
    /// reinitialized later.
    fn init_bitmap_data(&self, _mc: MutationContext<'gc, '_>, _new_bitmap: BitmapDataWrapper<'gc>) {
    }

    /// Get this objects `DateObject`, if it has one.
    fn as_date_object(&self) -> Option<DateObject<'gc>> {
        None
    }

    /// Get this object as a `DictionaryObject`, if it is one.
    fn as_dictionary_object(self) -> Option<DictionaryObject<'gc>> {
        None
    }

    /// Unwrap this object as a text format.
    fn as_text_format(&self) -> Option<Ref<TextFormat>> {
        None
    }

    /// Unwrap this object as a mutable text format.
    fn as_text_format_mut(&self, _mc: MutationContext<'gc, '_>) -> Option<RefMut<TextFormat>> {
        None
    }

    /// Unwrap this object as an Error.
    fn as_error_object(&self) -> Option<ErrorObject<'gc>> {
        None
    }

    fn as_xml_object(&self) -> Option<XmlObject<'gc>> {
        None
    }

    fn as_xml_list_object(&self) -> Option<XmlListObject<'gc>> {
        None
    }

    fn as_context_3d(&self) -> Option<Context3DObject<'gc>> {
        None
    }

    fn as_index_buffer(&self) -> Option<IndexBuffer3DObject<'gc>> {
        None
    }

    fn as_vertex_buffer(&self) -> Option<VertexBuffer3DObject<'gc>> {
        None
    }

    fn as_program_3d(&self) -> Option<Program3DObject<'gc>> {
        None
    }

    fn as_stage_3d(&self) -> Option<Stage3DObject<'gc>> {
        None
    }

    fn as_texture(&self) -> Option<TextureObject<'gc>> {
        None
    }

    fn as_netstream(self) -> Option<NetStream<'gc>> {
        None
    }

    fn as_socket(&self) -> Option<SocketObject<'gc>> {
        None
    }
}

pub enum ObjectPtr {}

impl<'gc> Object<'gc> {
    pub fn ptr_eq<T: TObject<'gc>>(a: T, b: T) -> bool {
        a.as_ptr() == b.as_ptr()
    }

    #[rustfmt::skip]
    pub fn downgrade(&self) -> WeakObject<'gc> {
        match self {
            Self::ScriptObject(o) => WeakObject::ScriptObject(ScriptObjectWeak(GcCell::downgrade(o.0))),
            Self::FunctionObject(o) => WeakObject::FunctionObject(FunctionObjectWeak(GcCell::downgrade(o.0))),
            Self::PrimitiveObject(o) => WeakObject::PrimitiveObject(PrimitiveObjectWeak(GcCell::downgrade(o.0))),
            Self::NamespaceObject(o) => WeakObject::NamespaceObject(NamespaceObjectWeak(GcCell::downgrade(o.0))),
            Self::ArrayObject(o) => WeakObject::ArrayObject(ArrayObjectWeak(GcCell::downgrade(o.0))),
            Self::StageObject(o) => WeakObject::StageObject(StageObjectWeak(GcCell::downgrade(o.0))),
            Self::DomainObject(o) => WeakObject::DomainObject(DomainObjectWeak(GcCell::downgrade(o.0))),
            Self::EventObject(o) => WeakObject::EventObject(EventObjectWeak(GcCell::downgrade(o.0))),
            Self::DispatchObject(o) => WeakObject::DispatchObject(DispatchObjectWeak(GcCell::downgrade(o.0))),
            Self::XmlObject(o) => WeakObject::XmlObject(XmlObjectWeak(GcCell::downgrade(o.0))),
            Self::XmlListObject(o) => WeakObject::XmlListObject(XmlListObjectWeak(GcCell::downgrade(o.0))),
            Self::RegExpObject(o) => WeakObject::RegExpObject(RegExpObjectWeak(GcCell::downgrade(o.0))),
            Self::ByteArrayObject(o) => WeakObject::ByteArrayObject(ByteArrayObjectWeak(GcCell::downgrade(o.0))),
            Self::LoaderInfoObject(o) => WeakObject::LoaderInfoObject(LoaderInfoObjectWeak(GcCell::downgrade(o.0))),
            Self::ClassObject(o) => WeakObject::ClassObject(ClassObjectWeak(GcCell::downgrade(o.0))),
            Self::VectorObject(o) => WeakObject::VectorObject(VectorObjectWeak(GcCell::downgrade(o.0))),
            Self::SoundObject(o) => WeakObject::SoundObject(SoundObjectWeak(GcCell::downgrade(o.0))),
            Self::SoundChannelObject(o) => WeakObject::SoundChannelObject(SoundChannelObjectWeak(GcCell::downgrade(o.0))),
            Self::BitmapDataObject(o) => WeakObject::BitmapDataObject(BitmapDataObjectWeak(GcCell::downgrade(o.0))),
            Self::DateObject(o) => WeakObject::DateObject(DateObjectWeak(GcCell::downgrade(o.0))),
            Self::DictionaryObject(o) => WeakObject::DictionaryObject(DictionaryObjectWeak(GcCell::downgrade(o.0))),
            Self::QNameObject(o) => WeakObject::QNameObject(QNameObjectWeak(GcCell::downgrade(o.0))),
            Self::TextFormatObject(o) => WeakObject::TextFormatObject(TextFormatObjectWeak(GcCell::downgrade(o.0))),
            Self::ProxyObject(o) => WeakObject::ProxyObject(ProxyObjectWeak(GcCell::downgrade(o.0))),
            Self::ErrorObject(o) => WeakObject::ErrorObject(ErrorObjectWeak(GcCell::downgrade(o.0))),
            Self::Stage3DObject(o) => WeakObject::Stage3DObject(Stage3DObjectWeak(Gc::downgrade(o.0))),
            Self::Context3DObject(o) => WeakObject::Context3DObject(Context3DObjectWeak(Gc::downgrade(o.0))),
            Self::IndexBuffer3DObject(o) => WeakObject::IndexBuffer3DObject(IndexBuffer3DObjectWeak(Gc::downgrade(o.0))),
            Self::VertexBuffer3DObject(o) => WeakObject::VertexBuffer3DObject(VertexBuffer3DObjectWeak(Gc::downgrade(o.0))),
            Self::TextureObject(o) => WeakObject::TextureObject(TextureObjectWeak(Gc::downgrade(o.0))),
            Self::Program3DObject(o) => WeakObject::Program3DObject(Program3DObjectWeak(Gc::downgrade(o.0))),
            Self::NetStreamObject(o) => WeakObject::NetStreamObject(NetStreamObjectWeak(GcCell::downgrade(o.0))),
            Self::ShaderDataObject(o) => WeakObject::ShaderDataObject(ShaderDataObjectWeak(Gc::downgrade(o.0))),
            Self::SocketObject(o) => WeakObject::SocketObject(SocketObjectWeak(Gc::downgrade(o.0)))
        }
    }
}

impl<'gc> PartialEq for Object<'gc> {
    fn eq(&self, other: &Self) -> bool {
        Object::ptr_eq(*self, *other)
    }
}

impl<'gc> Eq for Object<'gc> {}

impl<'gc> Hash for Object<'gc> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state);
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub enum WeakObject<'gc> {
    ScriptObject(ScriptObjectWeak<'gc>),
    FunctionObject(FunctionObjectWeak<'gc>),
    PrimitiveObject(PrimitiveObjectWeak<'gc>),
    NamespaceObject(NamespaceObjectWeak<'gc>),
    ArrayObject(ArrayObjectWeak<'gc>),
    StageObject(StageObjectWeak<'gc>),
    DomainObject(DomainObjectWeak<'gc>),
    EventObject(EventObjectWeak<'gc>),
    DispatchObject(DispatchObjectWeak<'gc>),
    XmlObject(XmlObjectWeak<'gc>),
    XmlListObject(XmlListObjectWeak<'gc>),
    RegExpObject(RegExpObjectWeak<'gc>),
    ByteArrayObject(ByteArrayObjectWeak<'gc>),
    LoaderInfoObject(LoaderInfoObjectWeak<'gc>),
    ClassObject(ClassObjectWeak<'gc>),
    VectorObject(VectorObjectWeak<'gc>),
    SoundObject(SoundObjectWeak<'gc>),
    SoundChannelObject(SoundChannelObjectWeak<'gc>),
    BitmapDataObject(BitmapDataObjectWeak<'gc>),
    DateObject(DateObjectWeak<'gc>),
    DictionaryObject(DictionaryObjectWeak<'gc>),
    QNameObject(QNameObjectWeak<'gc>),
    TextFormatObject(TextFormatObjectWeak<'gc>),
    ProxyObject(ProxyObjectWeak<'gc>),
    ErrorObject(ErrorObjectWeak<'gc>),
    Stage3DObject(Stage3DObjectWeak<'gc>),
    Context3DObject(Context3DObjectWeak<'gc>),
    IndexBuffer3DObject(IndexBuffer3DObjectWeak<'gc>),
    VertexBuffer3DObject(VertexBuffer3DObjectWeak<'gc>),
    TextureObject(TextureObjectWeak<'gc>),
    Program3DObject(Program3DObjectWeak<'gc>),
    NetStreamObject(NetStreamObjectWeak<'gc>),
    ShaderDataObject(ShaderDataObjectWeak<'gc>),
    SocketObject(SocketObjectWeak<'gc>),
}

impl<'gc> WeakObject<'gc> {
    pub fn upgrade(self, mc: MutationContext<'gc, '_>) -> Option<Object<'gc>> {
        Some(match self {
            Self::ScriptObject(o) => ScriptObject(o.0.upgrade(mc)?).into(),
            Self::FunctionObject(o) => FunctionObject(o.0.upgrade(mc)?).into(),
            Self::PrimitiveObject(o) => PrimitiveObject(o.0.upgrade(mc)?).into(),
            Self::NamespaceObject(o) => NamespaceObject(o.0.upgrade(mc)?).into(),
            Self::ArrayObject(o) => ArrayObject(o.0.upgrade(mc)?).into(),
            Self::StageObject(o) => StageObject(o.0.upgrade(mc)?).into(),
            Self::DomainObject(o) => DomainObject(o.0.upgrade(mc)?).into(),
            Self::EventObject(o) => EventObject(o.0.upgrade(mc)?).into(),
            Self::DispatchObject(o) => DispatchObject(o.0.upgrade(mc)?).into(),
            Self::XmlObject(o) => XmlObject(o.0.upgrade(mc)?).into(),
            Self::XmlListObject(o) => XmlListObject(o.0.upgrade(mc)?).into(),
            Self::RegExpObject(o) => RegExpObject(o.0.upgrade(mc)?).into(),
            Self::ByteArrayObject(o) => ByteArrayObject(o.0.upgrade(mc)?).into(),
            Self::LoaderInfoObject(o) => LoaderInfoObject(o.0.upgrade(mc)?).into(),
            Self::ClassObject(o) => ClassObject(o.0.upgrade(mc)?).into(),
            Self::VectorObject(o) => VectorObject(o.0.upgrade(mc)?).into(),
            Self::SoundObject(o) => SoundObject(o.0.upgrade(mc)?).into(),
            Self::SoundChannelObject(o) => SoundChannelObject(o.0.upgrade(mc)?).into(),
            Self::BitmapDataObject(o) => BitmapDataObject(o.0.upgrade(mc)?).into(),
            Self::DateObject(o) => DateObject(o.0.upgrade(mc)?).into(),
            Self::DictionaryObject(o) => DictionaryObject(o.0.upgrade(mc)?).into(),
            Self::QNameObject(o) => QNameObject(o.0.upgrade(mc)?).into(),
            Self::TextFormatObject(o) => TextFormatObject(o.0.upgrade(mc)?).into(),
            Self::ProxyObject(o) => ProxyObject(o.0.upgrade(mc)?).into(),
            Self::ErrorObject(o) => ErrorObject(o.0.upgrade(mc)?).into(),
            Self::Stage3DObject(o) => Stage3DObject(o.0.upgrade(mc)?).into(),
            Self::Context3DObject(o) => Context3DObject(o.0.upgrade(mc)?).into(),
            Self::IndexBuffer3DObject(o) => IndexBuffer3DObject(o.0.upgrade(mc)?).into(),
            Self::VertexBuffer3DObject(o) => VertexBuffer3DObject(o.0.upgrade(mc)?).into(),
            Self::TextureObject(o) => TextureObject(o.0.upgrade(mc)?).into(),
            Self::Program3DObject(o) => Program3DObject(o.0.upgrade(mc)?).into(),
            Self::NetStreamObject(o) => NetStreamObject(o.0.upgrade(mc)?).into(),
            Self::ShaderDataObject(o) => ShaderDataObject(o.0.upgrade(mc)?).into(),
            Self::SocketObject(o) => SocketObject(o.0.upgrade(mc)?).into(),
        })
    }
}
