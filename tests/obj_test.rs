use polyhedron_ops::Polyhedron;

#[test]
fn tetrahedron_to_terahedron() {
    // Tetrahedron

    let mut tetrahedron = Polyhedron::tetrahedron();

    //tetrahedron.dual();
    tetrahedron.kis(Some(0.3), None, None, None, false);

    #[cfg(feature = "obj")]
    tetrahedron
        .write_obj(&std::path::PathBuf::from("."), false)
        .unwrap();
}

#[test]
fn cube_to_octahedron() {
    let mut cube = Polyhedron::hexahedron();

    cube.dual(false);
    #[cfg(feature = "obj")]
    cube.write_obj(&std::path::PathBuf::from("."), false)
        .unwrap();
}

#[test]
fn triangulate_cube() {
    let mut cube = Polyhedron::hexahedron();

    cube.triangulate(Some(true));
    #[cfg(feature = "obj")]
    cube.write_obj(&std::path::PathBuf::from("."), false)
        .unwrap();
}
