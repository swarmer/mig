use quic::endpoint_role::EndpointRole;


#[derive(Clone, Debug, PartialEq)]
pub struct Connection {
    id: u64,
    endpoint_role: EndpointRole,
}

impl Connection {
    pub fn new(id: u64, endpoint_role: EndpointRole) -> Connection {
        Connection {
            id: id,
            endpoint_role: endpoint_role,
        }
    }
}
