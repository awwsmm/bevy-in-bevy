use bevy::prelude::*;
use bevy::text::{BreakLineOn, TextLayoutInfo};
use bevy::window::WindowResized;

use crate::{css, Link, when_debugging};

// literal --css-variables and also responsive CSS (usually depends on @media(min-width))
#[derive(Resource)]
pub struct CSS {
    pub(crate) rem: f32,
    pub(crate) header_message_font_size: f32, // depends on @media(min-width)
    pub(crate) logo_height: Val, // depends on @media(min-width)
    pub(crate) header_height: Val, // var(--header-height), depends on @media(min-width)
    pub(crate) header_padding: UiRect, // depends on @media(min-width)
    pub(crate) fira_sans: Handle<Font>,
    pub(crate) github_img_height: Val, // depends on @media(min-width)
}

impl Default for CSS {
    fn default() -> Self {
        Self {
            rem: 19.5,
            header_message_font_size: 1.3 * 19.5, // 1.3 * rem
            logo_height: Val::Px(28.),
            header_height: Val::Px(60.),
            header_padding: UiRect::right(Val::Px(8.)),
            fira_sans: Handle::default(),
            github_img_height: Val::Px(35.),
        }
    }
}

#[derive(PartialEq)]
pub(crate) enum Class {
    LayoutHeader, // layout__header
    HeaderLogo, // header__logo
    MainMenuLink, // main-menu__link
    HeaderContent, // header__content
    HeaderLeftBlock, // header__left-block
    Logo, // logo
    HeaderMessage, // header__message
    MainMenuEntry, // main-menu__entry
    MainMenuLinkText, // main-menu__link
    HeaderCtaContainer, // header__cta-container
    ButtonPink, // button--pink
    Button, // button
    ButtonIcon, // button__icon
    ButtonText, // button
    HeaderCtaGitHub, // header__cta--github
    HeaderCtaGitHubImg, // header__cta--github > img
}

