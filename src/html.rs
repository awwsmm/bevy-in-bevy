use bevy::prelude::*;

use crate::{css, Link, when_debugging};

pub(crate) fn a(
    parent: &mut ChildBuilder,
    css: &css::CSS,
    classes: css::Classes,
    href: &str,
    children: impl FnOnce(&mut ChildBuilder),
) {
    let mut bundle = classes.0.iter().fold(ButtonBundle::default(), |acc, class| {
        css::overlay_buttons(&css.button_style(class), &acc)
    });

    parent.spawn((
        ButtonBundle {
            style: classes.as_style(css),
            ..bundle
        },
        Link { href: href.into() },
        classes
    )).with_children(children);
}

// used for: div, li, navbar, header, etc.
pub(crate) fn div(
    parent: &mut ChildBuilder,
    css: &css::CSS,
    classes: css::Classes,
    children: impl FnOnce(&mut ChildBuilder),
) {
    let bundle = classes.0.iter().fold(NodeBundle::default(), |acc, class| {
        css::overlay_nodes(&css.node_style(class), &acc)
    });

    parent.spawn((
        NodeBundle {
            style: classes.as_style(css),
            ..bundle
        },
        classes
    )).with_children(children);
}

pub(crate) fn img(
    parent: &mut ChildBuilder,
    css: &css::CSS,
    classes: css::Classes,
    image: Handle<Image>,
) {
    parent.spawn(
        // FIXME -- no alt text on images
        ImageBundle {
            style: Style {
                ..classes.as_style(css)
            },
            image: UiImage::new(image),
            ..default()
        }
    );
}

pub(crate) fn text(
    parent: &mut ChildBuilder,
    css: &css::CSS,
    classes: css::Classes,
    text: &str,
) {
    // let text_style = classes.0.iter().fold(TextStyle::default(), |acc, class| {
    //     css::overlay_text(&css.text_style(class), &acc)
    // });
    //
    // parent.spawn(
    //     TextBundle {
    //         style: Style {
    //             border: when_debugging(UiRect::all(Val::Px(1.))),
    //             ..classes.as_style(css)
    //         },
    //         text: Text::from_section(
    //             text,
    //             text_style
    //         ),
    //         ..default()
    //     }
    // );

    let mut bundle = classes.0.iter().fold(TextBundle::default(), |acc, class| {
        css::overlay_text(&css.text_style(class), &acc)
    });

    parent.spawn((
        TextBundle {
            style: classes.as_style(css),
            text: Text::from_section(
                text,
                bundle.text.sections[0].clone().style // FIXME clunky to get style out of TextBundle
            ),
            ..bundle
        },
        classes
    ));
}