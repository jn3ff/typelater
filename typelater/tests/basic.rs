use typelater::Typelater;

struct Cube {
    side: usize,
}

#[derive(Typelater)]
#[typelater(from = "Cube")]
struct Square {
    side: usize,
}

#[test]
fn converts_cube_into_square() {
    let cube = Cube { side: 6 };
    let square: Square = cube.into();
    assert_eq!(square.side, 6);
}