impl CSS {
    fn style(&self, class: &Class) -> Style {
        match class {
            Class::LayoutHeader => {
                // .layout__header {
                //     position: fixed;
                //     top: 0px;
                //     width: 100%;
                //     height: var(--header-height);
                //     background-color: #1e1e22; // FIXME part of the NodeBundle, not the Style
                //     border-bottom: 2px solid #2c2c2d; // FIXME color is part of the NodeBundle, not the Style; 'solid' not present
                //     z-index: 800;
                // }
                Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    width: Val::Percent(100.),
                    height: self.header_height,
                    border: UiRect::bottom(Val::Px(2.0)),
                    padding: self.header_padding,
                    ..default()
                }
            }
            Class::HeaderLogo => {
                // .header__logo {
                //     display: flex;
                //     align-items: center;
                // }
                Style {
                    align_items: AlignItems::Center,
                    ..default()
                }
            }
            Class::MainMenuLink => {
                // .main-menu__link {
                //     display: flex;
                //     align-items: center;
                //     justify-content: center;
                //     padding: 0 8px;
                //     height: var(--header-height);
                //     font-size: 1.3rem;
                //     font-weight: 500;
                //     text-decoration: none;
                // }
                Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::new(Val::Px(7.), Val::Px(7.), Val::Px(0.), Val::Px(0.)), // FIXME font is too wide, reduce padding to compensate
                    height: self.header_height,
                    border: when_debugging(UiRect::all(Val::Px(1.))),
                    ..default()
                }
            }
            Class::HeaderContent => {
                // .header__content {
                //     width: 100%;
                //     max-width: 1200px;
                //     height: var(--header-height);
                //     display: flex;
                //     align-items: center;
                //     justify-content: space-between;
                //     margin: 0 auto;
                // }
                Style {
                    width: Val::Percent(100.),
                    max_width: Val::Px(1200.),
                    height: self.header_height,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::ZERO, Val::ZERO),
                    ..default()
                }
            }
            Class::HeaderLeftBlock => {
                // .header__left-block {
                //     display: flex;
                //     align-items: center;
                //     margin-right: auto;
                // }
                Style {
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Auto),
                    // height: Val::Percent(100.),
                    // width: Val::Px(520.0), // required by child node
                    ..default()
                }
            }
            Class::Logo => {
                // .logo {
                //     height: 28px;
                //     width: auto;
                // }
                Style {
                    height: self.logo_height,
                    width: Val::Auto,
                    ..default()
                }
            }
            Class::HeaderMessage => {
                // .header__message {
                //     display: none;
                //     font-size: 1.28rem;
                //     font-weight: 500;
                //     color: #868686;
                //     margin-left: 12px;
                //     white-space: nowrap;
                // }
                Style {
                    margin: UiRect::left(Val::Px(12.)),
                    ..default()
                }
            }
            Class::MainMenuEntry => {
                Style {
                    border: when_debugging(UiRect::all(Val::Px(1.))),
                    ..default()
                }
            }
            Class::HeaderCtaContainer => {
                // .header__cta-container {
                //     display: flex;
                //     align-items: center;
                //     margin-left: 16px;
                // }
                Style {
                    align_items: AlignItems::Center,
                    margin: UiRect::left(Val::Px(16.)),
                    ..default()
                }
            }
            Class::Button => {
                // .button {
                //     display: inline-flex;
                //     align-items: center;
                //     background-color: #4a6e91;
                //     border: 3px solid #6a8fb3;
                //     padding: 6px 8px;
                //     border-radius: 10px;
                //     font-size: 1.2rem;
                //     font-weight: 500;
                //     transition: transform 100ms;
                // }
                Style {
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(3.)),
                    padding: UiRect::px(8., 8., 6., 6.),
                    ..default()
                }
            }
            Class::ButtonPink => {
                // .header__cta:not(:last-child) {
                //     margin-right: 8px;
                // }
                Style {
                    margin: UiRect::right(Val::Px(8.)),
                    ..default()
                }
            }
            Class::ButtonIcon => {
                // .button__icon {
                //     height: 1.1em;
                //     width: auto;
                //     vertical-align: middle;
                //     margin-left: 0.2em;
                //     font-size: .9em;
                // }
                Style {
                    height: Val::Px(1.1 * 17.), // TODO define em
                    margin: UiRect::left(Val::Px(0.2 * self.rem)),
                    ..default()
                }
            }
            Class::HeaderCtaGitHub => {
                // .header__cta--github {
                //     display: flex;
                //     align-items: center;
                //     justify-content: center;
                // }
                Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                }
            }
            Class::HeaderCtaGitHubImg => {
                // .header__cta--github img {
                //     height: 35px;
                // }
                Style {
                    height: self.github_img_height,
                    width: Val::Auto,
                    ..default()
                }
            }
            _ => Style::default()
        }
    }

    // FIXME background_color and border_color are fields on NodeBundle, rather than on Style
    //       so to translate from CSS to Bevy, we need to specify styling in multiple places
    pub(crate) fn node_style(&self, class: &Class) -> NodeBundle {
        match class {
            Class::LayoutHeader => {
                NodeBundle {
                    background_color: Srgba::hex("#1e1e22").unwrap().into(),
                    border_color: Srgba::hex("#2c2c2d").unwrap().into(),
                    ..default()
                }
            }
            Class::MainMenuEntry => {
                NodeBundle {
                    border_color: when_debugging(Srgba::RED.into()),
                    // transform: Transform::from_scale(Vec3::new(0.97, 1.0, 1.0)),
                    ..default()
                }
            }
            _ => NodeBundle::default()
        }
    }

    // FIXME font, font_size, and font color are fields on TextStyle, rather than on Style
    //       so to translate from CSS to Bevy, we need to specify styling in multiple places
    pub(crate) fn text_style(&self, class: &Class) -> TextBundle {
        match class {
            Class::MainMenuLinkText => {
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: self.fira_sans.clone(),
                            font_size: 1.3 * self.rem,
                            color: Srgba::hex("#ececec").unwrap().into(),
                            ..default()
                        }
                    ),
                    transform: Transform::from_scale(Vec3::new(0.97, 1.0, 1.0)),
                    ..default()
                }
            }
            Class::HeaderMessage => {
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: self.fira_sans.clone(),
                            font_size: self.header_message_font_size,
                            color: Srgba::hex("#868686").unwrap().into(),
                            ..default()
                        }
                    ),
                    transform: Transform::from_scale(Vec3::new(0.97, 1.0, 1.0)),
                    ..default()
                }
            }
            Class::ButtonIcon => {
                // .button__icon {
                //     height: 1.1em;
                //     width: auto;
                //     vertical-align: middle;
                //     margin-left: 0.2em;
                //     font-size: .9em;
                // }
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: 0.9 * self.rem,
                            ..default()
                        }
                    ),
                    ..default()
                }
            }
            Class::ButtonText => {
                // .button {
                //     display: inline-flex;
                //     align-items: center;
                //     background-color: #4a6e91;
                //     border: 3px solid #6a8fb3;
                //     padding: 6px 8px;
                //     border-radius: 10px;
                //     font-size: 1.2rem;
                //     font-weight: 500;
                //     transition: transform 100ms;
                // }
                // FIXME Bevy doesn't yet support border-radius
                //       see: https://github.com/bevyengine/bevy/pull/8973
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: self.fira_sans.clone(),
                            font_size: 1.2 * self.rem,
                            ..default()
                        }
                    ),
                    ..default()
                }
            }
            _ => TextBundle::default()
        }
    }

    // FIXME image and border_color are fields on ButtonBundle, rather than on Style
    //       so to translate from CSS to Bevy, we need to specify styling in multiple places
    pub(crate) fn button_style(&self, class: &Class) -> ButtonBundle {
        match class {
            Class::ButtonPink => {
                // .button--pink {
                //     background-color: #9f517a;
                //     border-color: #ba789b;
                // }
                ButtonBundle {
                    image: UiImage::default().with_color(Srgba::hex("#9f517a").unwrap().into()),
                    border_color: Srgba::hex("#ba789b").unwrap().into(),
                    ..default()
                }
            }
            // default button background color is white -- it should be clear
            _ => ButtonBundle {
                image: UiImage::default().with_color(Color::NONE).into(),
                ..default()
            }
            // _ => Default::default()
        }
    }
}

