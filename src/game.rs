use bevy::{prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};
use std::{io::Result, time::Duration};

use crate::{
    loading::{BackgroundAssets, PowerAssets, UiAssets},
    save::{format_load, format_save, Save, Saveable},
    settings::Settings,
    ui::*,
    AppState, Cost, CurrentOwned, MaxOwned, PauseState, ProdAmount, ProdRate, ProdTimer, Title,
    UnlockBound, ID,
};

pub struct GameLoopPlugin;
impl Plugin for GameLoopPlugin {
    fn name(&self) -> &str {
        "Game Plugin"
    }

    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPowerButton>()
            .add_systems(OnEnter(AppState::Playing), startup)
            .add_systems(OnExit(AppState::Playing), cleanup)
            .add_systems(OnEnter(PauseState::Paused), pause_startup)
            .add_systems(OnExit(PauseState::Paused), pause_cleanup)
            .add_systems(Update, pause_click.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                (
                    evr_spawn_power_button,
                    screen_click,
                    update_power_text,
                    power_click,
                    tick_power_timers,
                    add_to_total_power,
                    check_power_unlock_flags,
                    auto_click,
                )
                    .run_if(in_state(PauseState::Unpaused)),
            )
            .add_systems(Update, save_button.run_if(in_state(PauseState::Paused)))
            .insert_resource(PowerUnlockFlags::load("power_unlocks.ron").unwrap_or_default())
            .insert_resource(Powers::load("powers.ron").unwrap_or_default())
            .insert_resource(TotalPower::load("total_power.ron").unwrap_or_default());
    }
}

#[derive(Component)]
struct CleanupGame;

#[derive(Component)]
struct CleanupPause;

#[derive(Default, Deref, DerefMut, Deserialize, Resource, Serialize)]
pub struct TotalPower(i64);
impl TotalPower {
    fn add_power(&mut self, i: i64) {
        self.0 += i;
    }

    fn power(&self) -> i64 {
        self.0
    }
}
impl Saveable for TotalPower {
    fn save(&self, filename: &str) -> Result<()> {
        format_save(self, filename)
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        format_load(filename)
    }
}

#[derive(Component, Clone)]
struct AutoClick(Timer);

#[derive(Component, Clone, Deserialize, Serialize)]
struct Power;

#[derive(Component, Deserialize, Serialize)]
struct PowerText;

#[derive(Bundle, Clone, Deserialize, Serialize)]
pub struct PowerBundle {
    power: Power,
    title: Title,
    id: ID,
    cost: Cost,
    production_amount: ProdAmount,
    production_rate: ProdRate,
    max_owned: MaxOwned,
    current_owned: CurrentOwned,
    unlock_bound: UnlockBound,
}

