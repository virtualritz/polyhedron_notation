use polyhedron_ops::Polyhedron;

#[test]
fn make_prisms() {
    for i in 3..9 {
        let prism = Polyhedron::prism(Some(i));

        #[cfg(feature = "obj")]
        prism
            .write_obj(&std::path::PathBuf::from("."), false)
            .unwrap();

        let f = prism.faces().len();
        let v = prism.positions_len();
        let e = prism.to_edges().len();
        assert!(f == i + 2);
        assert!(v == i * 2);
        assert!(e == 2 * i + i);
        assert!(f + v - e == 2); // Euler's Formula
    }
}

#[test]
fn make_antiprisms() {
    for i in 3..9 {
        let antiprism = Polyhedron::antiprism(Some(i));

        #[cfg(feature = "obj")]
        antiprism
            .write_obj(&std::path::PathBuf::from("."), false)
            .unwrap();

        let f = antiprism.faces().len();
        let v = antiprism.positions_len();
        let e = antiprism.to_edges().len();
        assert!(f == i * 2 + 2);
        assert!(v == i * 2);
        assert!(e == 2 * i + 2 * i);
        assert!(f + v - e == 2); // Euler's Formula
    }
}
