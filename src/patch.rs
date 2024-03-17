use bevy::prelude::*;
use bevy::text::{BreakLineOn, TextLayoutInfo};

// TODO: open PRs against Bevy to
//  - propose Color::NONE instead of Color::WHITE as default, to better align with CSS defaults

// overwrite self with patch if patch is not default
pub trait Patch {
    fn apply(&self, patch: &Self) -> Self;
}

// "blanket implementation" for most types, specialized implementations for others
fn patch_single_field<T: Default + PartialEq + Clone>(target: &T, patch: &T) -> T {
    if *patch == T::default() { target.clone() } else { patch.clone() }
}

// FIXME once https://github.com/bevyengine/bevy/pull/12532 is merged
//   remove this and use patch_single_field instead
impl Patch for BackgroundColor {
    fn apply(&self, patch: &Self) -> Self {
        BackgroundColor(patch_single_field(&self.0, &patch.0))
    }
}

// FIXME once https://github.com/bevyengine/bevy/pull/12532 is merged
//   remove this and use patch_single_field instead
// BorderColor only has one field, a Color.
// If patch is not the default (white), overwrite self.
impl Patch for BorderColor {
    fn apply(&self, patch: &Self) -> Self {
        let default = Self::default();

        // if patch == default
        if patch.0 == default.0 {
            *self
        } else {
            *patch
        }
    }
}

impl Patch for ButtonBundle {
    fn apply(&self, patch: &Self) -> Self {
        ButtonBundle {
            node: self.node.apply(&patch.node),
            button: Button, // marker component
            style: patch_single_field(&self.style, &patch.style),
            interaction: patch_single_field(&self.interaction, &patch.interaction),
            focus_policy: patch_single_field(&self.focus_policy, &patch.focus_policy),
            border_color: self.border_color.apply(&patch.border_color),
            image: self.image.apply(&patch.image),
            transform: patch_single_field(&self.transform, &patch.transform),
            global_transform: patch_single_field(&self.global_transform, &patch.global_transform),
            visibility: patch_single_field(&self.visibility, &patch.visibility),
            inherited_visibility: patch_single_field(&self.inherited_visibility, &patch.inherited_visibility),
            view_visibility: patch_single_field(&self.view_visibility, &patch.view_visibility),
            z_index: self.z_index.apply(&patch.z_index),
        }
    }
}

// We cannot merge the fields of Node because
//  - there is no public, non-default constructor
//  - the fields are not mutable (non-pub)
//
// So the best we can do is check if patch is the default, and if not, use it
impl Patch for Node {
    fn apply(&self, patch: &Self) -> Self {
        let default = Self::default();

        // if patch == default
        if patch.size() == default.size() &&
            patch.stack_index() == default.stack_index() &&
            patch.unrounded_size() == default.unrounded_size() &&
            patch.outline_width() == default.outline_width() {
            *self
        } else {
            *patch
        }
    }
}

impl Patch for NodeBundle {
    fn apply(&self, patch: &Self) -> Self {
        NodeBundle {
            node: self.node.apply(&patch.node),
            style: patch_single_field(&self.style, &patch.style),
            background_color: self.background_color.apply(&patch.background_color),
            border_color: self.border_color.apply(&patch.border_color),
            focus_policy: patch_single_field(&self.focus_policy, &patch.focus_policy),
            transform: patch_single_field(&self.transform, &patch.transform),
            global_transform: patch_single_field(&self.global_transform, &patch.global_transform),
            visibility: patch_single_field(&self.visibility, &patch.visibility),
            inherited_visibility: patch_single_field(&self.inherited_visibility, &patch.inherited_visibility),
            view_visibility: patch_single_field(&self.view_visibility, &patch.view_visibility),
            z_index: self.z_index.apply(&patch.z_index),
        }
    }
}

