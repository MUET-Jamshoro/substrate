window.SIDEBAR_ITEMS = {"enum":[["NodeKeyConfig","The configuration of a node’s secret key, describing the type of key and how it is obtained. A node’s identity keypair is the result of the evaluation of the node key configuration."],["NonReservedPeerMode","The policy for connections to non-reserved peers."],["ParseErr","Error that can be generated by `parse_str_addr`."],["Role","Role of the local node."],["Secret","The configuration options for obtaining a secret key `K`."],["SyncMode","Sync operation mode."],["TransportConfig","Configuration for the transport layer."]],"fn":[["parse_addr","Splits a Multiaddress into a Multiaddress and PeerId."],["parse_str_addr","Parses a string address and splits it into Multiaddress and PeerId, if valid."]],"macro":[["build_multiaddr","Easy way for a user to create a `Multiaddr`."]],"mod":[["ed25519","Ed25519 keys."],["identity","A node’s network identity keys."]],"struct":[["MultiaddrWithPeerId","Address of a node, including its identity."],["NetworkConfiguration","Network service configuration."],["NonDefaultSetConfig","Extension to [`SetConfig`] for sets that aren’t the default set."],["NotificationHandshake","Custom handshake for the notification protocol"],["Params","Network initialization parameters."],["ProtocolId","Protocol name prefix, transmitted on the wire for legacy protocol names. I.e., `dot` in `/dot/sync/2`. Should be unique for each chain. Always UTF-8. Deprecated in favour of genesis hash & fork ID based protocol names."],["SetConfig","Configuration for a set of nodes."]],"trait":[["ExHashT","Minimum Requirements for a Hash within Networking"],["WarpSyncProvider","Warp sync backend. Handles retrieveing and verifying warp sync proofs."]],"type":[["Ed25519Secret","The options for obtaining a Ed25519 secret key."]]};