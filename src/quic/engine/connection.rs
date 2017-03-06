use quic::endpoint_role::EndpointRole;


#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    id: u64,
    endpoint_role: EndpointRole,
}
