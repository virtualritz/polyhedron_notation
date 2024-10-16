#[allow(non_snake_case)]
mod equivalence_operator_tests {
    use polyhedron_ops::Polyhedron;

    #[test]
    fn equivalence_not() {
        let tI = Polyhedron::I().t().normalize().finalize();
        let tD = Polyhedron::D().t().normalize().finalize();

        assert_eq!(tI.positions().len(), tD.positions().len());
        assert_ne!(tI, tD);
    }

    #[test]
    fn equivalence_dd_nop() {
        let D = Polyhedron::D().normalize().finalize();
        let ddD = D.clone().d().d().normalize().finalize();

        assert_eq!(D, ddD);
    }

    #[test]
    fn equivalence_icosahedron() {
        let I = Polyhedron::I().normalize().finalize();
        let dD = Polyhedron::D().d().normalize().finalize();

        assert_eq!(I, dD);
    }
}

#[allow(non_snake_case)]
mod equivalence_archimedean_tests {
    use polyhedron_ops::Polyhedron;

    #[test]
    fn equivalence_cuboctahedron() {
        let aC = Polyhedron::C().a().normalize().finalize();
        let aO = Polyhedron::O().a().normalize().finalize();
        let eT = Polyhedron::T().e().normalize().finalize();

        assert_eq!(aC, aO);
        assert_eq!(aC, eT);
        assert_eq!(aO, eT);
    }

    #[test]
    fn equivalence_truncated_octahedron() {
        let tO = Polyhedron::O().t().normalize().finalize();
        let bT = Polyhedron::T().b().normalize().finalize();

        assert_eq!(tO, bT);
    }

    #[test]
    fn equivalence_rhombicuboctahedron() {
        let eC = Polyhedron::C().e().normalize().finalize();
        let eO = Polyhedron::O().e().normalize().finalize();

        assert_eq!(eC, eO);
    }

    #[test]
    fn equivalence_truncated_cuboctahedron() {
        let bC = Polyhedron::C().b().normalize().finalize();
        let bO = Polyhedron::O().b().normalize().finalize();

        assert_eq!(bC, bO);
    }

    #[test]
    fn equivalence_snub_cube() {
        let sC = Polyhedron::C().s().normalize().finalize();
        let sO = Polyhedron::O().s().normalize().finalize();

        // NOTE: Orientations of sC and sO are not the same
        assert_eq!(sC, sO);
    }

    #[test]
    fn equivalence_icosidodecahedron() {
        let aD = Polyhedron::D().a().normalize().finalize();
        let aI = Polyhedron::I().a().normalize().finalize();

        assert_eq!(aD, aI);
    }

    #[test]
    fn equivalence_rhombicosidodecahedron() {
        let eD = Polyhedron::D().e().normalize().finalize();
        let eI = Polyhedron::I().e().normalize().finalize();

        assert_eq!(eD, eI);
    }

    #[test]
    fn equivalence_truncated_icosidodecahedron() {
        let bD = Polyhedron::D().b().normalize().finalize();
        let bI = Polyhedron::I().b().normalize().finalize();

        assert_eq!(bD, bI);
    }

    #[test]
    fn equivalence_snub_dodecahedron() {
        let sD = Polyhedron::D().s().normalize().finalize();
        let sI = Polyhedron::I().s().normalize().finalize();

        // NOTE: Orientations of sD and sI are not the same
        assert_eq!(sD, sI);
    }
}

#[allow(non_snake_case)]
mod equivalence_catalan_tests {
    use polyhedron_ops::Polyhedron;

    #[test]
    fn equivalence_rhombic_dodecahedron() {
        let jC = Polyhedron::C().j().normalize().finalize();
        let jO = Polyhedron::O().j().normalize().finalize();
        let oT = Polyhedron::T().o().normalize().finalize();

        assert_eq!(jC, jO);
        assert_eq!(jC, oT);
        assert_eq!(jO, oT);
    }

    #[test]
    fn equivalence_tetrakis_hexahedron() {
        let k0p8C = Polyhedron::C()
            .kis(Some(0.8), None, None, None, true)
            .normalize()
            .finalize();
        let mT = Polyhedron::T().m().normalize().finalize();

        assert_eq!(k0p8C, mT);
    }

    #[test]
    fn equivalence_deltoidal_icositetrahedron() {
        let oC = Polyhedron::C().o().normalize().finalize();
        let oO = Polyhedron::O().o().normalize().finalize();

        assert_eq!(oC, oO);
    }

    #[test]
    fn equivalence_disdyakis_dodecahedron() {
        let mC = Polyhedron::C().m().normalize().finalize();
        let mO = Polyhedron::O().m().normalize().finalize();

        assert_eq!(mC, mO);
    }

    #[test]
    fn equivalence_pentagonal_icositetrahedron() {
        let gC = Polyhedron::C().g().normalize().finalize();
        let gO = Polyhedron::O().g().normalize().finalize();

        // NOTE: Orientations of gC and gO are not the same
        assert_eq!(gC, gO);
    }

    #[test]
    fn equivalence_rhombic_triacontahedron() {
        let jD = Polyhedron::D().j().normalize().finalize();
        let jI = Polyhedron::I().j().normalize().finalize();

        assert_eq!(jD, jI);
    }

    #[test]
    fn equivalence_deltoidal_hexecontahedron() {
        let oD = Polyhedron::D().o().normalize().finalize();
        let oI = Polyhedron::I().o().normalize().finalize();

        assert_eq!(oD, oI);
    }

    #[test]
    fn equivalence_disdyakis_triacontahedron() {
        let mD = Polyhedron::D().m().normalize().finalize();
        let mI = Polyhedron::I().m().normalize().finalize();

        assert_eq!(mD, mI);
    }

    #[test]
    fn equivalence_pentagonal_hexecontahedron() {
        let gD = Polyhedron::D().g().normalize().finalize();
        let gI = Polyhedron::I().g().normalize().finalize();

        // NOTE: Orientations of gD and gI are not the same
        assert_eq!(gD, gI);
    }
}
