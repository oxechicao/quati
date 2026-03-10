use quati::environments::Env;

#[test]
fn test_dotenv_loading_in_integration() {
    unsafe {
        std::env::set_var("TEST_QUATI_ENV", "integration_fixed_host");
        let test_env = Env::get_env("TEST_QUATI_ENV").unwrap();
        assert_eq!(test_env, "integration_fixed_host");
    }
}
