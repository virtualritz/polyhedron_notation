#[cfg(feature = "parser")]
mod parser_tests {
    use polyhedron_ops::Polyhedron;

    #[test]
    fn test_ambo() {
        let poly_from_str = Polyhedron::try_from("a0.2T").unwrap();
        let poly_from_ops =
            Polyhedron::tetrahedron().ambo(Some(0.2), true).finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    // All tests for the remaining operators
    #[test]
    fn test_bevel() {
        let poly_from_str = Polyhedron::try_from("b0.2,,,{t}T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .bevel(Some(0.2), None, None, Some(true), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_catmull_clark_subdivide() {
        let poly_from_str = Polyhedron::try_from("vT").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .catmull_clark_subdivide(true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_chamfer() {
        let poly_from_str = Polyhedron::try_from("c0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .chamfer(Some(0.2), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_dual() {
        let poly_from_str = Polyhedron::try_from("dT").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron().dual(true).finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_extrude() {
        let poly_from_str = Polyhedron::try_from("x0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .extrude(Some(0.2), None, None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_gyro() {
        let poly_from_str = Polyhedron::try_from("g0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .gyro(Some(0.2), None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_inset() {
        let poly_from_str = Polyhedron::try_from("i0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .inset(Some(0.2), None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_kis() {
        let poly_from_str = Polyhedron::try_from("k0.2,,,{t}T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .kis(Some(0.2), None, None, Some(true), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_medial() {
        let poly_from_str = Polyhedron::try_from("M,0.3T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .medial(None, Some(0.3), None, None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_meta() {
        let poly_from_str = Polyhedron::try_from("m0.2,,,{t}T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .meta(Some(0.2), None, None, Some(true), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_needle() {
        let poly_from_str = Polyhedron::try_from("n0.01T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .needle(Some(0.01), None, None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_ortho() {
        let poly_from_str = Polyhedron::try_from("o0.8T").unwrap();
        let poly_from_ops =
            Polyhedron::tetrahedron().ortho(Some(0.8), true).finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_propellor() {
        let poly_from_str = Polyhedron::try_from("p0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .propellor(Some(0.2), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_quinto() {
        let poly_from_str = Polyhedron::try_from("q0.2T").unwrap();
        let poly_from_ops =
            Polyhedron::tetrahedron().quinto(Some(0.2), true).finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_reflect() {
        let poly_from_str = Polyhedron::try_from("rT").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron().reflect(true).finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_spherize() {
        let poly_from_str = Polyhedron::try_from("S0.9T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .spherize(Some(0.9), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_snub() {
        let poly_from_str = Polyhedron::try_from("s,0.3T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .snub(None, Some(0.3), true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_truncate() {
        let poly_from_str = Polyhedron::try_from("t0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .truncate(Some(0.2), None, None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_whirl() {
        let poly_from_str = Polyhedron::try_from("w0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .whirl(Some(0.2), None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }

    #[test]
    fn test_zip() {
        let poly_from_str = Polyhedron::try_from("z0.2T").unwrap();
        let poly_from_ops = Polyhedron::tetrahedron()
            .zip(Some(0.2), None, None, true)
            .finalize();

        assert_eq!(poly_from_str.name(), poly_from_ops.name());
        assert_eq!(poly_from_str, poly_from_ops);
    }
}
