use bevy::prelude::*;
use bevy::text::{BreakLineOn, TextLayoutInfo};
use bevy::window::WindowResized;

use crate::{css, Link, when_debugging};
use crate::patch::Patch;

// cannot apply multiple components of the same type to a single entity, so group Vec<Class> into Classes
#[derive(Component)]
pub(crate) struct Classes(pub Vec<Class>);

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
            header_message_font_size: 1.3 * 19.5, // FIXME 1.3 * rem
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
    pub(crate) fn styles(&self, class: &Class) -> Styles {
        match class {
            Class::LayoutHeader => {
                // .layout__header {
                //     position: fixed;
                //     top: 0px;
                //     width: 100%;
                //     height: var(--header-height);
                //     background-color: #1e1e22;
                //     border-bottom: 2px solid #2c2c2d;
                //     z-index: 800;
                // }
                Styles {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        width: Val::Percent(100.),
                        height: self.header_height,
                        border: UiRect::bottom(Val::Px(2.0)),
                        padding: self.header_padding,
                        ..default()
                    },
                    node: NodeBundle {
                        background_color: Srgba::hex("#1e1e22").unwrap().into(),
                        border_color: Srgba::hex("#2c2c2d").unwrap().into(),
                        ..default()
                    },
                    ..default()
                }
            }
            Class::HeaderLogo => {
                // .header__logo {
                //     display: flex;
                //     align-items: center;
                // }
                Styles {
                    style: Style {
                        align_items: AlignItems::Center,
                        ..default()
                    },
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
                Styles {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::new(Val::Px(7.), Val::Px(7.), Val::Px(0.), Val::Px(0.)), // FIXME font is too wide, reduce padding to compensate
                        height: self.header_height,
                        border: when_debugging(UiRect::all(Val::Px(1.))),
                        ..default()
                    },
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
                Styles {
                    style: Style {
                        width: Val::Percent(100.),
                        max_width: Val::Px(1200.),
                        height: self.header_height,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        margin: UiRect::new(Val::Auto, Val::Auto, Val::ZERO, Val::ZERO),
                        ..default()
                    },
                    ..default()
                }
            }
            Class::HeaderLeftBlock => {
                // .header__left-block {
                //     display: flex;
                //     align-items: center;
                //     margin-right: auto;
                // }
                Styles {
                    style: Style {
                        align_items: AlignItems::Center,
                        margin: UiRect::right(Val::Auto),
                        // height: Val::Percent(100.),
                        // width: Val::Px(520.0), // required by child node
                        ..default()
                    },
                    ..default()
                }
            }
            Class::Logo => {
                // .logo {
                //     height: 28px;
                //     width: auto;
                // }
                Styles {
                    style: Style {
                        height: self.logo_height,
                        width: Val::Auto,
                        ..default()
                    },
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
                Styles {
                    style: Style {
                        margin: UiRect::left(Val::Px(12.)),
                        ..default()
                    },
                    text: TextBundle {
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
                    },
                    ..default()
                }
            }
            Class::MainMenuEntry => {
                Styles {
                    style: Style {
                        border: when_debugging(UiRect::all(Val::Px(1.))),
                        ..default()
                    },
                    node: NodeBundle {
                        border_color: when_debugging(Srgba::RED.into()),
                        // transform: Transform::from_scale(Vec3::new(0.97, 1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                }
            }
            Class::MainMenuLinkText => {
                Styles {
                    text: TextBundle {
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
                    },
                    ..default()
                }
            }
            Class::HeaderCtaContainer => {
                // .header__cta-container {
                //     display: flex;
                //     align-items: center;
                //     margin-left: 16px;
                // }
                Styles {
                    style: Style {
                        align_items: AlignItems::Center,
                        margin: UiRect::left(Val::Px(16.)),
                        ..default()
                    },
                    ..default()
                }
            }
            Class::ButtonPink => {
                // .header__cta:not(:last-child) {
                //     margin-right: 8px;
                // }
                // .button--pink {
                //     background-color: #9f517a;
                //     border-color: #ba789b;
                // }
                Styles {
                    style: Style {
                        margin: UiRect::right(Val::Px(8.)),
                        ..default()
                    },
                    button: ButtonBundle {
                        image: UiImage::default().with_color(Srgba::hex("#9f517a").unwrap().into()),
                        border_color: Srgba::hex("#ba789b").unwrap().into(),
                        ..default()
                    },
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
                //     border-radius: 10px; // FIXME Bevy doesn't yet support border-radius, see: https://github.com/bevyengine/bevy/pull/8973
                //     font-size: 1.2rem;
                //     font-weight: 500;
                //     transition: transform 100ms;
                // }
                Styles {
                    style: Style {
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(3.)),
                        padding: UiRect::px(8., 8., 6., 6.),
                        ..default()
                    },
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
                Styles {
                    style: Style {
                        height: Val::Px(1.1 * 17.), // TODO define em
                        margin: UiRect::left(Val::Px(0.2 * self.rem)),
                        ..default()
                    },
                    text: TextBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font_size: 0.9 * self.rem,
                                ..default()
                            }
                        ),
                        ..default()
                    },
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
                Styles {
                    text: TextBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: self.fira_sans.clone(),
                                font_size: 1.2 * self.rem,
                                ..default()
                            }
                        ),
                        ..default()
                    },
                    ..default()
                }
            }
            Class::HeaderCtaGitHub => {
                // .header__cta--github {
                //     display: flex;
                //     align-items: center;
                //     justify-content: center;
                // }
                Styles {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                }
            }
            Class::HeaderCtaGitHubImg => {
                // .header__cta--github img {
                //     height: 35px;
                // }
                Styles {
                    style: Style {
                        height: self.github_img_height,
                        width: Val::Auto,
                        ..default()
                    },
                    ..default()
                }
            }
        }
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

// ---

pub(crate) struct Styles {
    pub(crate) style: Style,
    pub(crate) node: NodeBundle,
    pub(crate) text: TextBundle,
    pub(crate) button: ButtonBundle
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            style: Default::default(),
            node: Default::default(),
            text: Default::default(),
            button: ButtonBundle {
                // default button background color is white -- it should be clear
                image: UiImage::default().with_color(Color::NONE).into(),
                ..default()
            }
        }
    }
}

impl From<Styles> for Style {
    fn from(value: Styles) -> Self {
        value.style
    }
}

impl From<Styles> for NodeBundle {
    fn from(value: Styles) -> Self {
        value.node
    }
}

impl From<Styles> for TextBundle {
    fn from(value: Styles) -> Self {
        value.text
    }
}

impl From<Styles> for ButtonBundle {
    fn from(value: Styles) -> Self {
        value.button
    }
}

pub(crate) trait Cascading {
    fn cascade(css: &CSS, classes: &Classes) -> Self;
}

impl<T: From<Styles> + Patch + Default> Cascading for T {
    fn cascade(css: &CSS, classes: &Classes) -> Self {
        classes.0.iter().map(|each| css.styles(each).into()).fold(Self::default(), |acc, patch| acc.apply(&patch))
    }
}