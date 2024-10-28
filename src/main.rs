use sspi::{CredentialUse, Ntlm, Sspi, Username, builders::EmptyInitializeSecurityContext, OwnedSecurityBuffer, ClientRequestFlags, DataRepresentation, SecurityBufferType, SspiImpl};

fn main() {
    let account_name = "example_user";
    let computer_name = "example_computer";
    let mut ntlm = Ntlm::new();
    let username = Username::new(&account_name, Some(&computer_name)).unwrap();
    let identity = sspi::AuthIdentity {
        username,
        password: String::from("example_password").into(),
    };

    let mut acq_cred_result = ntlm
        .acquire_credentials_handle()
        .with_credential_use(CredentialUse::Outbound)
        .with_auth_data(&identity)
        .execute(&mut ntlm)
        .unwrap();

    let mut output_buffer = vec![OwnedSecurityBuffer::new(Vec::new(), SecurityBufferType::Token)];
    // first time calling initialize_security_context, the input buffer should be empty
    let mut input_buffer = vec![OwnedSecurityBuffer::new(Vec::new(), SecurityBufferType::Token)];

    // create a builder for the first call to initialize_security_context
    // the target should start with the protocol name, e.g. "HTTP/example.com" or "LDAP/example.com"
    let mut builder = ntlm
        .initialize_security_context()
        .with_credentials_handle(&mut acq_cred_result.credentials_handle)
        .with_context_requirements(ClientRequestFlags::CONFIDENTIALITY | ClientRequestFlags::ALLOCATE_MEMORY)
        .with_target_data_representation(DataRepresentation::Native)
        .with_target_name("LDAP/example.com")
        .with_input(&mut input_buffer)
        .with_output(&mut output_buffer);

    // call initialize_security_context
    // Note: the initialize_security_context_impl returns a generator, for NTLM, 
    // this generator will never yield as NTLM requires no network communication to a third party
    // but negotiate and kerberos do require network communication, so the generator is used to
    // allow the caller to provide the network information through the generator.resume() method
    // take a look at the examples/kerberos.rs for more information
    let _result = ntlm
        .initialize_security_context_impl(&mut builder)
        .unwrap()
        .resolve_to_result()
        .unwrap();
    // ... exchange your token in output buffer with the server and repeat the process until either server is satisfied or an error is thrown
    print!("_result, {:?}", _result)
}