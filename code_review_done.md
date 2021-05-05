Code Review Done (thanks to https://github.com/Kromey)
======================================================

https://github.com/jonmsawyer/chui/blob/main/src/modules/player.rs#L39-L53
ditch these. replace with a single (required) `name` field. your current setup
is very ethnocentric and therefore limiting/alienating to individuals from
other cultures. this is growing more and more widely to be considered bad
practice as a result

you can look into it if you like, but the short version is that there are no
universal rules for names. in some cultures, the family name comes first; in
others, there's only a given name, while still others have 4 names considered
equally important and thus can't be represented with just "first" and "last"

    Good point. Player name is now just one field and is optional.


-------------------------------------------------------------------------------

minor nitpick, but i'll mention it anyway:
https://github.com/jonmsawyer/chui/blob/main/src/modules/player.rs#L109-L110
generally you want to match the order in your struct definition. obviously
this works just fine, and now that i'm mentioning it this might be a personal 
preference of my own. anyway, just bringing it up for you to consider and do
as you will

    Fixed.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/player.rs#L118
i suggest `let mut player = ...`
reason is that the string you're building isn't their name, but a broader
description of the player (color they're playing, name, and rating)
also the note above about using format!() to build the whole thing instead of
building a piece and then `+=`ing it on applies in this method too

the description (the /// comment) should likewise be updated, since you're not
really displaying a name per se

    Refactored and fixed. Format of name is also changed and tests have been
    updated.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/player.rs#L133-L169
give these tests more descriptive names, e.g. `full_name_is_computed_2` could
be `player_name_no_rating`. you should know what failed from the output of
`cargo test`, rather than having to cross-reference with the testing code to
figure out what component failed

    Done and done :)

-------------------------------------------------------------------------------

oh forgot to mention (and this is relevant to all other files i've looked at
thus far):
https://github.com/jonmsawyer/chui/blob/main/src/modules/color.rs#L1-L3
get rid of these "File:" and "Module:" lines. either you're looking at this in
the file, in which case you know what file (and, by extension, what module)
you're looking at, or you're looking at this in rustdoc documentation, in
which case you know what module (and, by extension, what file) you're looking
at

    Done. Fair points.

-------------------------------------------------------------------------------

the one case where this might be useful in certain circumstances is in mod.rs;
for this reason i've stopped using mod.rs files, and instead putting a
module-named file adjacent to the folder containing its submodules (e.g.
instead of src/modules/mod.rs, i'd have src/modules.rs, with submodules of
course living in src/modules/*)

    I've refactored mod.rs to their {module names}.rs. I didn't realize one
    could do this. Thanks for the tip.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/chess_move.rs#L119-L120
this is an anti-pattern you've got going on here. the loop is pointless, and
instead you should just have two consecutive checks on part_one[0] and
part_one[1]

https://github.com/jonmsawyer/chui/blob/main/src/modules/chess_move.rs#L153-L154
ditto

    As stated in the TODO, I will be implementing a real parser. This was just
    test code. The methods parse_square_to_sqaure_move() and
    parse_piece_capture_move() have been removed.

-------------------------------------------------------------------------------

more broadly in that same file:
i'd ditch MoveState entirely, and also the `Invalid` variant of MoveType, as
invalid moves can be more ergonomically and more idiomatically represented by
returning a Result from the relevant functions.
(also, your comments on the move_state and move_type fields of Move are
backwards)

    Done.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/engine.rs#L126-L134
scratch `display_` from these functions, because they are not displaying
anything -- they're returning a String
i'd also turn `display_board_colors_by_index` into `board_colors` and return a
Vec or [_; 64] or, if you really want to stick to the double-array board (I
recommend you don't), [[_; 8]; 8]

    Fair point. Renamed to `headers_for_white()` and `headers_for_black()`.

    `display_board_colors_by_index()` is just a test function. When I start
    displaying the GUI or displaying a colored board on the terminal, I'll
    change this function signature to something more appropriate.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/engine.rs#L167
see above re: `display*` -- you're not displaying anything, you're rendering a
string. either make this a `to_string` method, or 
impl From<Engine> for String

    Implemented `to_string()` method. I'm currently not understanding the
    `From` trait as I can't get it to work.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/engine.rs#L246
multiple comments:
1. there's no point in a Square having coords, since that's a function of
where it is in the board array-of-arrays
2. there's no point in storing a color in a square, since that's also a
function of where it is in the board
3. without those two there's no point in a struct at all; just use
Option<Piece>
4. don't use a Piece::None variant, use an Option<Piece>
5. `[[_; 8] 8]` may be simpler to reference into, but i still recommend
`[_; 64]`.
if the "loss" of distinct x,y coordinates bothers you, you could newtype the
[_; 64] into a Board struct with get and set methods that take x,y; you
couldalso have a move method that takes 2 pairs of x,y and moves the Piece
from the first to the second, which could return a Result<Option<Piece>> where:
Err indicates the move wasn't valid (at this level that probably means simply
no piece at the starting square, or a piece of the same color was at the
second square)
Ok(None) indicates the move was successful and no piece was captured
Ok(Some(Piece)) indicates the move was successful and Piece was captured

.

    1. `Square` has been removed altogether.
    2. "
    3. "
    4. Done.
    5. I'll think about this. For now I'ma leave it. Will probably abstract
    it out as you stated.

-------------------------------------------------------------------------------

6) after removing the coord and color fields, you can quickly initialize an
empty board with [None; 64] (or [[None; 8]; 8]), then place pieces. a for loop
can more succinctly place your Pawns

.

    I took a different approach. See code. (engine.rs lines 247:289).

    However, refactoring out `Square` and just using `Option<Piece>` has made
    initialization SO much shorter. Tis a blessing in disguise.

-------------------------------------------------------------------------------

code organization: here's what i'd do:
ditch lib.rs; this is a binary, not a library (well, unless it is a library
you intend to publish separately?)
ditch modules
piece.rs: this contains Piece and Color
board.rs: this contains your Board, which for ergonomics i'd at least make a
type alias for whatever structure you go with for your board, but you could
newtype it too to add additional useful methods
player.rs: as-is (but with a singular name field)
engine.rs: this is the "root" of your actual engine. i'd add sub-modules for
preprocessors and parsers, as well as any other logic that's ultimately needed.

    lib.rs -- I plan on pushing as a bin/lib crate, so keeping lib.rs.

    ditch modules -- keeping `modules` for now.

    piece.rs -- Now contains `Piece` and `Color`, ditched `Square` altogether.
    
    board.rs -- not made yet, I'm not yet seeing a need for a `Board` struct
    or module. This may change as I may implement a `.get(file, rank)`
    and `.set(file, rank)` in the future. I'll keep this in mind.

    newtype -- still not sure what this is or what you mean here.

    player.rs -- name has been refactored.

    engine.rs -- I think I'm going to keep the structure the way it is for now
    due to the nature of not exactly knowing how I'll use this in the GUI. It
    may make sense to abstract it away from `Engine` for now... Ultimately, not
    sure as of yet. I'll put it within `engine` module if it seems it strictly
    belongs there in the future.

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/engine.rs#L272-L281
row_of_pieces is a nice abstraction, but the others are unnecessary and their
implementations unnecessarily verbose; instead:
[
    Engine::row_of_pieces(Color::White),
    [Some(Pawn(Color::White)); 8],
    [None; 8],
    /* snip */
]

    Good point. Fixed.

-------------------------------------------------------------------------------

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Piece::Pawn => write!(f, "P"),
            /* snip */
       }
   }
}

