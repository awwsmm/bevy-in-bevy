use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResized;

mod css;
mod html;

// non-exhaustive list of missing features
// - ImageBundle: no alt text on images
// - ImageBundle: no native SVG support in Bevy
// - TextStyle: font_size cannot be specified in rem, only px
// - TextStyle: has no font_weight property, no letter_spacing property
// - NodeBundle: has no block or none options for display (https://www.w3schools.com/css/css_display_visibility.asp)

#[derive(Component)]
// <a href="...">
struct Link {
    href: String
}

#[derive(Component)]
// components which should be despawned before being redrawn
struct Despawn;

#[derive(Resource, Default)]
struct Images {
    bevy_logo_dark: Handle<Image>,
    heart: Handle<Image>,
    github_mark_white: Handle<Image>,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // https://github.com/bevyengine/bevy/issues/10157#issuecomment-1849092112
        .add_plugins(DefaultPlugins
             .set(WindowPlugin {
                 primary_window: Some(Window {
                     canvas: Some("#html-canvas-id".into()),
                     fit_canvas_to_parent: true,
                     ..default()
                 }),
                 ..default()
             })
        )
        .insert_resource(bevy::winit::WinitSettings::desktop_app())
        .insert_resource(Images::default())
        .insert_resource(ClearColor(Srgba::hex("#232326").unwrap().into()))
        .insert_resource(css::CSS::default())
        .add_systems(Startup, startup)
        .add_systems(Update, (despawn, css::recalculate, render).chain())
        .add_systems(Update, (css::a_hover, css::main_menu_link_hover))
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Images>,
    mut css: ResMut<css::CSS>,
) {

    commands.spawn(Camera2dBundle::default());

    // FIXME -- Bevy does not currently support SVG images natively
    // This crate adds SVG support but hasn't been updated since Bevy v0.12.0: https://github.com/Weasy666/bevy_svg
    // So instead, I converted all SVGs to PNGs
    // load() them once in setup() so we don't need to load() every Update schedule
    images.bevy_logo_dark = asset_server.load("bevy_logo_dark.png");
    images.heart = asset_server.load("heart.png");
    images.github_mark_white = asset_server.load("github-mark-white.png");

    // 500 font weight == Medium, according to https://fonts.google.com/specimen/Fira+Sans
    css.fira_sans = asset_server.load("fonts/FiraSans-Medium.ttf");
}

fn render(
    mut commands: Commands,
    resize_reader: EventReader<WindowResized>,
    css: Res<css::CSS>,
    images: Res<Images>,
) {
    // only run when window is resizing
    if resize_reader.is_empty() { return; }

    commands.spawn((
        NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
        Despawn
    )).with_children(|parent| {

        // layout__header
        html::div(parent, &css, css::Classes(vec![css::Class::LayoutHeader]), |parent| {

            // header__content
            html::div(parent, &css, css::Classes(vec![css::Class::HeaderContent]), |parent| {

                // <div> header__left-block
                html::div(parent, &css, css::Classes(vec![css::Class::HeaderLeftBlock]), |parent| {

                    // a.header__logo
                    html::a(parent, &css, css::Classes(vec![css::Class::HeaderLogo]), "https://bevyengine.org/", |parent| {
                        // <img> .logo
                        html::img(parent, &css, css::Classes(vec![css::Class::Logo]), images.bevy_logo_dark.clone())
                    });

                    // header__message
                    // FIXME -- wonkiness here relative to bevyengine.org due to lack of letter-spacing CSS property
                    html::text(parent, &css, css::Classes(vec![css::Class::HeaderMessage]), "Features");
                });

                // <nav> header__menu main-menu
                html::div(parent, &css, css::Classes(vec![]), |parent| {

                    // <div> main-menu__content
                    html::div(parent, &css, css::Classes(vec![]), |parent| {

                        // <ul> main-menu__menu
                        html::div(parent, &css, css::Classes(vec![]), |parent| {

                            fn li(parent: &mut ChildBuilder, css: &css::CSS, text: &str, href: &str) {
                                // <li> main-menu__entry
                                html::div(parent, &css, css::Classes(vec![css::Class::MainMenuEntry]), |parent| {
                                    // <a> main-menu__link
                                    html::a(parent, css, css::Classes(vec![css::Class::MainMenuLink]), href, |parent| {
                                        html::text(parent, &css, css::Classes(vec![css::Class::MainMenuLinkText]), text);
                                    });
                                });
                            }

                            li(parent, &css, "Learn", "/learn");
                            li(parent, &css, "News", "/news");
                            li(parent, &css, "Community", "/community");
                            li(parent, &css, "Foundation", "/foundation");
                            li(parent, &css, "Assets", "/assets");
                            li(parent, &css, "Examples", "/examples");
                        })
                    })
                });

                // <div> header__cta-container
                html::div(parent, &css, css::Classes(vec![css::Class::HeaderCtaContainer]), |parent| {

                    // <a> button--pink
                    html::a(parent, &css, css::Classes(vec![css::Class::Button, css::Class::ButtonPink]), "/donate", |parent| {
                        html::text(parent, &css, css::Classes(vec![css::Class::ButtonText]), "Donate");
                        html::img(parent, &css, css::Classes(vec![css::Class::ButtonIcon]), images.heart.clone());
                    });

                    // <a> header__cta--github
                    html::a(parent, &css, css::Classes(vec![css::Class::HeaderCtaGitHub]), "/donate", |parent| {
                        html::img(parent, &css, css::Classes(vec![css::Class::HeaderCtaGitHubImg]), images.github_mark_white.clone());
                    })

                })
            })
        })
    });
}

// only run when window is resizing
// despawn any entities with a Despawn component before spawning them again
fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Despawn>>,
    mut resize_reader: EventReader<WindowResized>
) {
    for _ in resize_reader.read() {
        for e in &query {
            commands.entity(e).despawn_recursive();
        }
    }
}

const DEBUG: bool = false;

fn when_debugging<T: Default>(t: T) -> T {
    if DEBUG { t } else { T::default() }
}