// if patch is default, do not apply it
fn apply_patch_node(patch: Node, onto: Node) -> Node {
    match patch {
        patch if patch.stack_index() == 0 && patch.size() == Vec2::ZERO && patch.outline_width() == 0. && patch.unrounded_size() == Vec2::ZERO => onto,
        _ => patch,
    }
}

// if patch is default, do not apply it
fn apply_patch_border_color(patch: BorderColor, onto: BorderColor) -> BorderColor {
    if patch.0 == BorderColor::default().0 { onto } else { patch }
}

// if patch is default, do not apply it
fn apply_patch_z_index(patch: ZIndex, onto: ZIndex) -> ZIndex {
    match patch {
        ZIndex::Local(0) => onto,
        _ => patch,
    }
}

// if patch is default, do not apply it
pub(crate) fn apply_patch_ui_image(patch: UiImage, onto: UiImage) -> UiImage {
    match patch.clone() {
        UiImage { color, texture, flip_x, flip_y }
            if color == Color::default() && texture == Handle::default() && flip_x == false && flip_y == bool::default() =>
        onto,
        _ => patch,
    }
}

fn apply_patch_text(patch: Text, onto: Text) -> Text {
    match patch.clone() {
        Text { sections, justify, linebreak_behavior }
            if sections.is_empty() && justify == JustifyText::default() && linebreak_behavior == Text::default().linebreak_behavior =>
        onto,
        _ => patch
    }
}

fn apply_patch_text_layout_info(patch: TextLayoutInfo, onto: TextLayoutInfo) -> TextLayoutInfo {
    match patch.clone() {
        TextLayoutInfo { glyphs, logical_size }
        if glyphs.is_empty() && logical_size == Vec2::default() =>
            onto,
        _ => patch
    }
}

