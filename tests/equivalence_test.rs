#[allow(non_snake_case)]
mod equivalence_archimedean_tests {
    use polyhedron_ops::Polyhedron;

    #[test]
    fn equivalence_cuboctahedron() {
        let aC = Polyhedron::cube().ambo(None, true).normalize().finalize();
        let aO = Polyhedron::octahedron()
            .ambo(None, true)
            .normalize()
            .finalize();
        let eT = Polyhedron::tetrahedron()
            .expand(None, true)
            .normalize()
            .finalize();

        assert_eq!(aC, aO);
        assert_eq!(aC, eT);
        assert_eq!(aO, eT);
    }

    #[test]
    fn equivalence_truncated_octahedron() {
        let tO = Polyhedron::tetrahedron()
            .truncate(None, None, None, true)
            .normalize()
            .finalize();

        let bT = Polyhedron::tetrahedron()
            .bevel(None, None, None, None, true)
            .normalize()
            .finalize();

        assert_eq!(tO, bT);
    }
}

#[allow(non_snake_case)]

mod equivalence_operator_tests {
    use polyhedron_ops::Polyhedron;

    #[test]
    fn equivalence_dd_nop() {
        let D = Polyhedron::dodecahedron().normalize().finalize();
        let ddD = D.clone().d().d().normalize().finalize();

        assert_eq!(D, ddD);
    }

    #[test]
    fn equivalence_ztI_ttD() {
        let ztI = Polyhedron::icosahedron()
            .truncate(None, None, None, true)
            .zip(None, None, None, true)
            .finalize();
        let ttD = Polyhedron::dodecahedron()
            .truncate(None, None, None, true)
            .truncate(None, None, None, true)
            .normalize()
            .finalize();

        assert_eq!(ztI, ttD);
    }
}
