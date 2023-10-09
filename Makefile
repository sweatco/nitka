help: ##@Miscellaneous Show this help
	@echo "Usage: make [target] ...\n"
	@perl -e '$(HELP_FUN)' $(MAKEFILE_LIST)

build: ##@Build Build the library.
	cargo build --all

test: ##@Test Test the library.
	cargo test --all

fmt: ##@Chores Format the code using rustfmt nightly.
	cargo +nightly fmt --all

lint: ##@Chores Run lint checks with Clippy.
	./scripts/lint.sh

HELP_FUN = \
    %help; while(<>){push@{$$help{$$2//'options'}},[$$1,$$3] \
    if/^([\w-_]+)\s*:.*\#\#(?:@(\w+))?\s(.*)$$/}; \
    print"$$_:\n", map"  $$_->[0]".(" "x(20-length($$_->[0])))."$$_->[1]\n",\
    @{$$help{$$_}},"\n" for keys %help; \