fn apply_patch_background_color(patch: BackgroundColor, onto: BackgroundColor) -> BackgroundColor {
    if patch.0 == BackgroundColor::default().0 { onto } else { patch }
}

pub fn overlay_nodes(patch: &NodeBundle, onto: &NodeBundle) -> NodeBundle {
    NodeBundle {
        node: apply_patch_node(patch.node, onto.node),
        // node: match onto.node {
        //     node if node.stack_index() == 0 && node.size() == Vec2::ZERO && node.outline_width() == 0. && node.unrounded_size() == Vec2::ZERO => onto.node,
        //     _ => patch.node,
        // },
        // node: onto.node.unless_non_default(patch.node),
        style: onto.style.clone().unless_non_default(patch.style.clone()),
        background_color: apply_patch_background_color(patch.background_color, onto.background_color),
        // background_color: onto.background_color.unless_non_default(patch.background_color),
        border_color: apply_patch_border_color(patch.border_color, onto.border_color),
        // border_color: if patch.border_color.0 == BorderColor::default().0 { onto.border_color } else { patch.border_color },
        // border_color: onto.border_color.unless_non_default(patch.border_color),
        focus_policy: onto.focus_policy.unless_non_default(patch.focus_policy),
        transform: onto.transform.unless_non_default(patch.transform),
        global_transform: onto.global_transform.unless_non_default(patch.global_transform),
        visibility: onto.visibility.unless_non_default(patch.visibility),
        inherited_visibility: onto.inherited_visibility.unless_non_default(patch.inherited_visibility),
        view_visibility: onto.view_visibility.unless_non_default(patch.view_visibility),
        z_index: apply_patch_z_index(patch.z_index, onto.z_index),
        // z_index: match patch.z_index {
        //     ZIndex::Local(0) => onto.z_index,
        //     _ => patch.z_index,
        // }
        // z_index: onto.z_index.unless_non_default(patch.z_index),
    }
}

pub fn overlay_buttons(patch: &ButtonBundle, onto: &ButtonBundle) -> ButtonBundle {
    ButtonBundle {
        node: apply_patch_node(patch.node, onto.node),
        button: Button, // marker component
        style: onto.style.clone().unless_non_default(patch.style.clone()),
        interaction: onto.interaction.unless_non_default(patch.interaction),
        focus_policy: onto.focus_policy.unless_non_default(patch.focus_policy),
        border_color: apply_patch_border_color(patch.border_color, onto.border_color),
        image: apply_patch_ui_image(patch.image.clone(), onto.image.clone()),
        // image: onto.image.clone().unless_non_default(patch.image.clone()),
        transform: onto.transform.unless_non_default(patch.transform),
        global_transform: onto.global_transform.unless_non_default(patch.global_transform),
        visibility: onto.visibility.unless_non_default(patch.visibility),
        inherited_visibility: onto.inherited_visibility.unless_non_default(patch.inherited_visibility),
        view_visibility: onto.view_visibility.unless_non_default(patch.view_visibility),
        z_index: apply_patch_z_index(patch.z_index, onto.z_index),
    }
}

trait Replaceable<T> {
    fn unless_non_default(self, other: T) -> T;
}

impl <T: Default + PartialEq> Replaceable<T> for T {
    fn unless_non_default(self, other: T) -> T {
        if other == T::default() { self } else { other }
    }
}

