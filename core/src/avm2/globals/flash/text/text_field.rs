//! `flash.text.TextField` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::globals::flash::display::display_object::initialize_for_allocator;
use crate::avm2::object::{ClassObject, Object, TObject, TextFormatObject};
use crate::avm2::parameters::ParametersExt;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::display_object::{AutoSizeMode, EditText, TDisplayObject, TextSelection};
use crate::html::TextFormat;
use crate::string::AvmString;
use crate::{avm2_stub_getter, avm2_stub_setter};
use swf::Color;

pub fn text_field_allocator<'gc>(
    class: ClassObject<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<Object<'gc>, Error<'gc>> {
    let textfield_cls = activation.avm2().classes().textfield;

    let mut class_object = Some(class);
    let orig_class = class;
    while let Some(class) = class_object {
        if class == textfield_cls {
            let movie = activation.context.swf.clone();
            let display_object =
                EditText::new(&mut activation.context, movie, 0.0, 0.0, 100.0, 100.0).into();
            return initialize_for_allocator(activation, display_object, orig_class);
        }

        if let Some((movie, symbol)) = activation
            .context
            .library
            .avm2_class_registry()
            .class_symbol(class)
        {
            let child = activation
                .context
                .library
                .library_for_movie_mut(movie)
                .instantiate_by_id(symbol, activation.context.gc_context)?;

            return initialize_for_allocator(activation, child, orig_class);
        }
        class_object = class.superclass_object();
    }
    unreachable!("A TextField subclass should have TextField in superclass chain");
}

pub fn get_always_show_selection<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.text.TextField", "alwaysShowSelection");
    Ok(Value::Bool(false))
}

pub fn set_always_show_selection<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.text.TextField", "alwaysShowSelection");
    Ok(Value::Undefined)
}

pub fn get_auto_size<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(match this.autosize() {
            AutoSizeMode::None => "none".into(),
            AutoSizeMode::Left => "left".into(),
            AutoSizeMode::Center => "center".into(),
            AutoSizeMode::Right => "right".into(),
        });
    }

    Ok(Value::Undefined)
}

pub fn set_auto_size<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let value = args.get_string(activation, 0)?;
        this.set_autosize(
            if &value == b"left" {
                AutoSizeMode::Left
            } else if &value == b"center" {
                AutoSizeMode::Center
            } else if &value == b"right" {
                AutoSizeMode::Right
            } else {
                AutoSizeMode::None
            },
            &mut activation.context,
        );
    }

    Ok(Value::Undefined)
}

pub fn get_background<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok((this.has_background()).into());
    }

    Ok(Value::Undefined)
}

pub fn set_background<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let has_background = args.get_bool(0);
        this.set_has_background(activation.context.gc_context, has_background);
    }

    Ok(Value::Undefined)
}

pub fn get_background_color<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.background_color().to_rgb().into());
    }

    Ok(Value::Undefined)
}

pub fn set_background_color<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let rgb = args.get_u32(activation, 0)?;
        let color = Color::from_rgb(rgb, 255);
        this.set_background_color(activation.context.gc_context, color);
    }

    Ok(Value::Undefined)
}

pub fn get_border<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.has_border().into());
    }

    Ok(Value::Undefined)
}

pub fn set_border<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let border = args.get_bool(0);
        this.set_has_border(activation.context.gc_context, border);
    }

    Ok(Value::Undefined)
}

pub fn get_border_color<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.border_color().to_rgb().into());
    }

    Ok(Value::Undefined)
}

pub fn set_border_color<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let rgb = args.get_u32(activation, 0)?;
        let color = Color::from_rgb(rgb, 255);
        this.set_border_color(activation.context.gc_context, color);
    }

    Ok(Value::Undefined)
}

pub fn get_condense_white<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.text.TextField", "condenseWhite");
    Ok(Value::Bool(false))
}

pub fn set_condense_white<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.text.TextField", "condenseWhite");
    Ok(Value::Undefined)
}

pub fn get_default_text_format<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(TextFormatObject::from_text_format(activation, this.new_text_format())?.into());
    }

    Ok(Value::Undefined)
}

