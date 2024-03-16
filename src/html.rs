use bevy::prelude::*;

use crate::{css, Link};
use crate::css::Cascading;

pub(crate) fn a(
    parent: &mut ChildBuilder,
    css: &css::CSS,
    classes: css::Classes,
    href: &str,
    children: impl FnOnce(&mut ChildBuilder),
) {
    parent.spawn((
        ButtonBundle {
            style: Style::cascade(css, &classes),
            ..ButtonBundle::cascade(css, &classes)
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
    parent.spawn((
        NodeBundle {
            style: Style::cascade(css, &classes),
            ..NodeBundle::cascade(css, &classes)
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
            style: Style::cascade(css, &classes),
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
    let mut bundle = TextBundle::cascade(css, &classes);
    parent.spawn((
        TextBundle {
            style: Style::cascade(css, &classes),
            text: Text::from_section(
                text,
                bundle.text.sections[0].clone().style // FIXME clunky to get style out of TextBundle
            ),
            ..bundle
        },
        classes
    ));
}