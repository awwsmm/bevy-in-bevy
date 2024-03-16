use bevy::prelude::*;
use bevy::text::{BreakLineOn, TextLayoutInfo};

// overwrite self with patch if patch is not default
pub trait Patch {
    fn apply(&self, patch: &Self) -> Self;
}

// "blanket implementation" for most types, specialized implementations for others
fn apply<T: Default + PartialEq + Clone>(target: &T, patch: &T) -> T {
    if *patch == T::default() { target.clone() } else { patch.clone() }

}

impl Patch for BackgroundColor {
    fn apply(&self, patch: &Self) -> Self {
        BackgroundColor(apply(&self.0, &patch.0))
    }

    // TODO: open PRs against Bevy to
    //  - derive PartialEq and Eq on BackgroundColor
}

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

    // TODO: open PRs against Bevy to
    //  - derive PartialEq and Eq on BorderColor
    //  - make BorderColor's inner Color an Option<Color>, use Color::NONE instead of Color::WHITE as default
    //  - derive Eq on Color
}

impl Patch for ButtonBundle {
    fn apply(&self, patch: &Self) -> Self {
        ButtonBundle {
            node: self.node.apply(&patch.node),
            button: Button, // marker component
            style: apply(&self.style, &patch.style),
            interaction: apply(&self.interaction, &patch.interaction),
            focus_policy: apply(&self.focus_policy, &patch.focus_policy),
            border_color: self.border_color.apply(&patch.border_color),
            image: self.image.apply(&patch.image),
            transform: apply(&self.transform, &patch.transform),
            global_transform: apply(&self.global_transform, &patch.global_transform),
            visibility: apply(&self.visibility, &patch.visibility),
            inherited_visibility: apply(&self.inherited_visibility, &patch.inherited_visibility),
            view_visibility: apply(&self.view_visibility, &patch.view_visibility),
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

    // TODO: open PRs against Bevy to
    //  - derive PartialEq and Eq on Node
}

impl Patch for NodeBundle {
    fn apply(&self, patch: &Self) -> Self {
        NodeBundle {
            node: self.node.apply(&patch.node),
            style: apply(&self.style, &patch.style),
            background_color: self.background_color.apply(&patch.background_color),
            border_color: self.border_color.apply(&patch.border_color),
            focus_policy: apply(&self.focus_policy, &patch.focus_policy),
            transform: apply(&self.transform, &patch.transform),
            global_transform: apply(&self.global_transform, &patch.global_transform),
            visibility: apply(&self.visibility, &patch.visibility),
            inherited_visibility: apply(&self.inherited_visibility, &patch.inherited_visibility),
            view_visibility: apply(&self.view_visibility, &patch.view_visibility),
            z_index: self.z_index.apply(&patch.z_index),
        }
    }
}

impl Patch for Style {
    fn apply(&self, patch: &Self) -> Self {
        Style {
            display: apply(&self.display, &patch.display),
            position_type: apply(&self.position_type, &patch.position_type),
            overflow: apply(&self.overflow, &patch.overflow),
            direction: apply(&self.direction, &patch.direction),
            left: apply(&self.left, &patch.left),
            right: apply(&self.right, &patch.right),
            top: apply(&self.top, &patch.top),
            bottom: apply(&self.bottom, &patch.bottom),
            width: apply(&self.width, &patch.width),
            height: apply(&self.height, &patch.height),
            min_width: apply(&self.min_width, &patch.min_width),
            min_height: apply(&self.min_height, &patch.min_height),
            max_width: apply(&self.max_width, &patch.max_width),
            max_height: apply(&self.max_height, &patch.max_height),
            aspect_ratio: apply(&self.aspect_ratio, &patch.aspect_ratio),
            align_items: apply(&self.align_items, &patch.align_items),
            justify_items: apply(&self.justify_items, &patch.justify_items),
            align_self: apply(&self.align_self, &patch.align_self),
            justify_self: apply(&self.justify_self, &patch.justify_self),
            align_content: apply(&self.align_content, &patch.align_content),
            justify_content: apply(&self.justify_content, &patch.justify_content),
            margin: apply(&self.margin, &patch.margin),
            padding: apply(&self.padding, &patch.padding),
            border: apply(&self.border, &patch.border),
            flex_direction: apply(&self.flex_direction, &patch.flex_direction),
            flex_wrap: apply(&self.flex_wrap, &patch.flex_wrap),
            flex_grow: apply(&self.flex_grow, &patch.flex_grow),
            flex_shrink: apply(&self.flex_shrink, &patch.flex_shrink),
            flex_basis: apply(&self.flex_basis, &patch.flex_basis),
            row_gap: apply(&self.row_gap, &patch.row_gap),
            column_gap: apply(&self.column_gap, &patch.column_gap),
            grid_auto_flow: apply(&self.grid_auto_flow, &patch.grid_auto_flow),
            grid_template_rows: apply(&self.grid_template_rows, &patch.grid_template_rows),
            grid_template_columns: apply(&self.grid_template_columns, &patch.grid_template_columns),
            grid_auto_rows: apply(&self.grid_auto_rows, &patch.grid_auto_rows),
            grid_auto_columns: apply(&self.grid_auto_columns, &patch.grid_auto_columns),
            grid_row: apply(&self.grid_row, &patch.grid_row),
            grid_column: apply(&self.grid_column, &patch.grid_column),
        }
    }
}

impl Patch for Text {
    fn apply(&self, patch: &Self) -> Self {
        Text {
            sections: self.sections.apply(&patch.sections),
            justify: apply(&self.justify, &patch.justify),
            // BreakLineOn does not implement Default, but the default value of linebreak_behavior
            // in Text is BreakLineOn::WordBoundary
            linebreak_behavior: if patch.linebreak_behavior != BreakLineOn::WordBoundary { patch.linebreak_behavior } else { self.linebreak_behavior },
        }
    }

    // TODO: open PRs against Bevy to
    //  - derive Copy, PartialEq and Eq on Text
}

impl Patch for TextBundle {
    fn apply(&self, patch: &Self) -> Self {
        TextBundle {
            node: self.node.apply(&patch.node),
            style: apply(&self.style, &patch.style),
            text: self.text.apply(&patch.text),
            text_layout_info: self.text_layout_info.apply(&patch.text_layout_info),
            // text_flags: apply(&self.text_flags, &patch.text_flags),
            // calculated_size: apply(&self.calculated_size, &patch.calculated_size),
            focus_policy: apply(&self.focus_policy, &patch.focus_policy),
            transform: apply(&self.transform, &patch.transform),
            global_transform: apply(&self.global_transform, &patch.global_transform),
            visibility: apply(&self.visibility, &patch.visibility),
            inherited_visibility: apply(&self.inherited_visibility, &patch.inherited_visibility),
            view_visibility: apply(&self.view_visibility, &patch.view_visibility),
            z_index: self.z_index.apply(&patch.z_index),
            background_color: self.background_color.apply(&patch.background_color),
            ..default() // FIXME unable to set, clone, copy, etc.: calculated_size, text_flags
        }
    }
}

impl Patch for TextLayoutInfo {
    fn apply(&self, patch: &Self) -> Self {
        TextLayoutInfo {
            glyphs: self.glyphs.apply(&patch.glyphs),
            logical_size: apply(&self.logical_size, &patch.logical_size),
        }
    }

    // TODO: open PRs against Bevy to
    //  - derive Copy, PartialEq and Eq on TextLayoutInfo
}

impl Patch for UiImage {
    fn apply(&self, patch: &Self) -> Self {
        UiImage {
            color: apply(&self.color, &patch.color),
            texture: apply(&self.texture, &patch.texture),
            flip_x: apply(&self.flip_x, &patch.flip_x),
            flip_y: apply(&self.flip_y, &patch.flip_y),
        }
    }

    // TODO: open PRs against Bevy to
    //  - derive Copy, PartialEq and Eq on UiImage
}

// probably unsafe to patch one vector onto another element-wise
// so if the patch is not empty, just use the entire patch vector
impl<T: Clone> Patch for Vec<T> {
    fn apply(&self, patch: &Self) -> Self {
        if !patch.is_empty() { patch.clone() } else { self.clone() }
    }
}

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

    // TODO: open PRs against Bevy to
    //  - derive PartialEq and Eq on ZIndex
}