pub fn set_default_text_format<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let new_text_format = args.get(0).unwrap_or(&Value::Undefined).as_object();

        if let Some(new_text_format) = new_text_format {
            if let Some(new_text_format) = new_text_format.as_text_format() {
                this.set_new_text_format(new_text_format.clone(), &mut activation.context);
            }
        }
    }

    Ok(Value::Undefined)
}

pub fn get_display_as_password<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.is_password().into());
    }

    Ok(Value::Undefined)
}

pub fn set_display_as_password<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let is_password = args.get_bool(0);

        this.set_password(is_password, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_embed_fonts<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok((!this.is_device_font()).into());
    }

    Ok(Value::Undefined)
}

pub fn set_embed_fonts<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let is_embed_fonts = args.get_bool(0);

        this.set_is_device_font(&mut activation.context, !is_embed_fonts);
    }

    Ok(Value::Undefined)
}

pub fn get_html_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(AvmString::new(activation.context.gc_context, this.html_text()).into());
    }

    Ok(Value::Undefined)
}

pub fn set_html_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let html_text = args.get_string(activation, 0)?;

        this.set_is_html(&mut activation.context, true);
        this.set_html_text(&html_text, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_length<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.text_length().into());
    }

    Ok(Value::Undefined)
}

pub fn get_multiline<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.is_multiline().into());
    }

    Ok(Value::Undefined)
}

pub fn set_multiline<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let is_multiline = args.get_bool(0);

        this.set_multiline(is_multiline, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_selectable<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.is_selectable().into());
    }

    Ok(Value::Undefined)
}

pub fn set_selectable<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let is_selectable = args.get_bool(0);

        this.set_selectable(is_selectable, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(AvmString::new(activation.context.gc_context, this.text()).into());
    }

    Ok(Value::Undefined)
}

pub fn set_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let text = args.get_string(activation, 0)?;

        this.set_is_html(&mut activation.context, false);
        this.set_text(&text, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_text_color<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        if let Some(color) = this.new_text_format().color {
            return Ok(color.to_rgb().into());
        } else {
            return Ok(0u32.into());
        }
    }

    Ok(Value::Undefined)
}

pub fn set_text_color<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let text_color = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_u32(activation)?;
        let desired_format = TextFormat {
            color: Some(swf::Color::from_rgb(text_color, 0xFF)),
            ..TextFormat::default()
        };

        this.set_text_format(
            0,
            this.text_length(),
            desired_format.clone(),
            &mut activation.context,
        );
        this.set_new_text_format(desired_format, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_text_height<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let metrics = this.measure_text(&mut activation.context);
        return Ok(metrics.1.to_pixels().into());
    }

    Ok(Value::Undefined)
}

pub fn get_text_width<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let metrics = this.measure_text(&mut activation.context);
        return Ok(metrics.0.to_pixels().into());
    }

    Ok(Value::Undefined)
}

pub fn get_type<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        match this.is_editable() {
            true => return Ok("input".into()),
            false => return Ok("dynamic".into()),
        }
    }

    Ok(Value::Undefined)
}

pub fn set_type<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let is_editable = args.get_string(activation, 0)?;

        if &is_editable == b"input" {
            this.set_editable(true, &mut activation.context);
        } else if &is_editable == b"dynamic" {
            this.set_editable(false, &mut activation.context);
        } else {
            return Err(format!("Invalid TextField.type: {is_editable}").into());
        }
    }

    Ok(Value::Undefined)
}

pub fn get_word_wrap<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.is_word_wrap().into());
    }

    Ok(Value::Undefined)
}

pub fn set_word_wrap<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let is_word_wrap = args.get_bool(0);

        this.set_word_wrap(is_word_wrap, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn append_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let new_text = args.get_string(activation, 0)?;
        let existing_length = this.text_length();

        this.replace_text(
            existing_length,
            existing_length,
            &new_text,
            &mut activation.context,
        );
    }

    Ok(Value::Undefined)
}

pub fn get_text_format<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let mut begin_index = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Integer(-1))
            .coerce_to_i32(activation)?;
        let mut end_index = args
            .get(1)
            .cloned()
            .unwrap_or(Value::Integer(-1))
            .coerce_to_i32(activation)?;

        if begin_index < 0 {
            begin_index = 0;
        }

        if end_index < 0 {
            end_index = this.text_length() as i32;
        }

        let tf = this.text_format(begin_index as usize, end_index as usize);
        return Ok(TextFormatObject::from_text_format(activation, tf)?.into());
    }

    Ok(Value::Undefined)
}

