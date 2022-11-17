//! This module defines our macros for creating asset collections.
//!
//! Thanks to Travis Veazey <https://github.com/Kromey> for the `asset_collection!()`,
//! `asset_collection_struct!()`, and `asset_collection_impl!()` macros.

/// This macro defines a DSL-like syntax for creating asset collections.
///
/// An asset collection is a struct containing Bevy [`Handle`]s to assets, and is meant to be
/// used as a Bevy resource within the ECS to provide access to those assets. This macro provides
/// a simple, declarative syntax for defining the asset collection.
///
/// The first argument is the name of the asset collection itself, which will be a `pub` struct.
/// A series of asset definitions, separated by commas, define the individual assets.
///
/// ```
/// asset_collection!(
///     AssetCollection,        // Name of the asset collection
///     Image(foo, "foo.png"),  // An Image asset named `foo`, which will load the file "foo.png"
///     Image(bar, "bar.png"),  // Image asset named `bar`, from "bar.png"
///     Atlas(baz, "tile.png", 32., 32., 10, 6) // TextureAtlas defining a grid of 32x32 pixel sprites, 10 columns by 6 rows
/// );
/// ```
///
/// Once the asset collection is defined, you must called its `init()` method to begin loading
/// the assets; a simple system to do this might look like:
/// ```
/// fn init_collection(
///     mut commands: Commands,
///     mut atlases: ResMut<Assets<TextureAtlas>>,
///     server: Res<AssetServer>,
/// ) {
///     let collection = AssetCollection::init(&server, &mut atlases);
///     commands.insert_resource(collection);
/// }
/// ```
///
/// After this you can proceed to use it immediately, however the assets won't actually appear
/// in your game until they finish loading. If you wish to check for their readiness, you can
/// use the `get_collection_load_state()` method; here's a simple system to do this:
/// ```
/// fn check_assets_ready(
///     server: Res<AssetServer>,
///     collection: Res<AssetCollection>,
///     atlases: Res<Assets<TextureAtlas>>,
///     mut app_state: ResMut<State<GameState>>,
/// ) {
///     if let LoadState::Loaded = collection.get_collection_load_state(&server, &atlases) {
///         app_state
///             .set(GameState::Loaded)
///             .expect("We don't run in this state so changing to it won't fail");
///     }
/// }
/// ```
/// This system changes the game's state to `GameState::Loaded` once all of the assets have
/// finished loading.
///
/// If any asset is in the `LoadState` of `Unloaded`, `Failed`, or `NotLoaded`, `get_collection_load_state()`
/// will return that state; otherwise, it will return `Loading` if any asset is still `Loading`,
/// or `Loaded` if all assets are `Loaded`.
///
/// Currently only `Image` and `TextureAtlas` assets can be defined.
macro_rules! asset_collection {
    ( $name:ident, $($assets:tt)* ) => {
        asset_collection_struct!($name { $($assets)*, } -> ());
        asset_collection_impl!($name self ctx server atlases { $($assets)*, } -> ()());
    };
}

/// Internal macro used to create the actual structure of the asset collection
macro_rules! asset_collection_struct {
    ( $name:ident {  } -> ( $($result:tt)*) ) => {
        #[derive(Default, Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $($result)*
        }
    };
    ($name:ident { Image($asset:ident, $path:expr), $($assets:tt)* } -> ($($result:tt)*) ) => {
        asset_collection_struct!($name { $($assets)* } -> (
            $($result)*
            pub $asset: Handle<Image>,
        ));
    };
    ($name:ident { Atlas($asset:ident, $path:expr, $width:expr, $height:expr, $columns:expr, $rows:expr), $($assets:tt)* } -> ($($result:tt)*) ) => {
        asset_collection_struct!($name { $($assets)* } -> (
            $($result)*
            pub $asset: Handle<TextureAtlas>,
        ));
    };
}

/// Internal macro used to implement the methods required by the asset collection
macro_rules! asset_collection_impl {
    ( $name:ident $self:ident $ctx:ident $server:ident $atlases:ident {  } -> ($($init:tt)*)($($status:tt)*) ) => {
        impl $name {
            fn init(
                $server: &AssetServer,
                $atlases: &mut Assets<TextureAtlas>,
            ) -> Self {
                let mut $ctx = Self::default();
                $($init)*


                $ctx
            }


            fn get_collection_load_state(
                &$self,
                $server: &AssetServer,
                $atlases: &Assets<TextureAtlas>,
            ) -> LoadState {
                let mut $ctx = LoadState::Loaded;
                $($status)*


                $ctx
            }
        }
    };
    ($name:ident $self:ident $ctx:ident $server:ident $atlases:ident { Image($asset:ident, $path:expr), $($assets:tt)* } -> ($($init:tt)*)($($status:tt)*) ) => {
        asset_collection_impl!($name $self $ctx $server $atlases { $($assets)* } -> (
            $($init)*
            $ctx.$asset = $server.load($path);
        )(
            $($status)*
            match $server.get_load_state($self.$asset.clone()) {
                LoadState::Loaded => {},
                LoadState::Loading => $ctx = LoadState::Loading,
                state => return state,
            }
        ));
    };
    ($name:ident $self:ident $ctx:ident $server:ident $atlases:ident { Atlas($asset:ident, $path:expr, $width:expr, $height:expr, $columns:expr, $rows:expr), $($assets:tt)* } -> ($($init:tt)*)($($status:tt)*) ) => {
        asset_collection_impl!($name $self $ctx $server $atlases { $($assets)* } -> (
            $($init)*
            let img = $server.load($path);
            $ctx.$asset = $atlases.add(TextureAtlas::from_grid(
                img,
                Vec2 { x: $width, y: $height },
                $columns,
                $rows
            ));
        )(
            $($status)*
            let img = &$atlases.get(&$self.$asset).unwrap().texture;
            match $server.get_load_state(img) {
                LoadState::Loaded => {},
                LoadState::Loading => $ctx = LoadState::Loading,
                state => return state,
            }
        ));
    };
}
