use crate::*;

pub struct LinuxNetComponentLookup;
impl cross::NetComponentLookup for LinuxNetComponentLookup {
    fn lookup_hostname(&self) -> cross::BridgeResult<String> {
        unix::lookup_hostname()
    }
    
    fn lookup_domain(&self) -> cross::BridgeResult<cross::Capable<cross::DomainsCapable, Option<String>>> {
        //let _nss_db = NssDatabase::lookup()?;
        //TODO: check parse sssd.conf
        cross::BridgeError::err_incapable(cross::Capability::Domains)
    }

    fn lookup_domain_authorities(&self) -> cross::BridgeResult<cross::Capable<cross::DomainsCapable, Vec<cross::DomainAuthority>>> {
        cross::BridgeError::err_incapable(cross::Capability::Domains)
    }
}