#[derive(Deref, DerefMut, Deserialize, Resource, Serialize)]
pub struct Powers(Vec<PowerBundle>);
impl Saveable for Powers {
    fn save(&self, filename: &str) -> std::io::Result<()> {
        format_save(self, filename)
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        format_load(filename)
    }
}
impl Default for Powers {
    fn default() -> Self {
        Self(vec![
            PowerBundle {
                power: Power,
                title: Title("Default Power".into()),
                id: ID(0),
                cost: Cost(0),
                production_amount: ProdAmount(1),
                production_rate: ProdRate(1000000.0),
                max_owned: MaxOwned(1),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(9_223_372_036_854_775_807),
            },
            PowerBundle {
                power: Power,
                title: Title("Middle School Science Project".into()),
                id: ID(1),
                cost: Cost(50),
                production_amount: ProdAmount(5),
                production_rate: ProdRate(5.0),
                max_owned: MaxOwned(9_223_372_036_854_775_807),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(50),
            },
            PowerBundle {
                power: Power,
                title: Title("Hamster on a Wheel".into()),
                id: ID(2),
                cost: Cost(1000),
                production_amount: ProdAmount(25),
                production_rate: ProdRate(1.0),
                max_owned: MaxOwned(30_000_000),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(1000),
            },
            PowerBundle {
                power: Power,
                title: Title("\'Gas\' Engine".into()),
                id: ID(3),
                cost: Cost(33_333),
                production_amount: ProdAmount(100_000),
                production_rate: ProdRate(240.0),
                max_owned: MaxOwned(1),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(10_000),
            },
            PowerBundle {
                power: Power,
                title: Title("Portable Generator".into()),
                id: ID(4),
                cost: Cost(242_424),
                production_amount: ProdAmount(800),
                production_rate: ProdRate(8.0),
                max_owned: MaxOwned(9_223_372_036_854_775_807),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(100_000),
            },
            PowerBundle {
                power: Power,
                title: Title("Hotwire the Neighbors".into()),
                id: ID(5),
                cost: Cost(999_999),
                production_amount: ProdAmount(222),
                production_rate: ProdRate(0.5),
                max_owned: MaxOwned(128_000_000),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(1_000_000),
            },
            PowerBundle {
                power: Power,
                title: Title("Electric Eel Farm".into()),
                id: ID(6),
                cost: Cost(2_500_000),
                production_amount: ProdAmount(45_000),
                production_rate: ProdRate(30.0),
                max_owned: MaxOwned(10_000_000),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(10_000_000),
            },
            PowerBundle {
                power: Power,
                title: Title("Miniscule Hadron Collider".into()),
                id: ID(7),
                cost: Cost(111_111_111),
                production_amount: ProdAmount(123_456),
                production_rate: ProdRate(33.0),
                max_owned: MaxOwned(123_456_789),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(100_000_000),
            },
            PowerBundle {
                power: Power,
                title: Title("Luke-warm Fusion Reactor".into()),
                id: ID(8),
                cost: Cost(987_654_321),
                production_amount: ProdAmount(9_999),
                production_rate: ProdRate(0.1),
                max_owned: MaxOwned(1),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(1_000_000_000),
            },
            PowerBundle {
                power: Power,
                title: Title("Buttered Cat Paradox".into()),
                id: ID(9),
                cost: Cost(1),
                production_amount: ProdAmount(1),
                production_rate: ProdRate(0.00001),
                max_owned: MaxOwned(999),
                current_owned: CurrentOwned(0),
                unlock_bound: UnlockBound(10_000_000_000),
            },
        ])
    }
}

#[derive(Deserialize, Resource, Serialize)]
pub struct PowerUnlockFlags(HashMap<usize, bool>);
impl Saveable for PowerUnlockFlags {
    fn save(&self, filename: &str) -> std::io::Result<()> {
        format_save(self, filename)
    }

    fn load(filename: &str) -> Result<Self>
    where
        Self: Sized,
    {
        format_load(filename)
    }
}
impl Default for PowerUnlockFlags {
    fn default() -> Self {
        PowerUnlockFlags(
            (0..10) // CREATE RANGE
                .map(|i| (i, false)) // MAP
                .collect(), // COLLECT
        )
    }
}

