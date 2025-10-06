use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub struct UIPlugin;

#[derive(Component)]
pub struct UICamera;

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq)]
pub struct TotalCoin{
    pub amount: u32,
}

#[derive(Component)]
pub struct CoinDisplay;

impl Plugin for UIPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource( TotalCoin {amount:0,})
            .add_systems(Startup, loadUI)
            .add_systems(Update, updateUI);
    }
}

pub fn loadUI(
    mut commands: Commands, 
){
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // draw after player camera
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(1),
        UICamera,
     ));
    commands.spawn((
        Node{
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            margin: UiRect::all(Val::Percent(0.)),
            padding: UiRect::all(Val::Percent(0.)),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Percent(2.),
            ..default()
        },  
    ))
    .with_children(|parent|{
        parent.spawn((
                Node {
                    width: Val::Percent(5.),
                    ..Default::default()
                },
            
            Text::new("Coins: "), 
            RenderLayers::layer(1),
        ));
        parent.spawn((
                Node {
                    width: Val::Percent(10.),
                    ..Default::default()
                },
            
            Text::new("coins"), 
            RenderLayers::layer(1),
            CoinDisplay,
        ));
        
    });
    
}

pub fn updateUI(
    coinCount: Res<TotalCoin>,
    mut query_coin: Query<&mut Text, With<CoinDisplay>>,
){
  // let mut coins = Text::new(coinCount.amount.to_string());
    //let mut text = query_coin.single().unwrap();
   
    for mut text in query_coin.iter_mut(){
        text.0 = coinCount.amount.to_string();
    }
}