and then you either call piece.to_string(), or do something like format!("{}",
piece)

(note that to_string has the overhead of allocating a String, so you probably
want to favor the format! version instead)

you could also just take your Engine::match_for_piece method and move it to a
Piece::to_str method, though you might find that having the Display trait
implemented is just more useful anyway. then again, you could have
Piece::to_str and then use that in your Display trait implementation to get
both!

(((
    Excellent observation. I don't know why I didn't think about that.
    Display has been implemented for Piece and all tests are passing.
    Thanks dude.
)))

-------------------------------------------------------------------------------

https://github.com/jonmsawyer/chui/blob/main/src/modules/engine.rs#L143-L163
this more appropriately belongs on Piece or, better yet, as 
impl From<Piece> for &str

    Not sure what you mean here. Can Piece::from(Piece) produce a &str?
    Investigate.

    ```
    impl From<Piece> for &str {
        fn from(piece: Piece) -> &'static str {
            match piece {
                Piece::None => "·",
                Piece::Pawn(PieceColor::White) => "P",
                Piece::Rook(PieceColor::White) => "R",
                Piece::Knight(PieceColor::White) => "N",
                Piece::Bishop(PieceColor::White) => "B",
                Piece::Queen(PieceColor::White) => "Q",
                Piece::King(PieceColor::White) => "K",
                Piece::Pawn(PieceColor::Black) => "p",
                Piece::Rook(PieceColor::Black) => "r",
                Piece::Knight(PieceColor::Black) => "n",
                Piece::Bishop(PieceColor::Black) => "b",
                Piece::Queen(PieceColor::Black) => "q",
                Piece::King(PieceColor::Black) => "k",
            }
        }
    }
    ```

    When used, the compiler says that Piece::from(Piece) returns a Piece,
    not a &str... Help? Not able to get this to work...

    Besides, I'm not sure if I want Piece::from(Piece) to return &str in the
    long run.

    However, this is all taken care of becaise Display has been implemented
    for Piece.

    I do have a stub in my code for:

    impl From<&str> for Piece {
        fn from(piece: &str) -> Piece {
            match piece {
                "P" => Piece::Pawn(Color::White),
                "R" => Piece::Rook(Color::White),
                "N" => Piece::Knight(Color::White),
                "B" => Piece::Bishop(Color::White),
                "Q" => Piece::Queen(Color::White),
                "K" => Piece::King(Color::White),
                "p" => Piece::Pawn(Color::Black),
                "r" => Piece::Rook(Color::Black),
                "n" => Piece::Knight(Color::Black),
                "b" => Piece::Bishop(Color::Black),
                "q" => Piece::Queen(Color::Black),
                "k" => Piece::King(Color::Black),
                _ => Piece::None, // This is one of the reasons why I
                                  // implemented a `Piece::None` variant in
                                  // the first place. How can I exhaust this
                                  // arm without returning a Piece? I've tried
                                  // fn from(piece: &str) -> Option<Piece>
                                  // but the compiler complains.
            }
        }

        Have implemented `Piece::None` for this reason. It won't be used in
        most logic though, it's primarily for the use case of exhaustive
        pattern matching on &str.

        Not sure if this will have further implications down the road. Will
        keep an eye on it.
    }

-------------------------------------------------------------------------------

in the std library, check out std::io::Result, which is broadly considered to
be a gold standard of an ergonomic use of Results in a library. short version:
declare your own Result type which specifies that the Err variant will be your
own error type, which makes using your Result as simple as, well, `use
crate::Result`, and then your function signatures just declare they return
`Result<T>` for whatever T they need to return. your error type can include a
string message, but better yet would be to create an ErrorKind enum with
distinct variants for each type of error, and messages generated only when
needed in calling code handling them.

So to save you some Googling, here's std::io::Result
https://doc.rust-lang.org/std/io/type.Result.html

    Have implemented my own `Result` type alias. Uses `ChuiError` type which
    is an enum of the various errors that can show up during processing.