#[derive(Event)]
struct SpawnPowerButton(usize);

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
    power_flags: Res<PowerUnlockFlags>,
    background_assets: Res<BackgroundAssets>,
    ui_assets: Res<UiAssets>,
    mut evr_spawn_power_button: EventWriter<SpawnPowerButton>,
) {
    // SPAWN BACKGROUND SPRITE
    commands.spawn((
        Sprite::from_image(background_assets.room_background.clone()),
        Transform::from_scale(Vec3::splat(settings.sprite_scale())),
        CleanupGame,
    ));

    // SPAWN SCREEN CLICK NODE
    let screen_button_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::NONE),
    );

    commands.spawn((
        ScreenButtonNode::default(),
        ScreenButtonNode::marker(),
        Button,
        ScreenButton,
        screen_button_style,
        CleanupGame,
    ));

    // SPAWN POWER BUTTON PARENT NODE
    commands.spawn((
        UIButtonParentNode::node(),
        UIButtonParentNode::marker(),
        CleanupGame,
    ));

    // SPAWN TOTAL POWER TEXT
    let font: Handle<Font> = asset_server.load("fonts/PublicPixel.ttf");
    let text_color = Pallette::White.srgb();
    let font_size = 60.0;
    commands
        .spawn((
            PowerTextNode::default(),
            PowerTextNode::marker(),
            Text::new("POWER: "),
            TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            TextColor(text_color),
            BackgroundColor(Color::NONE),
            CleanupGame,
        ))
        .with_child((
            TextSpan::default(),
            (
                TextFont {
                    font: font.clone(),
                    font_size,
                    ..default()
                },
                TextColor(text_color),
            ),
            PowerText,
        ));

    // SPAWN PAUSE BUTTON
    let entity = commands
        .spawn((
            PauseButtonParentNode::default(),
            PauseButtonParentNode::marker(),
            CleanupGame,
        ))
        .id();

    let children_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::NONE),
        ImageNode::from_atlas_image(
            ui_assets.pause_atlas.clone(),
            TextureAtlas {
                layout: ui_assets.pause_layout.clone(),
                index: 0,
            },
        ),
    );

    let grandchildren_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::NONE),
        ImageNode::from_atlas_image(
            ui_assets.pause_border_atlas.clone(),
            TextureAtlas {
                layout: ui_assets.pause_border_layout.clone(),
                index: 0,
            },
        ),
    );
    let children = commands
        .spawn((PauseButtonChildNode::node(), children_style))
        .id();
    let grandchildren = commands
        .spawn((
            PauseButtonChildNode::node(),
            PauseButtonChildNode::marker(),
            Button,
            UIButton,
            PauseButton,
            grandchildren_style,
        ))
        .id();
    commands.entity(children).add_children(&[grandchildren]);
    commands.entity(entity).add_children(&[children]);

    info!("[SPAWNED] Game Nodes");

    // SPAWN POWER BUTTONS ALREADY UNLOCKED
    for i in 0..power_flags.0.len() {
        if let Some((_k, v)) = power_flags.0.get_key_value(&i) {
            if *v {
                evr_spawn_power_button.send(SpawnPowerButton(i));
            }
        }
    }

    // SPAWN AUTO-CLICK TIMER
    commands.spawn(AutoClick(Timer::new(
        Duration::from_secs_f64(0.125),
        TimerMode::Repeating,
    )));
}

fn cleanup(mut commands: Commands, query_entity: Query<Entity, With<CleanupGame>>) {
    for entity in query_entity.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Game Entities");
    }
}

fn evr_spawn_power_button(
    mut evr_spawn_power_button: EventReader<SpawnPowerButton>,
    mut query_parent_node: Query<Entity, With<UIButtonParentNode>>,
    powers: Res<Powers>,
    power_assets: Res<PowerAssets>,
    query_power: Query<&ID, With<Power>>,
    mut commands: Commands,
    power_flags: Res<PowerUnlockFlags>,
    asset_server: Res<AssetServer>,
) {
    for ev in evr_spawn_power_button.read() {
        // CHECK IF KEY/VALUE PAIR EXISTS
        if let Some((_k, v)) = power_flags.0.get_key_value(&ev.0) {
            // CHECK IF POWER IS UNLOCKED
            if *v {
                // CHECK IF POWER IS ALREADY SPAWNED
                if !query_power.iter().any(|id| id.0 == ev.0) {
                    // SAFELY GET PARENT NODE ENTITY
                    if let Ok(parent_entity) = query_parent_node.get_single_mut() {
                        if let Some(power) = powers.0.iter().find(|power| power.id.0 == ev.0) {
                            // SPAWN AND INSERT POWER BUTTON
                            let zero_style = (
                                BorderColor(Color::NONE),
                                BorderRadius::ZERO,
                                BackgroundColor(Color::NONE),
                            );

                            let children = (
                                zero_style,
                                ImageNode::from_atlas_image(
                                    power_assets.power_atlas.clone(),
                                    TextureAtlas {
                                        layout: power_assets.power_layout.clone(),
                                        index: ev.0,
                                    },
                                ),
                            );

                            let grandchildren = (
                                zero_style,
                                ImageNode::from_atlas_image(
                                    power_assets.border_atlas.clone(),
                                    TextureAtlas {
                                        layout: power_assets.border_layout.clone(),
                                        index: 0,
                                    },
                                ),
                            );

                            let child_entity =
                                commands.spawn((UIButtonPowerNode::node(), children)).id();
                            let grandchild_entity = commands
                                .spawn((
                                    UIButtonPowerNode::node(),
                                    UIButtonPowerNode::marker(),
                                    Button,
                                    UIButton,
                                    PowerButton,
                                    ID(ev.0),
                                    grandchildren,
                                ))
                                .id();

                            let font = asset_server.load("fonts/PublicPixel.ttf");

                            let info_text_entity = commands
                                .spawn((UiButtonInfoNode::node(), zero_style))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new(power.title.0.clone()),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(Pallette::White.srgb()),
                                    ));

                                    parent.spawn((
                                        Text::new(format!(
                                            "COST: {}\nPROD: {}pwr/{}s",
                                            power.cost.0,
                                            power.production_amount.0,
                                            power.production_rate.0
                                        )),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(Pallette::Light.srgb()),
                                    ));

                                    parent.spawn((
                                        Text::new(format!("MAX: {}", power.max_owned.0)),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 10.0,
                                            ..default()
                                        },
                                        TextColor(Pallette::White.srgb()),
                                    ));
                                })
                                .id();

                            commands
                                .entity(grandchild_entity)
                                .add_children(&[info_text_entity]);
                            commands
                                .entity(child_entity)
                                .add_children(&[grandchild_entity]);
                            commands.entity(parent_entity).add_children(&[child_entity]);

                            // SPAWN ACTUAL POWER BUNDLE + INSERT TIMER
                            commands.spawn(power.clone()).insert(ProdTimer::new(
                                power.production_rate.0,
                                TimerMode::Repeating,
                            ));
                        }

                        info!("[SPAWNED] Power + Button: {}", ev.0);
                    } else {
                        info!("[ERROR] Button Parent Node Not Spawned.");
                    }
                } else {
                    info!("[ERROR] Power: {} Already Spawned", ev.0);
                }
            } else {
                info!("[ERROR] Power: {} Not Unlocked", ev.0);
            }
        }
    }
}