pub fn overlay_text(patch: &TextBundle, onto: &TextBundle) -> TextBundle {
    // TextStyle {
    //     font: onto.font.clone().unless_non_default(patch.font.clone()),
    //     font_size: if patch.font_size == 12.0 { onto.font_size } else { patch.font_size },
    //     color: onto.color.clone().unless_non_default(patch.color.clone()),
    // }
    TextBundle {
        node: apply_patch_node(patch.node, onto.node),
        style: onto.style.clone().unless_non_default(patch.style.clone()),
        text: apply_patch_text(patch.text.clone(), onto.text.clone()),
        // text: onto.text.clone().unless_non_default(patch.text.clone()),
        text_layout_info: apply_patch_text_layout_info(patch.text_layout_info.clone(), onto.text_layout_info.clone()),
        // text_layout_info: onto.text_layout_info.clone().unless_non_default(patch.text_layout_info.clone()),
        // text_flags: onto.text_flags.clone().unless_non_default(patch.text_flags.clone()),
        // calculated_size: patch.calculated_size,
        focus_policy: onto.focus_policy.clone().unless_non_default(patch.focus_policy.clone()),
        transform: onto.transform.clone().unless_non_default(patch.transform.clone()),
        global_transform: onto.global_transform.clone().unless_non_default(patch.global_transform.clone()),
        visibility: onto.visibility.clone().unless_non_default(patch.visibility.clone()),
        inherited_visibility: onto.inherited_visibility.clone().unless_non_default(patch.inherited_visibility.clone()),
        view_visibility: onto.view_visibility.clone().unless_non_default(patch.view_visibility.clone()),
        z_index: apply_patch_z_index(patch.z_index, onto.z_index),
        // z_index: onto.z_index.clone().unless_non_default(patch.z_index.clone()),
        background_color: apply_patch_background_color(patch.background_color, onto.background_color),
        // background_color: onto.background_color.clone().unless_non_default(patch.background_color.clone()),
        ..default() // FIXME unable to set, clone, copy, etc.: calculated_size, text_flags
    }
}

#[derive(Component)]
pub(crate) struct Classes(pub Vec<Class>);

impl Classes {
    fn overlay(patch: &Style, onto: &Style) -> Style {
        Style {
            display: onto.display.unless_non_default(patch.display),
            position_type: onto.position_type.unless_non_default(patch.position_type),
            overflow: onto.overflow.unless_non_default(patch.overflow),
            direction: onto.direction.unless_non_default(patch.direction),
            left: onto.left.unless_non_default(patch.left),
            right: onto.right.unless_non_default(patch.right),
            top: onto.top.unless_non_default(patch.top),
            bottom: onto.bottom.unless_non_default(patch.bottom),
            width: onto.width.unless_non_default(patch.width),
            height: onto.height.unless_non_default(patch.height),
            min_width: onto.min_width.unless_non_default(patch.min_width),
            min_height: onto.min_height.unless_non_default(patch.min_height),
            max_width: onto.max_width.unless_non_default(patch.max_width),
            max_height: onto.max_height.unless_non_default(patch.max_height),
            aspect_ratio: onto.aspect_ratio.unless_non_default(patch.aspect_ratio),
            align_items: onto.align_items.unless_non_default(patch.align_items),
            justify_items: onto.justify_items.unless_non_default(patch.justify_items),
            align_self: onto.align_self.unless_non_default(patch.align_self),
            justify_self: onto.justify_self.unless_non_default(patch.justify_self),
            align_content: onto.align_content.unless_non_default(patch.align_content),
            justify_content: onto.justify_content.unless_non_default(patch.justify_content),
            margin: onto.margin.unless_non_default(patch.margin),
            padding: onto.padding.unless_non_default(patch.padding),
            border: onto.border.unless_non_default(patch.border),
            flex_direction: onto.flex_direction.unless_non_default(patch.flex_direction),
            flex_wrap: onto.flex_wrap.unless_non_default(patch.flex_wrap),
            flex_grow: onto.flex_grow.unless_non_default(patch.flex_grow),
            flex_shrink: onto.flex_shrink.unless_non_default(patch.flex_shrink),
            flex_basis: onto.flex_basis.unless_non_default(patch.flex_basis),
            row_gap: onto.row_gap.unless_non_default(patch.row_gap),
            column_gap: onto.column_gap.unless_non_default(patch.column_gap),
            grid_auto_flow: onto.grid_auto_flow.unless_non_default(patch.grid_auto_flow),
            grid_template_rows: onto.grid_template_rows.clone().unless_non_default(patch.grid_template_rows.clone()),
            grid_template_columns: onto.grid_template_columns.clone().unless_non_default(patch.grid_template_columns.clone()),
            grid_auto_rows: onto.grid_auto_rows.clone().unless_non_default(patch.grid_auto_rows.clone()),
            grid_auto_columns: onto.grid_auto_columns.clone().unless_non_default(patch.grid_auto_columns.clone()),
            grid_row: onto.grid_row.unless_non_default(patch.grid_row),
            grid_column: onto.grid_column.unless_non_default(patch.grid_column),
        }
    }

