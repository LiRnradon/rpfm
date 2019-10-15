//---------------------------------------------------------------------------//
// Copyright (c) 2017-2019 Ismael Gutiérrez González. All rights reserved.
//
// This file is part of the Rusted PackFile Manager (RPFM) project,
// which can be found here: https://github.com/Frodo45127/rpfm.
//
// This file is licensed under the MIT license, which can be found here:
// https://github.com/Frodo45127/rpfm/blob/master/LICENSE.
//---------------------------------------------------------------------------//

/*!
Module with all the submodules for controlling the views of each decodeable PackedFile Type.

This module contains the code to manage the views and actions of each decodeable PackedFile View.
!*/

pub mod image;

use qt_widgets::widget::Widget;

use core::sync::atomic::{AtomicPtr, Ordering};

use crate::packedfile_views::image::slots::PackedFileImageViewSlots;
use crate::utils::create_grid_layout_unsafe;

//-------------------------------------------------------------------------------//
//                              Enums & Structs
//-------------------------------------------------------------------------------//

/// This struct contains the widget of the view of a PackedFile and his info.
pub struct PackedFileView {
    widget: AtomicPtr<Widget>,
    is_preview: bool,
    slots: TheOneSlot,
}

/// One slot to rule them all,
/// One slot to find them,
/// One slot to bring them all
/// and in the darkness bind them.
pub enum TheOneSlot {
    //Table(PackedFileTableViewSlots),
    //Text(PackedFileTextViewSlots),
    Image(PackedFileImageViewSlots),
    //Text(PackedFileTextViewSlots),
    //TreeView(AddFromPackFileViewSlots),
    //Decoder(PackedFileDBDecoder),
    //RigidModel(PackedFileRigidModelDataView),
    None,
}

//-------------------------------------------------------------------------------//
//                             Implementations
//-------------------------------------------------------------------------------//

/// Default implementation for `PackedFileView`.
impl Default for PackedFileView {
    fn default() -> Self {
        let widget = AtomicPtr::new(Widget::new().into_raw());
        create_grid_layout_unsafe(widget.load(Ordering::SeqCst));
        let is_preview = true;
        let slots = TheOneSlot::None;
        Self {
            widget,
            is_preview,
            slots,
        }
    }
}

/// Implementation for `PackedFileView`.
impl PackedFileView {

    /// This function returns a mutable pointer to the `Widget` of the `PackedFileView`.
    pub fn get_mut_widget(&self) -> *mut Widget {
        self.widget.load(Ordering::SeqCst)
    }

    /// This function replaces the widget of the `PackedFileView` with the provided one.
    pub fn set_widget(&self, widget: *mut Widget) {
        self.widget.store(widget, Ordering::SeqCst)
    }

    /// This function returns if the `PackedFileView` is a preview or not.
    pub fn get_is_preview(&self) -> bool {
        self.is_preview
    }

    /// This function allows you to set a `PackedFileView` as a preview or normal view.
    pub fn set_is_preview(&mut self, is_preview: bool) {
        self.is_preview = is_preview;
    }
}