fn screen_click(
    mut total_power: ResMut<TotalPower>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ScreenButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            total_power.add_power(1);
            info!("[EVENT] Click");
            info!("[MODIFIED] Total Power: {}", total_power.0);
        }
    }
}

fn pause_click(
    pause_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match pause_state.get() {
                PauseState::Paused => {
                    next_state.set(PauseState::Unpaused);
                    info!("[MODIFIED] PauseState >> Unpaused");
                }
                PauseState::Unpaused => {
                    next_state.set(PauseState::Paused);
                    info!("[MODIFIED] PauseState >> Paused");
                }
            }
        }
    }
}

fn update_power_text(
    total_power: Res<TotalPower>,
    mut query_power_text: Query<&mut TextSpan, With<PowerText>>,
) {
    if total_power.is_changed() {
        for mut span in &mut query_power_text {
            **span = format!("{}", total_power.power());
        }
    }
}

fn pause_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/PublicPixel.ttf");
    let parent_style = (
        BorderColor(Color::NONE),
        BorderRadius::ZERO,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    );

    let children_style = (
        (BorderColor(Pallette::Black.srgb())),
        BorderRadius::all(Val::Percent(10.0)),
        BackgroundColor(Pallette::Lighter.srgb()),
    );

    commands
        .spawn((
            PauseParentNode::default(),
            PauseParentNode::marker(),
            parent_style,
            CleanupPause,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::from("PAUSED"),
                TextFont {
                    font: font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Pallette::Lighter.srgb()),
            ));
            parent
                .spawn((
                    UIButtonChildNode::node(),
                    UIButtonChildNode::marker(),
                    Button,
                    UIButton,
                    SaveExitButton,
                    children_style,
                ))
                .with_child((
                    Text::from("SAVE"),
                    TextFont {
                        font,
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Pallette::Darker.srgb()),
                ));
        });
    info!("[SPAWNED] Pause Entities");
}

fn pause_cleanup(mut commands: Commands, query_entity: Query<Entity, With<CleanupPause>>) {
    for entity in query_entity.iter() {
        commands.entity(entity).despawn_recursive();
        info!("[DESPAWNED] Pause Entities");
    }
}

