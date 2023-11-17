install-tooling:
	apt install graphviz

depgraph:
	cargo depgraph --workspace-only | dot -Tpng > depgraph.png