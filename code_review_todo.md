Code Review (thanks to http://github.com/Kromey)
================================================

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

    Will refactor crate::Result later. Unless you can give me a quick example,
    I'm unsure how to implement this.

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