impl Patch for Style {
    // TODO rewrite this with a macro
    fn apply(&self, patch: &Self) -> Self {
        Style {
            display: patch_single_field(&self.display, &patch.display),
            position_type: patch_single_field(&self.position_type, &patch.position_type),
            overflow: patch_single_field(&self.overflow, &patch.overflow),
            direction: patch_single_field(&self.direction, &patch.direction),
            left: patch_single_field(&self.left, &patch.left),
            right: patch_single_field(&self.right, &patch.right),
            top: patch_single_field(&self.top, &patch.top),
            bottom: patch_single_field(&self.bottom, &patch.bottom),
            width: patch_single_field(&self.width, &patch.width),
            height: patch_single_field(&self.height, &patch.height),
            min_width: patch_single_field(&self.min_width, &patch.min_width),
            min_height: patch_single_field(&self.min_height, &patch.min_height),
            max_width: patch_single_field(&self.max_width, &patch.max_width),
            max_height: patch_single_field(&self.max_height, &patch.max_height),
            aspect_ratio: patch_single_field(&self.aspect_ratio, &patch.aspect_ratio),
            align_items: patch_single_field(&self.align_items, &patch.align_items),
            justify_items: patch_single_field(&self.justify_items, &patch.justify_items),
            align_self: patch_single_field(&self.align_self, &patch.align_self),
            justify_self: patch_single_field(&self.justify_self, &patch.justify_self),
            align_content: patch_single_field(&self.align_content, &patch.align_content),
            justify_content: patch_single_field(&self.justify_content, &patch.justify_content),
            margin: patch_single_field(&self.margin, &patch.margin),
            padding: patch_single_field(&self.padding, &patch.padding),
            border: patch_single_field(&self.border, &patch.border),
            flex_direction: patch_single_field(&self.flex_direction, &patch.flex_direction),
            flex_wrap: patch_single_field(&self.flex_wrap, &patch.flex_wrap),
            flex_grow: patch_single_field(&self.flex_grow, &patch.flex_grow),
            flex_shrink: patch_single_field(&self.flex_shrink, &patch.flex_shrink),
            flex_basis: patch_single_field(&self.flex_basis, &patch.flex_basis),
            row_gap: patch_single_field(&self.row_gap, &patch.row_gap),
            column_gap: patch_single_field(&self.column_gap, &patch.column_gap),
            grid_auto_flow: patch_single_field(&self.grid_auto_flow, &patch.grid_auto_flow),
            grid_template_rows: patch_single_field(&self.grid_template_rows, &patch.grid_template_rows),
            grid_template_columns: patch_single_field(&self.grid_template_columns, &patch.grid_template_columns),
            grid_auto_rows: patch_single_field(&self.grid_auto_rows, &patch.grid_auto_rows),
            grid_auto_columns: patch_single_field(&self.grid_auto_columns, &patch.grid_auto_columns),
            grid_row: patch_single_field(&self.grid_row, &patch.grid_row),
            grid_column: patch_single_field(&self.grid_column, &patch.grid_column),
        }
    }
}


// FIXME once https://github.com/bevyengine/bevy/pull/12532 is merged
//   remove this and use patch_single_field instead
impl Patch for Text {
    fn apply(&self, patch: &Self) -> Self {
        Text {
            sections: self.sections.apply(&patch.sections),
            justify: patch_single_field(&self.justify, &patch.justify),
            // BreakLineOn does not implement Default, but the default value of linebreak_behavior
            // in Text is BreakLineOn::WordBoundary
            linebreak_behavior: if patch.linebreak_behavior != BreakLineOn::WordBoundary { patch.linebreak_behavior } else { self.linebreak_behavior },
        }
    }
}

impl Patch for TextBundle {
    fn apply(&self, patch: &Self) -> Self {
        TextBundle {
            node: self.node.apply(&patch.node),
            style: patch_single_field(&self.style, &patch.style),
            text: self.text.apply(&patch.text),
            text_layout_info: self.text_layout_info.apply(&patch.text_layout_info),
            // text_flags: apply(&self.text_flags, &patch.text_flags),
            // calculated_size: apply(&self.calculated_size, &patch.calculated_size),
            focus_policy: patch_single_field(&self.focus_policy, &patch.focus_policy),
            transform: patch_single_field(&self.transform, &patch.transform),
            global_transform: patch_single_field(&self.global_transform, &patch.global_transform),
            visibility: patch_single_field(&self.visibility, &patch.visibility),
            inherited_visibility: patch_single_field(&self.inherited_visibility, &patch.inherited_visibility),
            view_visibility: patch_single_field(&self.view_visibility, &patch.view_visibility),
            z_index: self.z_index.apply(&patch.z_index),
            background_color: self.background_color.apply(&patch.background_color),
            ..default() // NOTE: unable to set, clone, copy, etc.: calculated_size, text_flags
        }
    }
}

impl Patch for TextLayoutInfo {
    fn apply(&self, patch: &Self) -> Self {
        TextLayoutInfo {
            glyphs: self.glyphs.apply(&patch.glyphs),
            logical_size: patch_single_field(&self.logical_size, &patch.logical_size),
        }
    }
}

impl Patch for UiImage {
    fn apply(&self, patch: &Self) -> Self {
        UiImage {
            color: patch_single_field(&self.color, &patch.color),
            texture: patch_single_field(&self.texture, &patch.texture),
            flip_x: patch_single_field(&self.flip_x, &patch.flip_x),
            flip_y: patch_single_field(&self.flip_y, &patch.flip_y),
        }
    }
}

// probably unsafe to patch one vector onto another element-wise
// so if the patch is not empty, just use the entire patch vector
impl<T: Clone> Patch for Vec<T> {
    fn apply(&self, patch: &Self) -> Self {
        if !patch.is_empty() { patch.clone() } else { self.clone() }
    }
}


// FIXME once https://github.com/bevyengine/bevy/pull/12532 is merged
//   remove this and use patch_single_field instead
// If patch is not the default (Local(0)), overwrite self
impl Patch for ZIndex {
    fn apply(&self, patch: &Self) -> Self {

        // if patch == default
        if let ZIndex::Local(0) = patch {
            *self
        } else {
            *patch
        }
    }
}