fn power_click(
    query_interaction: Query<(&Interaction, &ID), (Changed<Interaction>, With<PowerButton>)>,
    mut query_power: Query<(&ID, &Cost, &MaxOwned, &mut CurrentOwned), With<Power>>,
    mut total_power: ResMut<TotalPower>,
) {
    for (interaction, interaction_id) in &query_interaction {
        // ONLY RUN IF A POWER BUTTON IS PRESSED
        if *interaction == Interaction::Pressed {
            for (power_id, cost, max_owned, mut current_owned) in query_power.iter_mut() {
                // GRAB CORRESPONDING ENTITY + COMPONENTS
                if power_id.0 == interaction_id.0 {
                    // MAKE SURE YOU HAVE ENOUGH POWER
                    if total_power.0 - cost.0 >= 0 {
                        // MAKE SURE IT WOULD NOT PUT YOU OVER LIMIT
                        if current_owned.0 < max_owned.0 {
                            // DO THE THING
                            total_power.0 -= cost.0;
                            current_owned.0 += 1;
                            info!(
                                "[MODIFIED] Current Owned -- ID: {} >> Amt: {}",
                                power_id.0, current_owned.0
                            );
                        } else {
                            info!("[INVALID] Maximum Already Owned");
                        }
                    } else {
                        info!("[INVALID] Insufficient Power");
                    }
                }
            }
        }
    }
}

fn tick_power_timers(mut query_timer: Query<&mut ProdTimer>, time: Res<Time>) {
    for mut timer in query_timer.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn add_to_total_power(
    mut query_timer: Query<(&mut ProdTimer, &ProdAmount, &CurrentOwned, &ID)>,
    mut total_power: ResMut<TotalPower>,
) {
    for (mut timer, prod_amount, current_owned, id) in query_timer.iter_mut() {
        if timer.0.finished() {
            let total_amount = prod_amount.0 * current_owned.0;
            total_power.0 += total_amount;
            timer.0.reset();
            if total_amount > 0 {
                info!(
                    "[MODIFIED] Total Power +{} From Power: {}",
                    total_amount, id.0
                );
            }
        }
    }
}

fn check_power_unlock_flags(
    mut power_flags: ResMut<PowerUnlockFlags>,
    mut evw_spawn_power_button: EventWriter<SpawnPowerButton>,
    powers: Res<Powers>,
    total_power: Res<TotalPower>,
) {
    for power in powers.0.iter() {
        if let Some(flag) = power_flags.0.get_mut(&power.id.0) {
            if total_power.0 >= power.unlock_bound.0 {
                if !flag.to_owned() {
                    *flag = true;
                    evw_spawn_power_button.send(SpawnPowerButton(power.id.0));
                    info!("[UNLOCKED] Power ID: {}", power.id.0);
                }
            }
        }
    }
}

fn save_button(
    mut query_interaction: Query<&Interaction, (Changed<Interaction>, With<SaveExitButton>)>,
    mut evw_save: EventWriter<Save>,
    mut powers: ResMut<Powers>,
    query_powers: Query<(
        &Power,
        &Title,
        &ID,
        &Cost,
        &ProdAmount,
        &ProdRate,
        &MaxOwned,
        &CurrentOwned,
        &UnlockBound,
    )>,
) {
    for interaction in &mut query_interaction {
        if *interaction == Interaction::Pressed {
            for (
                power,
                title,
                id,
                cost,
                production_amount,
                production_rate,
                max_owned,
                current_owned,
                unlock_bound,
            ) in query_powers.iter()
            {
                for power_bundle in powers.0.iter_mut() {
                    if power_bundle.id.0 == id.0 {
                        power_bundle.power = power.to_owned();
                        power_bundle.title.0 = title.0.to_owned();
                        power_bundle.id.0 = id.0.to_owned();
                        power_bundle.cost.0 = cost.0.to_owned();
                        power_bundle.production_amount.0 = production_amount.0.to_owned();
                        power_bundle.production_rate.0 = production_rate.0.to_owned();
                        power_bundle.max_owned.0 = max_owned.0.to_owned();
                        power_bundle.current_owned.0 = current_owned.0.to_owned();
                        power_bundle.unlock_bound.0 = unlock_bound.0.to_owned();
                    }

                    info!("[MODIFIED] Power: {} -- Syncing", power_bundle.id.0);
                }
            }
            evw_save.send(Save);
        }
    }
}

fn auto_click(
    time: Res<Time>,
    settings: Res<Settings>,
    mut total_power: ResMut<TotalPower>,
    mut query_auto_click: Query<&mut AutoClick>,
) {
    if settings.auto_click {
        if let Ok(mut auto_click) = query_auto_click.get_single_mut() {
            auto_click.0.tick(time.delta());
            if auto_click.0.finished() {
                total_power.0 += 1;
                auto_click.0.reset();
                info!("[EVENT] Auto-Click");
            }
        }
    }
}