pub fn replace_selected_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let value = args.get_string(activation, 0)?;
        let selection = this
            .selection()
            .unwrap_or_else(|| TextSelection::for_position(0));

        this.replace_text(
            selection.start(),
            selection.end(),
            &value,
            &mut activation.context,
        );
    }

    Ok(Value::Undefined)
}

pub fn replace_text<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let begin_index = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_u32(activation)?;
        let end_index = args
            .get(1)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_u32(activation)?;
        let value = args.get_string(activation, 2)?;

        this.replace_text(
            begin_index as usize,
            end_index as usize,
            &value,
            &mut activation.context,
        );
    }

    Ok(Value::Undefined)
}

pub fn get_caret_index<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return if let Some(selection) = this.selection() {
            Ok(selection.to().into())
        } else {
            Ok(0.into())
        };
    }

    Ok(Value::Undefined)
}

pub fn get_selection_begin_index<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return if let Some(selection) = this.selection() {
            Ok(selection.start().into())
        } else {
            Ok(0.into())
        };
    }

    Ok(Value::Undefined)
}

pub fn get_selection_end_index<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return if let Some(selection) = this.selection() {
            Ok(selection.end().into())
        } else {
            Ok(0.into())
        };
    }

    Ok(Value::Undefined)
}

pub fn set_selection<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let begin_index = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_u32(activation)?;
        let end_index = args
            .get(1)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_u32(activation)?;

        this.set_selection(
            Some(TextSelection::for_range(
                begin_index as usize,
                end_index as usize,
            )),
            activation.context.gc_context,
        );
    }

    Ok(Value::Undefined)
}

pub fn set_text_format<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let tf = args.get(0).unwrap_or(&Value::Undefined).as_object();
        if let Some(tf) = tf {
            if let Some(tf) = tf.as_text_format() {
                let mut begin_index = args
                    .get(1)
                    .unwrap_or(&(-1).into())
                    .coerce_to_i32(activation)?;
                let mut end_index = args
                    .get(2)
                    .unwrap_or(&(-1).into())
                    .coerce_to_i32(activation)?;

                if begin_index < 0 {
                    begin_index = 0;
                }

                if begin_index as usize > this.text_length() {
                    return Err("RangeError: The supplied index is out of bounds.".into());
                }

                if end_index < 0 {
                    end_index = this.text_length() as i32;
                }

                if end_index as usize > this.text_length() {
                    return Err("RangeError: The supplied index is out of bounds.".into());
                }

                this.set_text_format(
                    begin_index as usize,
                    end_index as usize,
                    tf.clone(),
                    &mut activation.context,
                );
            }
        }
    }

    Ok(Value::Undefined)
}

pub fn get_anti_alias_type<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return if this.render_settings().is_advanced() {
            Ok("advanced".into())
        } else {
            Ok("normal".into())
        };
    }

    Ok(Value::Undefined)
}

pub fn set_anti_alias_type<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let old_settings = this.render_settings();
        let new_type = args.get_string(activation, 0)?;

        if &new_type == b"advanced" {
            this.set_render_settings(
                activation.context.gc_context,
                old_settings.with_advanced_rendering(),
            );
        } else if &new_type == b"normal" {
            this.set_render_settings(
                activation.context.gc_context,
                old_settings.with_normal_rendering(),
            );
        }
    }
    Ok(Value::Undefined)
}

pub fn get_grid_fit_type<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return match this.render_settings().grid_fit() {
            swf::TextGridFit::None => Ok("none".into()),
            swf::TextGridFit::Pixel => Ok("pixel".into()),
            swf::TextGridFit::SubPixel => Ok("subpixel".into()),
        };
    }

    Ok(Value::Undefined)
}

