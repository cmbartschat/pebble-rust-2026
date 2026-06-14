// use core::{cell::RefCell, ffi::CStr, marker::PhantomData, ptr::NonNull};

// use alloc::{rc::Rc, vec::Vec};

// use crate::{
//     Layer,
//     font::Font,
//     layer::ChildLayer,
//     sys::{self, GColor, GRect},
// };

// pub struct TextLayerCreateFailed;

// struct TextLayerInner {
//     raw: NonNull<sys::TextLayer>,
//     text_vec: Vec<u8>,
//     base_layer: NonNull<sys::Layer>,
//     font: Option<Rc<Font>>,
// }

// pub struct TextLayer {
//     handle: Rc<RefCell<TextLayerInner>>,
// }

// impl<'a> ChildLayer for TextLayer {
//     unsafe fn as_ptr(&mut self) -> NonNull<sys::Layer> {
//         self.base_layer
//     }
// }

// impl TextLayer {
//     pub fn new(r: GRect) -> Result<Self, TextLayerCreateFailed> {
//         unsafe {
//             let Some(raw) = NonNull::new(sys::text_layer_create(r)) else {
//                 return Err(TextLayerCreateFailed);
//             };
//             // let Some(base_layer) = LayerHandle::from_ptr(sys::text_layer_get_layer(layer), false)
//             let Some(base_layer) = NonNull::new(sys::text_layer_get_layer(layer)) else {
//                 sys::text_layer_destroy(raw.as_ptr());
//                 return Err(TextLayerCreateFailed);
//             };

//             Ok(Self {
//                 handle: {
//                 raw,
//                 base_layer,
//                 text_vec: alloc::vec![],
//                 font: None,
//                 }
//             })
//         }
//     }

//     pub fn set_font(&mut self, font: &'a Font<'a>) {
//         unsafe { sys::text_layer_set_font(self.inner, font.raw.as_ptr()) };
//     }

//     pub fn set_text(&mut self, text: &str) {
//         self.text_vec.clear();
//         self.text_vec.reserve(text.len() + 1);
//         self.text_vec.extend(text.bytes());
//         self.text_vec.push(0);
//         unsafe { sys::text_layer_set_text(self.inner, self.text_vec.as_ptr()) };
//     }

//     pub fn set_text_c_str(&mut self, text: &'a CStr) {
//         unsafe { sys::text_layer_set_text(self.inner, text.as_ptr()) };
//     }

//     pub fn set_background_color(&mut self, color: GColor) {
//         unsafe { sys::text_layer_set_background_color(self.inner, color) };
//     }

//     pub fn set_text_color(&mut self, color: GColor) {
//         unsafe { sys::text_layer_set_text_color(self.inner, color) };
//     }

//     pub fn get_layer(&self) -> &Layer<'a> {
//         &self.base_layer
//     }

//     pub fn get_layer_mut(&mut self) -> &mut Layer<'a> {
//         &mut self.base_layer
//     }
// }