    pub(crate) fn as_style(&self, css: &CSS) -> Style {
        self.0.iter().map(|each| css.style(each)).fold(Style::default(), |acc, patch| Self::overlay(&patch, &acc))
    }
}

pub(crate) fn recalculate(
    mut css: ResMut<CSS>,
    resize_reader: EventReader<WindowResized>,
    window: Query<&Window>,
) {
    // only run when window is resizing
    if resize_reader.is_empty() { return; }

    // FIXME -- found by experimentation, rem is not supported by Bevy
    css.rem = 19.5;

    let width = window.single().resolution.width();

    css.logo_height = match width {
        w if w >= 992.0 => Val::Px(40.),
        _ => Val::Px(28.)
    };

    css.header_height = match width {
        w if w >= 992.0 => Val::Px(72.),
        _ => Val::Px(60.)
    };

    css.header_message_font_size = match width {
        w if w >= 1200. => 1.3 * css.rem,
        w if w >= 768. => 1.28 * css.rem,
        w if w >= 550. => 0.875 * 1.28 * css.rem,
        // FIXME -- scale to 0 here to simulate "display: none" CSS
        _ => 0.
    };

    css.header_padding = match width {
        w if w >= 992. => UiRect::px(16., 16., 0., 0.),
        _ => UiRect::right(Val::Px(8.))
    };

    css.github_img_height = match width {
        w if w >= 768. => Val::Px(35.),
        _ => Val::Px(30.),
    }
}

pub(crate) fn a_hover(
    link_interaction: Query<&Interaction, (Changed<Interaction>, With<Link>)>,
    mut window: Query<&mut Window>,
) {
    let mut window = window.single_mut();

    // When buttons are right next to each other, it's possible to stop interacting with one at the same
    // time we start interacting with the next one. This can cause a race condition if these interactions are
    // handled out of order, e.g.
    //
    //   1) handle hovering over the new button first == change icon to Pointer
    //   2) handle hovering off of the old button second == change icon to Default
    //
    // So rather than looping over all interactions and handling each one, we just see if _anything_ is
    // being hovered over (or pressed), and if so, set the icon to Pointer.

    let interactions = link_interaction.iter().collect::<Vec<&Interaction>>();

    if interactions.iter().any(|interaction| matches!(interaction, Interaction::Hovered | Interaction::Pressed)) {
        window.cursor.icon = CursorIcon::Pointer
    } else if interactions.len() > 0 {
        window.cursor.icon = CursorIcon::Default
    }

    // TODO add URL box pinned to lower-left corner of browser window

}

pub(crate) fn main_menu_link_hover(
    link_interaction: Query<(&Interaction, &Children, &css::Classes), Changed<Interaction>>,
    mut query_text: Query<&mut Text>,
) {
    for (interaction, children, classes) in link_interaction.iter() {
        if !classes.0.contains(&css::Class::MainMenuLink) { continue; }

        info!("user interacted with a MainMenuLink OR window was just re-rendered due to resizing");

        let hovered_color = Srgba::hex("#b1d9ff").unwrap().into();
        let normal_color = Srgba::hex("#ececec").unwrap().into();

        match interaction {
            Interaction::Hovered | Interaction::Pressed => {
                for child in children {
                    if let Ok(mut text) = query_text.get_mut(*child) {
                        text.sections[0].style.color = hovered_color;
                    }
                }
            }
            Interaction::None => {
                for child in children {
                    if let Ok(mut text) = query_text.get_mut(*child) {
                        text.sections[0].style.color = normal_color;
                    }
                }
            }
        }
    }
}