pub fn set_grid_fit_type<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let old_settings = this.render_settings();
        let new_type = args.get_string(activation, 0)?;

        if &new_type == b"pixel" {
            this.set_render_settings(
                activation.context.gc_context,
                old_settings.with_grid_fit(swf::TextGridFit::Pixel),
            );
        } else if &new_type == b"subpixel" {
            this.set_render_settings(
                activation.context.gc_context,
                old_settings.with_grid_fit(swf::TextGridFit::SubPixel),
            );
        } else {
            //NOTE: In AS3 invalid values are treated as None.
            this.set_render_settings(
                activation.context.gc_context,
                old_settings.with_grid_fit(swf::TextGridFit::None),
            );
        }
    }
    Ok(Value::Undefined)
}

pub fn get_thickness<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.render_settings().thickness().into());
    }

    Ok(0.into())
}

pub fn set_thickness<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let old_settings = this.render_settings();
        let mut new_thickness = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_number(activation)?;

        // NOTE: The thickness clamp is ONLY enforced on AS3.
        new_thickness = new_thickness.clamp(-200.0, 200.0);

        this.set_render_settings(
            activation.context.gc_context,
            old_settings.with_thickness(new_thickness as f32),
        );
    }

    Ok(Value::Undefined)
}

pub fn get_sharpness<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.render_settings().sharpness().into());
    }

    Ok(0.into())
}

pub fn set_sharpness<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let old_settings = this.render_settings();
        let mut new_sharpness = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_number(activation)?;

        // NOTE: The sharpness clamp is only enforced on AS3.
        new_sharpness = new_sharpness.clamp(-400.0, 400.0);

        this.set_render_settings(
            activation.context.gc_context,
            old_settings.with_sharpness(new_sharpness as f32),
        );
    }

    Ok(Value::Undefined)
}

pub fn get_num_lines<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.layout_lines().into());
    }

    Ok(Value::Undefined)
}

pub fn get_line_metrics<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let line_num = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_i32(activation)?;
        let metrics = this.layout_metrics(Some(line_num as usize));

        if let Some(metrics) = metrics {
            let metrics_class = activation.avm2().classes().textlinemetrics;
            return Ok(metrics_class
                .construct(
                    activation,
                    &[
                        metrics.x.to_pixels().into(),
                        metrics.width.to_pixels().into(),
                        metrics.height.to_pixels().into(),
                        metrics.ascent.to_pixels().into(),
                        metrics.descent.to_pixels().into(),
                        metrics.leading.to_pixels().into(),
                    ],
                )?
                .into());
        } else {
            return Err("RangeError".into());
        }
    }

    Ok(Value::Undefined)
}

pub fn get_bottom_scroll_v<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.bottom_scroll().into());
    }

    Ok(Value::Undefined)
}

pub fn get_max_scroll_v<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.maxscroll().into());
    }

    Ok(Value::Undefined)
}

pub fn get_max_scroll_h<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.maxhscroll().into());
    }

    Ok(Value::Undefined)
}

pub fn get_scroll_v<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.scroll().into());
    }

    Ok(Value::Undefined)
}

pub fn set_scroll_v<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let input = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_i32(activation)?;
        this.set_scroll(input as f64, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_scroll_h<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.hscroll().into());
    }

    Ok(Value::Undefined)
}

pub fn set_scroll_h<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        // NOTE: The clamping behavior here is identical to AVM1.
        // This is incorrect, SWFv9 uses more complex behavior and AS3 can only
        // be present in v9 SWFs.
        let input = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_i32(activation)?;
        let clamped = input.clamp(0, this.maxhscroll() as i32);
        this.set_hscroll(clamped as f64, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_max_chars<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        return Ok(this.max_chars().into());
    }

    Ok(Value::Undefined)
}

pub fn set_max_chars<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this
        .as_display_object()
        .and_then(|this| this.as_edit_text())
    {
        let input = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_i32(activation)?;
        this.set_max_chars(input, &mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_mouse_wheel_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.text.TextField", "mouseWheelEnabled");
    Ok(true.into())
}

pub fn set_mouse_wheel_enabled<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.text.TextField", "mouseWheelEnabled");
    Ok(Value::Undefined)
}

pub fn get_restrict<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.text.TextField", "restrict");
    Ok(Value::Null)
}

pub fn set_restrict<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.text.TextField", "restrict");
    Ok(Value::Undefined)
}
